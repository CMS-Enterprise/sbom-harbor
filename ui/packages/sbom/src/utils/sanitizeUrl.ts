/**
 * Sanitizes a URL by removing any double slashes.
 * @module @cyclonedx/ui/sbom/utils/sanitizeUrl
 *
 * @param {string} urlString The URL to sanitize as a string.
 * @returns {URL} The sanitized URL.
 */
const sanitizeUrl = (urlString: string): URL =>
  new URL(urlString.replace(/\/\//g, '/'))

export default sanitizeUrl
