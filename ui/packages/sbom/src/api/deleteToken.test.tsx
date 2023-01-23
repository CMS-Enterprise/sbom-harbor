import deleteToken from './deleteToken'

test('calls makes a fetch request', () => {
  deleteToken({
    jwtToken: 'some-token',
    teamId: 'some-team',
    tokenId: 'some-token',
  })
  expect(global.fetch).toHaveBeenCalledTimes(1)
})
