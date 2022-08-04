from cyclonedx.core_utils.cyclonedx_util import (
    __get_team_by_team_id,
    __create_team_response
)


def get_team_handler(event: dict = None, context: dict = None):

    team_id = event["pathParameters"]["team"]
    team = __get_team_by_team_id(team_id)

    return __create_team_response(
        status_code=200,
        msg=team
    )
