import formatTimestampForServer from '@/utils/formatTimestampForServer'
import createToken from '@/api/createToken'

test('calls makes a single fetch request', async () => {
  await createToken({
    name: 'some-name',
    expires: formatTimestampForServer(1, new Date()),
    jwtToken: 'some-token',
    teamId: 'some-team',
  })

  expect(global.fetch).toHaveBeenCalledTimes(1)
})
