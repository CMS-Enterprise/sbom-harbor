/**
 * Types used in custom hooks.
 * @module @cyclone-dx/sbom/ui/hooks/types
 */

/**
 * @typedef {Object} LoginParams - Parameters for the login hook.
 * @property {string} email - The user's email address.
 * @property {string} password - The user's password.
 * @see {@link @cyclone-dx/sbom/ui/actions/loginUser}
 */
export type LoginParams = {
  email: string
  password: string
}

/**
 * @typedef {Object} AuthValuesType - Parameters for the login hook.
 * @property {string} email - The user's email address.
 * @property {string[]} teams - The user's team ids, if any.
 * @property {string} jwtToken - The user's jwtToken
 * @property {string} username - The user's username
 * @property {string} loading - Whether there's an auth request in progress.
 * @see {@link @cyclone-dx/sbom/ui/hooks/useAuth}
 */
export type AuthValuesType = {
  email: string
  teams: string[]
  jwtToken?: string
  username?: string
  /**
   * If `true`, an request to perform an authentication action is in progress.
   * This is used to prevent multiple authentication requests from being sent
   * at the same time.
   */
  loading: boolean
}
