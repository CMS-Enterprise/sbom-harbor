import { UserDataType } from '@/types'

export type LoginParams = {
  email: string
  password: string
}

/**
 * Types used in custom hooks.
 * @module @cyclone-dx/sbom/ui/hooks/types
 */
export type AuthValuesType = {
  /**
   * State variable holding current user's session data. If the user is not
   * logged in or the current session is expired/invalid, this will be null.
   */
  user: UserDataType
  /**
   * Dispatch function to update the state variable `user` corresponding to
   * the current user's session data, or to clear it when the user logs out.
   */
  setUser: (value: UserDataType) => void
  /**
   * Dispatch function to get the user data from Cognito and put it in state
   * by calling `setUser`.
   */
  /**
   * If `true`, an request to perform an authentication action is in progress.
   * This is used to prevent multiple authentication requests from being sent
   * at the same time.
   */
  loading: boolean
  /**
   * At the start of their execution, the `login` and `logout` methods call
   * `setLoading` to update the `loading` to `true`. When the requests are
   * complete, they call `setLoading` again to set `loading` back to `false`.
   * @param {boolean} value The value to set the state variable to.
   */
  setLoading: (value: boolean) => void
  /**
   * Method for logging in a user with a username and password.
   * @param {LoginParams} params The username and password to use to login.
   * @param {ErrorCallbackType} errCallback The callback for handling errors.
   */
  login: (
    params: LoginParams,
    errorCallback?: ErrorCallbackType
  ) => Promise<void>
  /**
   * Method for logging out the user and clearing their session data.
   */
  logout: () => Promise<void>
}
