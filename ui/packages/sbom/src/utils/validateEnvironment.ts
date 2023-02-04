enum NodeEnv {
  PRODUCTION = 'production',
  TEST = 'test',
  DEVELOPMENT = 'development',
}

enum EnvVars {
  NODE_ENV = 'NODE_ENV',
  AWS_REGION = 'AWS_REGION',
  CF_DOMAIN = 'CF_DOMAIN',
  USER_POOL_ID = 'USER_POOL_ID',
  USER_POOL_CLIENT_ID = 'USER_POOL_CLIENT_ID',
}

type EnvVarsType = {
  NODE_ENV: string
  ENVIRONMENT: string
  AWS_REGION: string
  CF_DOMAIN: string
  USER_POOL_ID: string
  USER_POOL_CLIENT_ID: string
}

const requiredEnvVars = Object.values(EnvVars)

const validNodeEnvs = Object.keys(NodeEnv)

const ENV: EnvVarsType = {
  ENVIRONMENT: process.env.ENVIRONMENT || '',
  NODE_ENV: process.env.NODE_ENV || 'development',
  AWS_REGION: process.env.AWS_REGION || '',
  CF_DOMAIN: process.env.CF_DOMAIN || '',
  USER_POOL_ID: process.env.USER_POOL_ID || '',
  USER_POOL_CLIENT_ID: process.env.USER_POOL_CLIENT_ID || '',
}

/**
 * Validate the required environment variables.
 * @returns {EnvVarsType} an object containing each
 *  the required environment variables and their values.
 */
export function validateEnvironment(): EnvVarsType & { API_URL: string } {
  // ensure all required environment variables are set
  const missing = requiredEnvVars.filter((e: EnvVars) => !ENV[e])
  if (missing.length > 0) {
    throw new Error(
      `Missing required environment variables: ${missing.join(', ')}`
    )
  }
  return {
    NODE_ENV: ENV.NODE_ENV,
    ENVIRONMENT: ENV.ENVIRONMENT,
    AWS_REGION: ENV.AWS_REGION,
    USER_POOL_ID: ENV.USER_POOL_ID,
    USER_POOL_CLIENT_ID: ENV.USER_POOL_CLIENT_ID,
    CF_DOMAIN: ENV.CF_DOMAIN,
    API_URL: getApiUrl(ENV.CF_DOMAIN).toString(),
  }
}

/**
 * Ensure a cloudfront domain has a protocol.
 * @param {string} cfDomain the cloudfront domain
 * @returns the cloudfront domain with a protocol
 */
export function addProtocolToCloudfrontDomain(cfDomain: string): string {
  const protocol = cfDomain.split('://')[0]
  if (protocol && protocol.match(/^(http|https)$/)) {
    return cfDomain
  }
  if (ENV.NODE_ENV === 'test') {
    return `http://${cfDomain}`
  }
  return `https://${cfDomain}`
}

/**
 * Validate a URL string.
 * @param {String} url a url to validate
 * @returns {URL} a URL object
 */
export function getApiUrl(cfDomain: string): URL {
  try {
    const trimmed = cfDomain.trim().replace(/"/, '')
    if (!trimmed || typeof trimmed !== 'string') {
      throw new Error('(must be a non-empty string)')
    }
    return new URL(addProtocolToCloudfrontDomain(`${trimmed}/api`))
  } catch (error) {
    console.error(error)
    throw new Error(`Invalid URL ${cfDomain} ${(error as Error).message}`)
  }
}

/**
 * Validate the NODE_ENV environment variable
 */
export function validateNodeEnv() {
  if (!validNodeEnvs.includes(ENV.NODE_ENV)) {
    const options = validNodeEnvs.join(', ')
    throw new Error(
      `Invalid NODE_ENV: ${ENV.NODE_ENV} (must be one of: ${options})`
    )
  }
}
