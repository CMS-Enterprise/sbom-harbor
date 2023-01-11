import harborRequest from '@/utils/harborRequest'
import { FormState } from '@/views/Dashboard/Team/types'

type UpdateTeamParams = {
  newTeamRouteMatch: boolean
  teamId: string
  jwtToken: string
  formInput: FormState
  abortController?: AbortController
}

const updateTeam = async ({
  newTeamRouteMatch,
  teamId,
  jwtToken,
  formInput,
  abortController = new AbortController(),
}: UpdateTeamParams) => {
  return harborRequest({
    /**
     * The path of the request.
     * @type {string}
     *
     * The path is /team if this is a new team, otherwise it is /team/:teamId.
     * This is determined by the newTeamRouteMatch boolean, which is a truthy
     * value if the route matches the new team route. Otherwise, it is null.
     *
     * @see @/views/Dashboard/Team/TeamForm.tsx for the route match.
     * @see @/router/router.tsx for the route definitions.
     * @see https://reactrouter.com/web/api/match
     */
    path: newTeamRouteMatch ? '/team' : `/team/${teamId}`,

    /**
     * The method of the request.
     * @type {string}
     *
     * The method is POST if this is a new team, otherwise it is PUT.
     * This is determined by the newTeamRouteMatch boolean, which is a truthy
     *  value if the route matches the new team route. Otherwise, it is null.
     *
     * @see @/views/Dashboard/Team/TeamForm.tsx for the route match.
     * @see @/router/router.tsx for the route definitions.
     * @see https://reactrouter.com/web/api/match
     */
    method: newTeamRouteMatch ? 'POST' : 'PUT',

    /**
     * The body of the request.
     * @type {Object} body
     * @property {string} body.name The name of the team.
     * @property {Member[]} body.members List of team members.
     * @property {Project[]} body.projects List of team projects.
     * @property {Token[]} body.tokens List of team tokens.
     */
    body: {
      /**
       * The name of the team.
       * @type {string}
       */
      name: formInput.name,

      /**
       * List of team members.
       * @type {Member[]}
       *
       * This maps the members object to an array of members.
       *
       * Any members without an email are filtered out.
       * TODO: add email validation to the form instead of filtering here.
       */
      members: Object.values(formInput.members).filter((m) => !!m.email),

      /**
       * List of team projects.
       * @type {Project[]}
       *
       * This maps the projects object to an array of projects and maps
       *  the codebases object of each project to an array of codebases.
       *
       * Any projects without a name are filtered out.
       * TODO: add project validation to the form instead of filtering here.
       */
      projects: Object.values(formInput.projects)
        .filter((p) => !!p.name)
        .map(({ codebases, ...rest }) => ({
          ...rest,
          codebases: Object.values(codebases),
        })),

      /**
       * List of team tokens.
       * @type {Token[]}
       *
       * This maps the tokens object to an array of tokens.
       * TODO: add token validation to the form.
       */
      tokens: Object.values(formInput.tokens),
    },

    /**
     * The JWT token to use for authentication.
     * @type {string}
     * @see @/router/authLoader
     */
    jwtToken,

    // pass the abort controller to the request to allow for cancelling the request.
    signal: abortController.signal,

    // include updating of children
    children: true,
  })
}

export default updateTeam
