import deleteToken from './deleteToken'

test('calls makes a fetch request', async () => {
  // @ts-ignore
  window.fetch.mockResolvedValueOnce({
    ok: true,
    json: async () => ({}),
  })

  await deleteToken({
    jwtToken: 'some-token',
    teamId: 'some-team',
    tokenId: 'some-token',
  })
  expect(window.fetch).toHaveBeenCalledTimes(1)
})
