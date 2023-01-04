import formatTimestampForServer from '@/utils/formatTimestampForServer'

const input = '2023-01-01T00:00:00.000Z'

test('should return a timestamp formatted without the "Z"', () => {
  const result = formatTimestampForServer(0, new Date(input))
  expect(result).toEqual('2023-01-01T00:00:00.000')
})

test('should add the number of days to the date', () => {
  const result = formatTimestampForServer(1, new Date(input))
  expect(result).toEqual('2023-01-02T00:00:00.000')
})
