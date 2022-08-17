/**
 * Types used in the AuthContext and AuthProvider.
 * @module @cyclone-dx/sbom/ui/AuthContext/types
 */
import { CognitoIdToken, CognitoUser } from 'amazon-cognito-identity-js'
import { CognitoUserInfo } from '@/utils/types'

export type ErrCallbackType = (err: Error) => void

export type UserDataState = UserDataType | null

export type AuthProviderProps = {
  children: React.ReactNode
}

export type LoginParams = {
  email: string
  password: string
}

export type RegisterParams = {
  email: string
  username: string
  password: string
}

export type UserDataType = {
  cognitoUser: CognitoUser
  userInfo?: CognitoUserInfo
  idToken?: CognitoIdToken
  id?: string
  role?: string
  jwt?: string
  avatar?: string | null
  email?: string
  fullName?: string
  password?: string
  username?: string
}

export type AuthValuesType = {
  /**
   * State variable holding current user's session data. If the user is not
   * logged in or the current session is expired/invalid, this will be null.
   */
  user: UserDataState
  /**
   * Dispatch function to update the state variable `user` corresponding to
   * the current user's session data, or to clear it when the user logs out.
   */
  setUser: (value: UserDataState) => void
  /**
   * Boolean flag indicating whether authentication state has initialized.
   * If `true`, the app has loaded, and the check for the existence of a
   * user session has been performed and completed. If a valid session was
   * found, the user's session data has loaded into the state variable `user`.
   * If `false`, the app has loaded but the check for the existence of a
   * user session has not yet been completed.
   * @see {@link @cyclonedx/ui-sbom/context/AuthContext}
   */
  initialized: boolean
  /**
   * Dispatch function to update the state variable `initialized`.
   * @param {boolean} value The value to set the state variable to.
   */
  setInitialized: (value: boolean) => void
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
   * @param {ErrCallbackType} errCallback The callback for handling errors.
   */
  login: (params: LoginParams, errorCallback?: ErrCallbackType) => void
  /**
   * Method for logging out the user and clearing their session data.
   */
  logout: () => void
}
