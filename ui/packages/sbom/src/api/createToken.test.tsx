import createToken from './createToken'

test('calls makes a fetch request', () => {
  const date = new Date()
  // TODO: undo this once the API gets corrected
  const expires = date.toISOString().replace(/Z$/, '') as TDateISOWithoutZ

  createToken({
    name: 'some-name',
    expires,
    jwtToken: 'some-token',
    teamId: 'some-team',
  })

  expect(global.fetch).toHaveBeenCalledTimes(1)

  // TODO: add a test to verify the request verb and path
})
