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
 * @property {string} jwtToken - The user's jwtToken
 * @see {@link @cyclone-dx/sbom/ui/hooks/useAuth}
 */
export type AuthValuesType = {
  jwtToken: string
}
