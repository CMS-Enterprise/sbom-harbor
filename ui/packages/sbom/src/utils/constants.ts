/**
 * @module @cyclonedx/ui/sbom/utils/constants
 * @exports CONFIG
 * @exports storageTokenKeyName
 */
import { AppConfig } from '@/types'
import { validateEnvironment, validateURL } from '@/utils/validateEnv'

const ENV = validateEnvironment()

const apiUrl = validateURL(`${ENV.CF_DOMAIN}/api`).toString()

/**
 * Set global configuration for the application provided by webpack (craco) at build time.
 * @see {@link @cyclonedx-python/ui/packages/sbom/craco.config.js}.
 */
export const CONFIG = {
  ...ENV,
  API_URL: apiUrl,
  TEAM_API_URL: `${apiUrl}/v1/team`,
  TEAMS_API_URL: `${apiUrl}/v1/teams`,
  USER_API_URL: `${apiUrl}/v1/user`,
  USER_API_SEARCH_URL: `${apiUrl}/v1/user/search`,
} as AppConfig
