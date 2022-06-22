const { DefinePlugin } = require('webpack')
const path = require('path')

const CONFIG = require(path.resolve(__dirname, './prebuild.js'))

module.exports = {
  webpack: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
      '@/assets/icons': path.resolve(__dirname, 'src/assets/icons'),
    },
    plugins: [
      new DefinePlugin({
        'process.env.CONFIG': JSON.stringify(CONFIG),
      }),
    ],
  },
  jest: {
    configure: {
      moduleNameMapper: {
        '^@/(.*)$': '<rootDir>/src/$1',
      },
    },
  },
  eslint: {
    enable: false,
  },
}
