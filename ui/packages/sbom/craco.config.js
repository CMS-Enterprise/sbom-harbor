/**
 * Craco config that overrides the default webpack config.
 */
const { EnvironmentPlugin } = require('webpack')
const path = require('path')

// mock environment variables for the global app config during tests
if (process.env.NODE_ENV === 'test') {
  process.env.CF_DOMAIN = 'localhost:3000'
  process.env.USER_POOL_ID = 'us-east-1_123456789'
  process.env.USER_POOL_CLIENT_ID = '1234567890123456789012'
}

// set the global app config from environment variables
const CONFIG = {
  API_URL: `https://${process.env.CF_DOMAIN}/api`,
  CF_DOMAIN: process.env.CF_DOMAIN,
  USER_POOL_ID: process.env.USER_POOL_ID,
  USER_POOL_CLIENT_ID: process.env.USER_POOL_CLIENT_ID,
  AWS_REGION: process.env.AWS_REGION,
}

// Output the global app config to the shell during builds
// except when running tests to avoid polluting the test output.
console.log('CONFIG:', CONFIG)

module.exports = {
  webpack: {
    // webpack aliases should match jest module name mappings below
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
    plugins: [
      // the environment plugin is used to set environment vars
      // that are read by the client app. see usage in index.tsx.
      // see: https://webpack-v3.jsx.app/plugins/environment-plugin/
      new EnvironmentPlugin({
        API_URL: `https://${process.env.CF_DOMAIN}/api`,
        CF_DOMAIN: process.env.CF_DOMAIN,
        USER_POOL_ID: process.env.USER_POOL_ID,
        USER_POOL_CLIENT_ID: process.env.USER_POOL_CLIENT_ID,
        AWS_REGION: process.env.AWS_REGION,
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
