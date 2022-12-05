/**
 * @module @cyclone-dx/ui/sbom/utils/setupTests
 */
// jest-dom adds custom jest matchers for asserting on DOM nodes.
// allows you to do things like:
// expect(element).toHaveTextContent(/react/i)
// learn more: https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom'
import path from 'path'

/**
 * Mock `process.env.CONFIG`
 * @type {string}
 * @see {@link @cyclone-dx/ui-sbom/prebuild.js}
 * @see {@link @cyclone-dx/ui-sbom/craco.config.js}
 * @see {@link @cyclone-dx/ui-sbom/utils/constants.js}
 */
const CONFIG = require(path.resolve(__dirname, '..', '..', 'prebuild'))
process.env.CONFIG = JSON.stringify(CONFIG)
