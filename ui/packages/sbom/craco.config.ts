/**
 * Craco config that overrides the default webpack config.
 */
// mock environment variables for the global app config during tests
if (process.env.NODE_ENV === 'test') {
  process.env.ENVIRONMENT = 'test'
  process.env.AWS_REGION = 'us-east-1'
  process.env.CF_DOMAIN = 'localhost:3000'
  process.env.API_URL = `http://${process.env.CF_DOMAIN}`
  process.env.USER_POOL_ID = 'us-east-1_123456789'
  process.env.USER_POOL_CLIENT_ID = '1234567890123456789012'
}

import { resolve } from 'path'
import { EnvironmentPlugin } from 'webpack'
import { validateEnvironment } from './src/utils/validateEnvironment'

// validate environment variables
const ENV = validateEnvironment()

// Output the global app config to the shell during builds
// except when running tests to avoid polluting the test output.
if (ENV.NODE_ENV !== 'test') {
  console.log('CONFIG:', JSON.stringify(ENV, null, 2))
}

export default {
  webpack: {
    // webpack aliases should match jest module name mappings below
    alias: {
      '@': resolve(__dirname, 'src'),
    },
    plugins: [
      // the environment plugin is used to set environment vars
      // that are read by the client app. see usage in index.tsx.
      // see: https://webpack-v3.jsx.app/plugins/environment-plugin/
      new EnvironmentPlugin({
        ...ENV,
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
