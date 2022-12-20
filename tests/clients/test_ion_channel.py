"""
-> Module to house Ion Channel Client tests
"""
from importlib.resources import files

import boto3
import responses
from botocore.client import BaseClient
from importlib_resources.abc import Traversable
from moto import mock_ssm

from cyclonedx.clients import IonChannelClient
from cyclonedx.clients.ion_channel.ion_channel import get_ic_urls
from cyclonedx.constants import IC_API_KEY, IC_RULESET_TEAM_ID
from tests import sboms


@mock_ssm
@responses.activate
def test_ion_channel_client_already_exists():

    """
    -> A test here for IC
    """

    sbom_folder: Traversable = files(sboms)
    sbom_obj: Traversable = sbom_folder.joinpath("panther_python.json")

    ssm: BaseClient = boto3.client("ssm")

    team_id: str = "test.team.id"
    sbom_id: str = "test.sbom.id"
    project_ids: list[str] = [
        "test.project.id.0",
        "test.project.id.1",
        "test.project.id.2",
    ]
    analysis_ids: list[str] = [
        "test.analysis.id.0",
        "test.analysis.id.1",
        "test.analysis.id.2",
    ]

    ssm.put_parameter(
        Name=IC_API_KEY,
        Description="IC_API_KEY",
        Value="not.an.actual.api.key.from.moto.mock",
        Type="String",
    )

    ssm.put_parameter(
        Name=IC_RULESET_TEAM_ID,
        Description="IC_RULESET_TEAM_ID",
        Value="not.an.actual.team.id.from.moto.mock",
        Type="String",
    )

    (
        get_own_org_url,
        get_rulesets_url,
        get_sboms_url,
        create_sbom_url,
        import_sbom_url,
        save_sbom_url,
        update_projects_url,
        get_projects_url,
        get_analysis_url,
        get_vuln_list_url,
        get_analysis_status_url,
        get_last_analysis_url,
        *nothing,
    ) = get_ic_urls()

    responses.add(
        method=responses.GET,
        url=get_own_org_url,
        status=200,
        json={
            "data": [
                {
                    "organization": {
                        "id": "test.organization.from.responses",
                    }
                }
            ]
        },
    )

    responses.add(
        method=responses.GET,
        url=get_rulesets_url,
        status=200,
        json={
            "data": [
                {
                    "id": "test.rulesets.id.from.responses",
                }
            ]
        },
    )

    responses.add(
        method=responses.GET,
        url=get_sboms_url,
        status=200,
        json={
            "data": {
                "softwareLists": [],
            }
        },
    )

    responses.add(
        method=responses.POST,
        url=create_sbom_url,
        status=200,
        json={
            "data": {
                "id": sbom_id,
                "team_id": team_id,
            }
        },
    )

    responses.add(
        method=responses.POST,
        url=import_sbom_url,
        status=200,
        json={},
    )

    responses.add(
        method=responses.POST,
        url=save_sbom_url,
        status=200,
        json={},
    )

    def proj_data(i: int):
        return {
            "active": True,
            "team_id": team_id,
            "id": project_ids[i],
            "analysis_summary": {
                "analysis_id": analysis_ids[i],
            },
        }

    responses.add(
        method=responses.GET,
        url=get_projects_url,
        status=200,
        json={
            "data": [
                proj_data(0),
                proj_data(1),
                proj_data(2),
            ]
        },
    )

    responses.add(
        method=responses.GET,
        url=get_analysis_status_url,
        status=200,
        json={
            "data": {
                "status": "finished",
            }
        },
    )

    responses.add(
        method=responses.GET,
        url=get_last_analysis_url,
        status=200,
        json={
            "data": {
                "analysis_id": "test.analysis.id.0",
            }
        },
    )

    responses.add(
        method=responses.GET,
        url=get_analysis_url,
        status=200,
        json={
            "data": {
                "analysis": {
                    "name": "test.component.name1",
                    "source": "github",
                    "passed": True,
                    "scan_summaries": [
                        {
                            "results": {
                                "type": "test.scan.type.0",
                                "data": "test.data.0",
                            }
                        },
                        {
                            "results": {
                                "type": "test.scan.type.1",
                                "data": "test.data.1",
                            }
                        },
                    ],
                },
            }
        },
    )

    responses.add(
        method=responses.GET,
        url=get_analysis_url,
        status=200,
        json={
            "data": {
                "analysis": {
                    "name": "test.component.name2",
                    "source": "github",
                    "passed": True,
                    "scan_summaries": [
                        {
                            "results": {
                                "type": "test.scan.type.2",
                                "data": "test.data.2",
                            }
                        },
                        {
                            "results": {
                                "type": "test.scan.type.3",
                                "data": "test.data.3",
                            }
                        },
                    ],
                },
            }
        },
    )

    responses.add(
        method=responses.GET,
        url=get_vuln_list_url,
        status=200,
        json={
            "data": "vulnerabilities!!",
        },
    )

    responses.add(
        method=responses.GET,
        url=update_projects_url,
        status=200,
        json={
            "data": {
                "failed": 0,
            },
        },
    )

    test_report: dict = {
        "analyses": [
            {
                "name": "test.component.name1",
                "source": "github",
                "passing": True,
                "results": [
                    ["test.scan.type.0", "test.data.0"],
                    ["test.scan.type.1", "test.data.1"],
                ],
            },
            {
                "name": "test.component.name2",
                "source": "github",
                "passing": True,
                "results": [
                    ["test.scan.type.2", "test.data.2"],
                    ["test.scan.type.3", "test.data.3"],
                ],
            },
            {
                "name": "test.component.name2",
                "source": "github",
                "passing": True,
                "results": [
                    ["test.scan.type.2", "test.data.2"],
                    ["test.scan.type.3", "test.data.3"],
                ],
            },
        ],
        "vulnerabilities": {"data": "vulnerabilities!!"},
    }

    ic_client: IonChannelClient = IonChannelClient(team_id, True)

    if not ic_client.already_exists:
        ic_client.import_sbom(sbom_obj.open())

    ic_client.monitor_sbom_analysis()

    final_report: dict = ic_client.get_report()

    assert nothing == []
    assert final_report == test_report
