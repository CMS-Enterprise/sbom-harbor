export type LoginParams = {
  email: string
  password: string
}

/**
 * Types used in custom hooks.
 * @module @cyclone-dx/sbom/ui/hooks/types
 */
export type AuthValuesType = {
  jwtToken?: string
  username?: string
  email: string
  teams: string[]
  errorMessage: any
  /**
   * If `true`, an request to perform an authentication action is in progress.
   * This is used to prevent multiple authentication requests from being sent
   * at the same time.
   */
  loading: boolean
}
