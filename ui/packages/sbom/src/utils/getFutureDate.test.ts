import getFutureDate from '@/utils/getFutureDate'

const input = '2023-01-01T00:00:00.000Z'

test('should return a timestamp formatted without the "Z"', () => {
  const result = getFutureDate(0, new Date(input))
  expect(result).toEqual('2023-01-01T00:00:00.000Z')
})

test('should add the number of days to the date', () => {
  const result = getFutureDate(1, new Date(input))
  expect(result).toEqual('2023-01-02T00:00:00.000Z')
})
