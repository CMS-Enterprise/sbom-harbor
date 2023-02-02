import sanitizeUrl from '@/utils/sanitizeUrl'

const origin = 'test.cloudfront.net'
const route = '/api/v1/teams'
const routeWithDoubleSlashes = '//api//v1/teams'

const input = `${origin}${routeWithDoubleSlashes}`
const output = `${origin}${route}`

test('should return remove double slashes after the protocol', () => {
  const result = sanitizeUrl(`https://${input}`)
  const expected = new URL(`https://${output}`)
  expect(result).toStrictEqual(expected)
})

test('should throw an error if the protocol is not http or https', () => {
  // TODO: create a custom error class to throw and test for it here
  expect(() => sanitizeUrl('ftp://${input}')).toThrow()
})
