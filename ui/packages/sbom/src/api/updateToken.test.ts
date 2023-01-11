import updateToken from './updateToken'

test('calls makes a fetch request', async () => {
  // @ts-ignore
  window.fetch.mockResolvedValueOnce({
    ok: true,
    json: async () => ({}),
  })

  await updateToken({
    jwtToken: 'some-token',
    teamId: 'some-team',
    tokenId: 'some-token',
    token: {
      name: 'some-name',
      enabled: true,
    },
  })

  expect(window.fetch).toHaveBeenCalledTimes(1)
})
