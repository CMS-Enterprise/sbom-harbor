import deleteToken from './deleteToken'

test('calls makes a fetch request', () => {
  // @ts-ignore
  window.fetch.mockResolvedValueOnce({
    ok: true,
    json: async () => ({}),
  })

  deleteToken({
    jwtToken: 'some-token',
    teamId: 'some-team',
    tokenId: 'some-token',
  })
  expect(window.fetch).toHaveBeenCalledTimes(1)
})
