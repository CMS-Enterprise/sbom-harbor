/**
 * @module @cyclonedx/ui/sbom/utils/constants
 * @exports CONFIG
 * @exports storageTokenKeyName
 */
import { AppConfig } from '@/types'

// parse `CONFIG` from environment variables
const envConfig = JSON.parse(JSON.stringify(process.env.CONFIG))

/**
 * Set global configuration for the application provided by webpack (craco) at build time.
 * @see {@link @cyclonedx-python/ui/packages/sbom/src/utils/prebuild.js} for implementation.
 */
export const CONFIG = {
  ...envConfig,
  TEAMS_API_URL: `${envConfig.API_URL}/team`,
  USER_API_URL: `${envConfig.API_URL}/user`,
} as AppConfig
