/**
 * @module @cyclone-dx/ui/sbom/utils/setupTests
 */
// jest-dom adds custom jest matchers for asserting on DOM nodes.
// allows you to do things like:
// expect(element).toHaveTextContent(/react/i)
// learn more: https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom'
import '@testing-library/jest-dom/extend-expect'

/**
 * Set the NODE_ENV to test to use the test environment configuration.
 * This disables logging in {@link @cyclone-dx/ui-sbom/prebuild.js}.
 */
process.env.NODE_ENV = 'test'

/**
 * Set the CONFIG environment variable to the prebuild configuration.
 */
process.env.CONFIG = JSON.stringify({
  API_URL: 'https://localhost:3000/api',
  COGNITO_USER_POOL_ID: 'us-east-1_123456789',
  COGNITO_USER_POOL_CLIENT_ID: '1234567890123456789012',
})

/**
 * Create a mock fetch response per endpoint.
 * @param {URL | string} url the fetch request url
 * @param {Object} config the fetch request configuration
 * @returns  {Promise<Object>} the mocked fetch response per endpoint
 */
async function mockFetch(url, config) {
  // sbom upload endpoint
  if (url.includes('/sbom')) {
    return {
      json: async () => ({}),
      ok: true,
      status: 200,
      statusText: 'OK',
    }
  }
}

beforeAll(() => {
  jest.spyOn(global, 'fetch')
})

beforeEach(() => {
  fetch.mockImplementation(mockFetch)
})
