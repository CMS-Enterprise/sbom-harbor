/**
 * State loader for react-router data routes that require user's all teams.
 * @module @cyclonedx/ui/sbom/loaders/teamsLoader
 * @see {@link @cyclonedx/ui/sbom/Routes}
 */
import { Auth } from 'aws-amplify'
import { defer } from 'react-router-dom'
import harborRequest from '@/utils/harborRequest'
import teamsResponseToMappedProps from '@/selectors/mapTeamsPropertiesToObjects'
import { Team, TeamEntity } from '@/types'

const teamsLoader = ({ request }: { request: Request }) => {
  return defer({
    data: Auth.currentSession()
      .then((session) => {
        const jwtToken = session.getAccessToken().getJwtToken()
        if (!jwtToken) {
          throw new Response('Invalid Session', { status: 401 })
        }
        return jwtToken
      })
      .then(
        (jwtToken: string): Promise<Response> =>
          harborRequest({
            jwtToken,
            path: `teams`,
            signal: request.signal,
          })
      )
      .then((response: Response): Promise<TeamEntity[]> => response.json())
      .then((teams: TeamEntity[]): Team[] => teamsResponseToMappedProps(teams)),
  })
}

export default teamsLoader
