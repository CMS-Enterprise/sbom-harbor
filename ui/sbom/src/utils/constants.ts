/**
 * @module @cyclonedx/ui/sbom/utils/constants
 */
import { AppConfig } from './types'

// (craco) config. See {ui/sbom/src/utils/prebuild.js} for the implementation.
export const ENV_CONFIG = JSON.parse(JSON.stringify(process.env.CONFIG))

export const TEAMS_API_URL = `${ENV_CONFIG.API_URL}/team`
export const USER_API_URL = `${ENV_CONFIG.API_URL}/user`

// initialize the global app config object
export const CONFIG = {
  ...ENV_CONFIG,
  TEAMS_API_URL,
  USER_API_URL,
} as AppConfig
