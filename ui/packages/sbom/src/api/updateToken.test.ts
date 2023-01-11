import updateToken from '@/api/updateToken'

test('calls makes a single fetch request', async () => {
  await updateToken({
    jwtToken: 'some-token',
    teamId: 'some-team',
    tokenId: 'some-token',
    token: {
      name: 'some-name',
      enabled: true,
    },
  })

  expect(global.fetch).toHaveBeenCalledTimes(1)
})
