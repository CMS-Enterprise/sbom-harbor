/**
 * @module @cyclone-dx/ui/sbom/utils/setupTests
 */
// jest-dom adds custom matchers for asserting on DOM nodes.
// see https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom'
import path from 'path'

/**
 * Set the NODE_ENV to test to use the test NODE_ENV.
 * This disables logging in the prebuild script:
 * @see {@link @cyclone-dx/ui-sbom/prebuild.js}.
 */
process.env.NODE_ENV = 'test'

/**
 * Mock `process.env.CONFIG` for tests
 * @type {string}
 * @see {@link @cyclone-dx/ui-sbom/prebuild.js}
 * @see {@link @cyclone-dx/ui-sbom/craco.config.js}
 * @see {@link @cyclone-dx/ui-sbom/utils/constants.js}
 */
const CONFIG = require(path.resolve(__dirname, '..', '..', 'prebuild'))
process.env.CONFIG = JSON.stringify(CONFIG)
