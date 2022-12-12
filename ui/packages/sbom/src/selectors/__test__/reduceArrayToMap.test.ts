import reduceArrayToMap from '../reduceArrayToMap'

const input = [
  { id: '0', name: 'foo' },
  { id: '1', name: 'bar' },
]

const correctOutput = {
  [input[0].id]: { ...input[0] },
  [input[1].id]: { ...input[1] },
}

test('reduces an array of items to a map of items by id', () => {
  const result = reduceArrayToMap(input)
  expect(result).toEqual(correctOutput)
})
