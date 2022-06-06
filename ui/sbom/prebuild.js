const path = require('path')
const { execSync } = require('child_process')

process.env.CF_DOMAIN = execSync(
  `bash ${path.resolve(__dirname, '../../bin/get-cf-domain.sh')}`
)
  .toString()
  .trim()

process.env.USER_POOL_ID = execSync(
  `bash ${path.resolve(__dirname, '../../bin/get-user-pool-id.sh')}`
)
  .toString()
  .trim()

process.env.USER_POOL_CLIENT_ID = execSync(
  `bash ${path.resolve(__dirname, '../../bin/get-user-pool-client-id.sh')}`
)
  .toString()
  .trim()

const CONFIG = {
  CF_DOMAIN: process.env.CF_DOMAIN,
  USER_POOL_ID: process.env.USER_POOL_ID,
  USER_POOL_CLIENT_ID: process.env.USER_POOL_CLIENT_ID,
}

console.log('CONFIG:\n')
console.log(CONFIG)

module.exports = CONFIG
