/**
 * State loader for react-router data routes that require a user's single team.
 * @module @cyclonedx/ui/sbom/loaders/teamLoader
 * @see {@link @cyclonedx/ui/sbom/Routes}
 */
import { Params } from 'react-router-dom'
import { Team, TeamEntity } from '@/types'
import authLoader from '@/router/authLoader'
import harborRequest from '@/utils/harborRequest'
import reduceProjectsArrayToMap from '@/selectors/reduceProjectsArrayToMap'
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
      (jwtToken: string): Promise<TeamEntity> =>
        harborRequest({
          jwtToken,
          path: `team/${teamId}`,
          signal,
        })
    )
    .then(
      ({ members, tokens, projects, ...rest }: TeamEntity): Team => ({
        ...rest,
        members: reduceArrayToMap(members),
        tokens: reduceArrayToMap(tokens),
        projects: reduceProjectsArrayToMap(projects),
      })
    )

export default teamLoader
