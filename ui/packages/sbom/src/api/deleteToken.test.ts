import deleteToken from '@/api/deleteToken'

test('calls makes a single fetch request', async () => {
  await deleteToken({
    jwtToken: 'some-token',
    teamId: 'some-team',
    tokenId: 'some-token',
  })
  expect(global.fetch).toHaveBeenCalledTimes(1)
})
