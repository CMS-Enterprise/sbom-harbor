/**
 * Prebuild script that sets environment variables for the build.
 */
const CONFIG = {
  API_URL: `${process.env.CF_DOMAIN}/api`,
  CF_DOMAIN: process.env.CF_DOMAIN,
  USER_POOL_ID: process.env.USER_POOL_ID,
  USER_POOL_CLIENT_ID: process.env.USER_POOL_CLIENT_ID,
  AWS_REGION: process.env.AWS_REGION,
}

// Ouput the global app config to the shell during builds except
// for when running tests to avoid polluting the test output.
if (process.env.NODE_ENV !== 'test' || process.env.CI === 'true') {
  console.log('CONFIG:', CONFIG)
}

module.exports = CONFIG
