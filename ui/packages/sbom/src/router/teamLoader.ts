/**
 * State loader for react-router data routes that require a user's single team.
 * @module @cyclonedx/ui/sbom/loaders/teamLoader
 * @see {@link @cyclonedx/ui/sbom/Routes}
 */
import { Auth } from 'aws-amplify'
import { defer, Params } from 'react-router-dom'
import { Team, TeamEntity, TeamMemberRole } from '@/types'
import reduceProjectsArrayToMap from '@/selectors/reduceProjectsArrayToMap'
import reduceArrayToMap from '@/selectors/reduceArrayToMap'
import harborRequest from '@/utils/harborRequest'

const teamLoader = ({
  params: { teamId = '' },
  request: { signal = new AbortController().signal },
}: {
  params: Params<string>
  request: Request
}) => {
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
            path: `team/${teamId}`,
            signal,
          })
      )
      .then((response: Response): Promise<TeamEntity> => response.json())
      .then(
        ({
          members,
          tokens,
          projects,
          ...rest
        }: TeamEntity): Team & {
          membersTableRows: {
            id: string
            email: string
            isTeamLead: boolean
            role: TeamMemberRole
            username: string
          }[]
        } => {
          const membersMap = reduceArrayToMap(members)
          const tokensMap = reduceArrayToMap(tokens)
          const projectsMap = reduceProjectsArrayToMap(projects)
          return {
            ...rest,
            members: membersMap,
            tokens: tokensMap,
            projects: projectsMap,
            membersTableRows: Object.values(membersMap).map(
              ({ id, email, isTeamLead }) => ({
                id,
                email,
                isTeamLead,
                role: isTeamLead
                  ? TeamMemberRole.TEAM_LEAD
                  : TeamMemberRole.MEMBER,
                username: id,
              })
            ),
          }
        }
      ),
  })
}
export default teamLoader
