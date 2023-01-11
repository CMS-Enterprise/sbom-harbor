import updateToken from './updateToken'

test('calls makes a fetch request', () => {
  updateToken({
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
