import updateTeam from '@/api/updateTeam'
import { BuildTool, CodebaseLanguage } from '@/types'
import formatTimestampForServer from '@/utils/formatTimestampForServer'

const apiUrl = 'https://localhost:3000/api/v1/team'

const member = {
  isTeamLead: true,
  id: 'member-id-1',
  email: 'some-email',
}

const codebase = {
  id: 'codebase-id-1',
  name: 'some-codebase',
  language: CodebaseLanguage.JAVASCRIPT,
  buildTool: BuildTool.NPM,
}

const project = {
  id: 'project-id-1',
  name: 'some-project',
  fisma: 'some-fisma',
  codebases: { 'codebase-id-1': codebase },
}

const token = {
  id: 'token-id-1',
  name: 'some-token',
  enabled: true,
  token: 'sbom-api-abcdefg',
  created: formatTimestampForServer(0, new Date()),
  expires: formatTimestampForServer(1, new Date()),
}

const teamId = 'abcd-0123'

const team = {
  name: 'some-name',
  members: { 'member-id-1': member },
  projects: { 'project-id-1': project },
  tokens: { 'token-id-1': token },
}

const bodyData = {
  name: 'some-name',
  members: [member],
  projects: [{ ...project, codebases: [codebase] }],
  tokens: [token],
}

const updateTeamParams = {
  formInput: team,
  jwtToken: 'some-token',
  newTeamRouteMatch: true,
  teamId,
}

test('calls makes a fetch request once', async () => {
  await updateTeam(updateTeamParams)
  expect(global.fetch).toHaveBeenCalledTimes(1)
})

test('calls makes a fetch request with correct body', async () => {
  await updateTeam(updateTeamParams)
  const [[, { body }]] = (global.fetch as jest.Mock).mock.calls
  expect(JSON.parse(body)).toMatchObject(bodyData)
})

test('creates a new team on the update team route', async () => {
  await updateTeam(updateTeamParams)
  const [[requestUrl]] = (global.fetch as jest.Mock).mock.calls
  const desiredUrl = `${apiUrl}?children=true`
  expect(requestUrl.toString()).toStrictEqual(desiredUrl)
})

test('updates an existing team on the update team route', async () => {
  await updateTeam({ ...updateTeamParams, newTeamRouteMatch: false })
  const [[requestUrl]] = (global.fetch as jest.Mock).mock.calls
  const desiredUrl = `${apiUrl}/${teamId}?children=true`
  expect(requestUrl.toString()).toStrictEqual(desiredUrl)
})
