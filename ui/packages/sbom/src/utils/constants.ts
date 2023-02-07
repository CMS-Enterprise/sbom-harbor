/**
 * @module @cyclonedx/ui/sbom/utils/constants
 * @exports CONFIG
 * @exports storageTokenKeyName
 */
import { AppConfig } from '@/types'

// parse `CONFIG` from environment variables
const apiUrl = `https://${process.env.CF_DOMAIN}/api`

/**
 * Set global configuration for the application provided by webpack (craco) at build time.
 * @see {@link @cyclonedx-python/ui/packages/sbom/craco.config.js}.
 */
export const CONFIG = {
  CF_DOMAIN: process.env.CF_DOMAIN,
  API_URL: apiUrl,
  TEAM_API_URL: `${apiUrl}/v1/team`,
  TEAMS_API_URL: `${apiUrl}/v1/teams`,
  USER_API_URL: `${apiUrl}/v1/user`,
  USER_API_SEARCH_URL: `${apiUrl}/v1/user/search`,
  USER_POOL_ID: process.env.USER_POOL_ID,
  USER_POOL_CLIENT_ID: process.env.USER_POOL_CLIENT_ID,
  AWS_REGION: process.env.AWS_REGION,
} as AppConfig
