import excludeIdFromArrayValues from '../excludeIdFromArrayValues'

describe('excludeIdFromArrayValues', () => {
  const input = [
    { id: '1', name: 'foo' },
    { id: '2', name: 'bar' },
  ]

  it('should exclude the id property from each item in an array', () => {
    const result = excludeIdFromArrayValues(input)

    // check that the correct output is returned
    expect(result).toEqual([{ name: 'foo' }, { name: 'bar' }])

    // check that the object keys does not contain id
    expect(
      Object.keys(result).reduce((acc, item) => acc && item !== 'id', true)
    ).toBe(true)

    // check that the input array is not mutated
    expect(input).toEqual([
      { id: '1', name: 'foo' },
      { id: '2', name: 'bar' },
    ])
  })
})
