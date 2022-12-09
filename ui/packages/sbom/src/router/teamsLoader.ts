/**
 * State loader for react-router data routes that require user's all teams.
 * @module @cyclonedx/ui/sbom/loaders/teamsLoader
 * @see {@link @cyclonedx/ui/sbom/Routes}
 */
import authLoader from '@/router/authLoader'
import harborRequest from '@/utils/harborRequest'
import teamsResponseToMappedProps from '@/selectors/mapTeamsPropertiesToObjects'
import { Team, TeamModel } from '@/types'

const teamsLoader = ({
  request: { signal = new AbortController().signal },
}: {
  request: Request
}): Promise<Team[]> =>
  authLoader()
    .then(
      (jwtToken: string): Promise<TeamModel[]> =>
        harborRequest({
          jwtToken,
          path: `teams`,
          signal,
        })
    )
    .then((teams: TeamModel[]): Team[] => teamsResponseToMappedProps(teams))

export default teamsLoader
