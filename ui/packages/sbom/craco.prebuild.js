/**
 * This file is executed before the CRA (craco.config.js) build process.
 * It is used to set the global app config from environment variables.
 * The config is then passed to the app via the webpack EnvironmentPlugin.
 */

// mock environment variables for the global app config during tests
if (process.env.NODE_ENV === 'test') {
  process.env.CF_DOMAIN = 'localhost:3000'
  process.env.USER_POOL_ID = 'us-east-1_123456789'
  process.env.USER_POOL_CLIENT_ID = '1234567890123456789012'
}

// set the global app config from environment variables
const CONFIG = {
  NODE_ENV: process.env.NODE_ENV,
  ENVIRONMENT: process.env.ENVIRONMENT,
  CF_DOMAIN: process.env.CF_DOMAIN,
  API_URL: `https://${process.env.CF_DOMAIN}/api`,
  USER_POOL_ID: process.env.USER_POOL_ID,
  USER_POOL_CLIENT_ID: process.env.USER_POOL_CLIENT_ID,
  AWS_REGION: process.env.AWS_REGION,
}

// Output the global app config to the shell during builds
console.log('UI CONFIG:', CONFIG, '\n')

// export the global app config for use by the webpack EnvironmentPlugin
module.exports = CONFIG
