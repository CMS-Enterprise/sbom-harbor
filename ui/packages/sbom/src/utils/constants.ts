/**
 * @module @cyclonedx/ui/sbom/utils/constants
 * @exports CONFIG
 * @exports storageTokenKeyName
 */
import { AppConfig } from '@/types'
import { validateEnvironment } from '@/utils/validateEnvironment'

const {
  ENVIRONMENT,
  NODE_ENV,
  API_URL,
  AWS_REGION,
  USER_POOL_ID,
  USER_POOL_CLIENT_ID,
} = validateEnvironment()

/**
 * Set global configuration for the application provided by webpack (craco) at build time.
 * @see {@link @cyclonedx-python/ui/packages/sbom/craco.config.js}.
 */
export const CONFIG = {
  ENVIRONMENT,
  NODE_ENV,
  API_URL,
  AWS_REGION,
  USER_POOL_ID,
  USER_POOL_CLIENT_ID,
  TEAM_API_URL: `${API_URL}/team`,
  TEAMS_API_URL: `${API_URL}/teams`,
  USER_API_URL: `${API_URL}/user`,
  USER_API_SEARCH_URL: `${API_URL}/user/search`,
} as AppConfig
