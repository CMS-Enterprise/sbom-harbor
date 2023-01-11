import formatTimestampForServer from '@/utils/formatTimestampForServer'
import createToken from './createToken'

test('calls makes a fetch request', async () => {
  // @ts-ignore
  window.fetch.mockResolvedValueOnce({
    ok: true,
    json: async () => ({}),
  })

  await createToken({
    name: 'some-name',
    expires: formatTimestampForServer(1, new Date()),
    jwtToken: 'some-token',
    teamId: 'some-team',
  })

  expect(window.fetch).toHaveBeenCalledTimes(1)
})
