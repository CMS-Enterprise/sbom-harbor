import harborRequest from '@/utils/harborRequest'
import { FormState } from '@/views/Dashboard/Team/types'
import { User } from '@/types'

type UpdateTeamParams = {
  abortController?: AbortController
  admins: User[]
  formInput: FormState
  jwtToken: string
  members: User[]
  newTeamRouteMatch: boolean
  teamId: string
}

const updateTeam = async ({
  abortController = new AbortController(),
  admins,
  formInput,
  jwtToken,
  members,
  newTeamRouteMatch,
  teamId,
}: UpdateTeamParams) => {
  // filter out projects with empty name values from the projects and
  // then map the codebases of each project to an array of codebases.
  // TODO: add project schema validation to the form to prevent this from happening
  const projectValues = Object.values(formInput.projects)
    .filter((p) => !!p.name)
    .map(({ codebases, ...rest }) => ({
      ...rest,
      codebases: Object.values(codebases),
    }))

  // filter out any empty email values from the projects object
  // TODO: add emailvalidation to the form to prevent this from happening
  const membersValues = [...admins, ...members].filter((m) => !!m.email)

  // make request to update teams data in the database.
  return harborRequest({
    // determine the endpoint to use based on if this is a create or edit form.
    path: newTeamRouteMatch ? '/teams' : `/team/${teamId}`,
    // determine the request verb based on if this is a create or edit form.
    method: newTeamRouteMatch ? 'POST' : 'PUT',
    // pass the jwt token from the authLoader to the request to authenticate the user.
    jwtToken,
    // add the final object representing the team data to send to the server.
    body: {
      name: formInput.name,
      members: membersValues,
      projects: projectValues,
      tokens: Object.values(formInput.tokens),
    },
    // pass the abort controller to the request to allow for cancelling the request.
    signal: abortController.signal,
  })
}

export default updateTeam
