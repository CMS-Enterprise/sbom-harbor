import sanitizeUrl from '@/utils/sanitizeUrl'

const input = 'https://test.cloudfront.net//'
const output = 'https://test.cloudfront.net/'

test('should return remove double trailing slashes', () => {
  expect(sanitizeUrl(input)).toStrictEqual(new URL(output))
})
