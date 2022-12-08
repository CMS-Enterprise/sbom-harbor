/**
 * @module @cyclonedx/ui/sbom/utils/constants
 * @exports CONFIG
 * @exports storageTokenKeyName
 */
import { AppConfig } from '@/types'

let envConfig: AppConfig

// parse `CONFIG` from environment variables
if (process.env.NODE_ENV !== 'test') {
  envConfig = JSON.parse(JSON.stringify(process.env.CONFIG))
} else {
  envConfig = JSON.parse(process.env.CONFIG || '{}')
}

/**
 * Set global configuration for the application provided by webpack (craco) at build time.
 * @see {@link @cyclonedx-python/ui/packages/sbom/src/utils/prebuild.js} for implementation.
 */
export const CONFIG = {
  ...envConfig,
  TEAMS_API_URL: `${envConfig.API_URL}/v1/teams`,
  TEAM_API_URL: `${envConfig.API_URL}/v1/team`,
  USER_API_URL: `${envConfig.API_URL}/user`,
} as AppConfig
