/**
 * Craco config that overrides the default webpack config.
 */
const { resolve } = require('path')
const { EnvironmentPlugin } = require('webpack')
const CONFIG = require('./craco.prebuild')

module.exports = {
  webpack: {
    // webpack aliases should match jest module name mappings below
    alias: {
      '@': resolve(__dirname, 'src'),
    },
    plugins: [
      // the environment plugin is used to set environment vars
      // that are read by the client app. See craco.prebuild.js.
      // see: https://webpack-v3.jsx.app/plugins/environment-plugin/
      new EnvironmentPlugin({
        ...CONFIG,
      }),
    ],
  },
  jest: {
    configure: {
      // jest module name mappings should match the webpack aliases above
      moduleNameMapper: { '^@/(.*)$': '<rootDir>/src/$1' },
      roots: ['<rootDir>/src/'],
      testMatch: ['<rootDir>/src/**/?(*.)+(spec|test).[jt]s?(x)'],
      setupFilesAfterEnv: '<rootDir>/jest.setup.ts',
      collectCoverage: true,
      collectCoverageFrom: ['<rootDir>/**/*.{js,jsx,ts,tsx}'],
      coverageReporters: ['json', 'lcov', 'text'],
    },
  },
  eslint: {
    enable: false,
  },
}
