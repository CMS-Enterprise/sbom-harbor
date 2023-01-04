import createToken from './createToken'

test('calls makes a fetch request', () => {
  // @ts-ignore
  window.fetch.mockResolvedValueOnce({
    ok: true,
    json: async () => ({}),
  })

  const date = new Date()
  // TODO: undo this once the API gets corrected
  const expires = date.toISOString().replace(/Z$/, '') as TDateISOWithoutZ

  createToken({
    name: 'some-name',
    expires,
    jwtToken: 'some-token',
    teamId: 'some-team',
  })

  expect(window.fetch).toHaveBeenCalledTimes(1)

  // TODO: add a test to verify the request verb and path
})
