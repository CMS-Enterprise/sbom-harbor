/**
 * State loader for react-router data routes that require a user's single team.
 * @module @cyclonedx/ui/sbom/loaders/teamLoader
 * @see {@link @cyclonedx/ui/sbom/Routes}
 */
import { Params } from 'react-router-dom'
import { Team, TeamApiResponse } from '@/types'
import authLoader from '@/router/authLoader'
import harborRequest from '@/utils/harborRequest'
import projectsArrayToEntries from '@/selectors/projectsArrayToEntries'
import reduceArrayToMap from '@/selectors/reduceArrayToMap'

const teamLoader = ({
  params: { teamId = '' },
  request: { signal = new AbortController().signal },
}: {
  params: Params<string>
  request: Request
}): Promise<Team> =>
  authLoader()
    .then(
      (jwtToken: string): Promise<TeamApiResponse> =>
        harborRequest({
          jwtToken,
          path: `team/${teamId}`,
          signal,
        })
    )
    .then(
      ({ members, tokens, projects, ...rest }: TeamApiResponse): Team => ({
        ...rest,
        members: members,
        tokens: reduceArrayToMap(tokens),
        projects: projectsArrayToEntries(projects),
      })
    )

export default teamLoader
