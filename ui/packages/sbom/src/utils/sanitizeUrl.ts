/**
 * Sanitizes a URL by removing any double slashes.
 * @module @cyclonedx/ui/sbom/utils/sanitizeUrl
 *
 * @param {string} url The URL to sanitize as a string.
 * @returns {URL} The sanitized URL.
 */
const sanitizeUrl = (url: string): URL => {
  // split the url into the protocol and the rest of the url
  const [protocol, rest] = url.split('://')

  // validate that the protocol is either http or https
  if (protocol !== 'https' && protocol !== 'http') {
    throw new Error(`Invalid URL: ${url} (invalid/missing protocol)`)
  }

  // replace double slashes after the protocol with single slashes
  const sanitized = rest.replace(/\/\//g, '/')

  // return the sanitized URL with the protocol
  return new URL(`${protocol}://${sanitized}`)
}

export default sanitizeUrl
