/**
 * @module @cyclone-dx/ui/sbom/utils/setupTests
 */
import '@testing-library/jest-dom'
import '@testing-library/jest-dom/extend-expect'

/**
 * Create a mock fetch response per endpoint.
 * @param {URL | string} url - The fetch request url
 * @param {Object} config - The fetch request configuration
 * @returns  {Promise<Object>} - The mocked fetch response per endpoint
 */
async function mockFetch(url, config) {
  return Promise.resolve({
    json: async () => ({}),
    ok: true,
    status: 200,
    statusText: 'OK',
  })
}

beforeEach(() => {
  /**
   * Mock `window.fetch` for tests
   * @see {@link https://jestjs.io/docs/en/manual-mocks}
   */
  // assuming jest's resetMocks is configured to "true"
  // so we don't need to worry about cleanup. also assumes
  // that a fetch polyfill like `whatwg-fetch` is loaded.
  jest.spyOn(global, 'fetch').mockImplementation(mockFetch as jest.Mock)
})