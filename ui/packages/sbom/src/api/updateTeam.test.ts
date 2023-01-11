import updateTeam from '@/api/updateTeam'
import { BuildTool, CodebaseLanguage } from '@/types'
import formatTimestampForServer from '@/utils/formatTimestampForServer'

const port = window.location.port || 3000
const host = window.location.hostname
const requestUrl = `https://${host}:${port}/api/v1/team`

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

const token = {
  id: 'token-id-1',
  name: 'some-token',
  enabled: true,
  token: 'sbom-api-abcdefg',
  created: formatTimestampForServer(0, new Date()),
  expires: formatTimestampForServer(1, new Date()),
}

const project = {
  id: 'project-id-1',
  name: 'some-project',
  fisma: 'some-fisma',
  codebases: { 'codebase-id-1': codebase },
}

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
  teamId: 'team-id',
}

const fetchMockResolvedValue = { ok: true, json: async () => ({}) }

test('calls makes a fetch request once with correct body', async () => {
  // @ts-ignore
  window.fetch.mockResolvedValueOnce(fetchMockResolvedValue)
  await updateTeam({ ...updateTeamParams, newTeamRouteMatch: false })

  expect(window.fetch).toHaveBeenCalledTimes(1)

  // @ts-ignore
  const [[, { body }]] = window.fetch.mock.calls

  expect(JSON.parse(body)).toMatchObject(bodyData)
})

test('creates a new team on the update team route', async () => {
  // @ts-ignore
  window.fetch.mockResolvedValueOnce(fetchMockResolvedValue)
  await updateTeam({ ...updateTeamParams, newTeamRouteMatch: true })

  // @ts-ignore
  const [[url]] = window.fetch.mock.calls
  expect(url.toString()).toStrictEqual(`${requestUrl}?children=true`)
})

test('updates an existing team on the update team route', async () => {
  // @ts-ignore
  window.fetch.mockResolvedValueOnce(fetchMockResolvedValue)
  await updateTeam({ ...updateTeamParams, newTeamRouteMatch: false })

  // @ts-ignore
  const [[url]] = window.fetch.mock.calls
  expect(url.toString()).toStrictEqual(`${requestUrl}/team-id?children=true`)
})
