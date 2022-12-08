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
process.env.CONFIG = JSON.stringify(require('./prebuild.js'))

/**
 * Mock `window.fetch` for tests
 * @see {@link https://jestjs.io/docs/en/manual-mocks}
 */
// assuming jest's resetMocks is configured to "true"
// so we don't need to worry about cleanup. also assumes
// that a fetch polyfill like `whatwg-fetch` is loaded.
jest.spyOn(window, 'fetch')
