import sanitizeUrl from '@/utils/sanitizeUrl'

const input = 'https://test.cloudfront.net//'
const output = 'https://test.cloudfront.net/'

test('should return remove double trailing slashes', () => {
  const result = sanitizeUrl(input)
  expect(result).toEqual(output)
})
