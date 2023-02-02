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
  AWS_REGION: string
  CF_DOMAIN: string
  USER_POOL_ID: string
  USER_POOL_CLIENT_ID: string
}

const requiredEnvVars = Object.values(EnvVars)

const validNodeEnvs = Object.keys(NodeEnv)

const ENV: EnvVarsType = {
  NODE_ENV: process.env.NODE_ENV || '',
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
export function validateEnvironment(): EnvVarsType {
  // ensure all required environment variables are set
  const missing = requiredEnvVars.filter((e: EnvVars) => !ENV[e])

  if (missing.length > 0) {
    throw new Error(
      `Missing required environment variables: ${missing.join(', ')}`
    )
  }

  try {
    // ensure the CF_DOMAIN is valid
    validateURL(ENV.CF_DOMAIN)
  } catch (error) {
    throw new Error(`Invalid CF_DOMAIN: ${ENV.CF_DOMAIN}`)
  }

  return {
    NODE_ENV: ENV.NODE_ENV,
    AWS_REGION: ENV.AWS_REGION,
    CF_DOMAIN: ENV.CF_DOMAIN,
    USER_POOL_ID: ENV.USER_POOL_ID,
    USER_POOL_CLIENT_ID: ENV.USER_POOL_CLIENT_ID,
  }
}

/**
 * Validate a URL string.
 * @param {String} url a url to validate
 * @returns {URL} a URL object
 */
export function validateURL(url: string): URL {
  try {
    const trimmed = url.trim().replace(/"/, '')

    if (!trimmed || typeof trimmed !== 'string') {
      throw new Error('(must be a non-empty string)')
    }

    const protocol = trimmed.split('://')[0]
    if (!protocol || !protocol.match(/^(http|https)$/)) {
      throw new Error('(invalid protocol)')
    }

    return new URL(trimmed)
  } catch (error) {
    console.error(error)
    throw new Error(`Invalid URL ${url} ${(error as Error).message}`)
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
