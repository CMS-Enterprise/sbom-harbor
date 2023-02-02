import updateToken from '@/api/updateToken'
import getFutureDate from '@/utils/getFutureDate'

test('calls makes a single fetch request', async () => {
  await updateToken({
    jwtToken: 'some-token',
    teamId: 'some-team',
    tokenId: 'some-token',
    token: {
      name: 'some-name',
      enabled: true,
      expires: getFutureDate(1, new Date()),
    },
  })

  expect(global.fetch).toHaveBeenCalledTimes(1)
})
