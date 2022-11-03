/**
 * Prebuild script that sets environment variables for the build.
 */
const path = require('path')
const { execSync } = require('child_process')

process.env.CF_DOMAIN =
  process.env.CF_DOMAIN ||
  execSync(`bash ${path.resolve(__dirname, './bin/get-cf-domain.sh')}`)
    .toString()
    .trim()

process.env.API_URL = `${process.env.CF_DOMAIN}/api`

process.env.USER_POOL_ID =
  process.env.USER_POOL_ID ||
  execSync(`bash ${path.resolve(__dirname, './bin/get-user-pool-id.sh')}`)
    .toString()
    .trim()

process.env.USER_POOL_CLIENT_ID =
  process.env.USER_POOL_CLIENT_ID ||
  execSync(
    `bash ${path.resolve(__dirname, './bin/get-user-pool-client-id.sh')}`
  )
    .toString()
    .trim()

process.env.AWS_REGION = process.env.AWS_REGION || 'us-east-1'

const CONFIG = {
  API_URL: process.env.API_URL,
  CF_DOMAIN: process.env.CF_DOMAIN,
  USER_POOL_ID: process.env.USER_POOL_ID,
  USER_POOL_CLIENT_ID: process.env.USER_POOL_CLIENT_ID,
  AWS_REGION: process.env.AWS_REGION,
}

console.log('CONFIG:', CONFIG)

module.exports = CONFIG
