"""
-> End-to-End Test for the Members
"""
from json import dumps
from time import sleep

from requests import Response, put

from cyclonedx.model.member import Member
from tests.e2e import (
    cleanup,
    create_team_with_projects,
    get_cloudfront_url,
    get_entity_by_id,
    get_team_url,
    login,
    print_response,
)


def test_update_member():

    """
    -> Test updating a member
    """

    cf_url: str = get_cloudfront_url()
    jwt: str = login(cf_url)

    # Create a team with 2 projects
    team_name: str = "1Team1"
    proj1_name: str = "1Project1"
    proj2_name: str = "2Project2"

    team_url: str = get_team_url(cf_url)
    create_rsp: dict = create_team_with_projects(
        team_name=team_name,
        project_names=[proj1_name, proj2_name],
        team_url=team_url,
        jwt=jwt,
    )

    team_id: str = create_rsp.get("id")
    members: list[dict] = create_rsp.get("members")
    init_member: dict = members[0]

    member_id: str = init_member.get(Member.Fields.ID)
    old_email: str = init_member.get(Member.Fields.EMAIL)
    new_email: str = "billybob@gmail.com"

    assert old_email != new_email

    init_member[Member.Fields.EMAIL] = new_email

    member_url: str = f"{cf_url}/api/v1/member/{member_id}?teamId={team_id}"
    print(f"Sending To: PUT:{member_url}")
    put_rsp: Response = put(
        member_url,
        headers={
            "Authorization": jwt,
        },
        json=init_member,
    )
    print_response(put_rsp)

    # There needs to be a sleep here because DynamoDB does not
    # update fast enough to get the new data if it's not.
    sleep(10)

    get_project_rsp: dict = get_entity_by_id(
        team_id=team_id,
        entity_key="member",
        entity_id=member_id,
        cf_url=cf_url,
        jwt=jwt,
    )

    assert get_project_rsp.get(Member.Fields.EMAIL) == new_email

    print(dumps(create_rsp, indent=2))
    cleanup(
        team_id=team_id,
        team_url=team_url,
        jwt=jwt,
    )
