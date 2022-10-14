/**
 * Craco config that overrides the default webpack config.
 */
const path = require('path')
const { DefinePlugin } = require('webpack')

// get the app config by importing the prebuild script
const CONFIG = require(path.resolve(__dirname, './prebuild.js'))

module.exports = {
  webpack: {
    // webpack aliases should match jest module name mappings below
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
    plugins: [
      // the define plugin is used to set environment variables
      // that are read by the client app. see usage in index.tsx.
      // see: https://webpack.js.org/plugins/define-plugin/
      new DefinePlugin({
        'process.env.CONFIG': JSON.stringify(CONFIG),
      }),
    ],
  },
  jest: {
    configure: {
      // jest module name mappings should match the webpack aliases above
      moduleNameMapper: {
        '^@/(.*)$': '<rootDir>/src/$1',
      },
      roots: ['<rootDir>/src/'],
      testMatch: ['<rootDir>/src/**/?(*.)+(spec|test).[jt]s?(x)'],
      setupFilesAfterEnv: '<rootDir>/src/utils/setupTests.ts',
    },
  },
  eslint: {
    enable: false,
  },
}
