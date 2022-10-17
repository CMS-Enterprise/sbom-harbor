/**
 * Custom React Hook to use the AuthContext in a functional component
 *  in order to access the AuthContext's state and dispatch functions.
 * @module @cyclonedx/ui-sbom/hooks/useAuth
 * @exports AuthContext
 * @exports AuthProvider
 * @exports useAuth
 */
import * as React from 'react'
import { useMatch, useNavigate } from 'react-router-dom'
import { Auth } from '@aws-amplify/auth'
import { getUserData } from '@/utils/get-cognito-user'
import { UserDataType } from '@/types'
import { AuthValuesType, LoginParams } from './types'
import useAlert from '@/hooks/useAlert'

type UserDataState = UserDataType | null

type AuthProviderProps = {
  children: React.ReactNode
}

/**
 * @type {AuthValuesType} The initial values provided by AuthContext.
 */
const defaultProvider: AuthValuesType = {
  user: null,
  loading: true,
  updateUser: () => null,
  setUser: () => null,
  setLoading: () => Boolean,
  login: () => Promise.resolve(),
  logout: () => Promise.resolve(),
}

/**
 * @default {AuthValuesType=defaultProvider} The initial AuthContext
 */
export const AuthContext = React.createContext(defaultProvider)

/**
 * The custom hook to use the AuthContext in a functional component.
 * @returns {AuthValuesType} Hook that returns the current AuthContext value.
 */
export const useAuth = () => React.useContext(AuthContext)

/**
 * The AuthContextProvider is used to provide user data to components.
 * @param {AuthProviderProps} props The input props passed to the component.
 * @param {React.ReactNode} props.children The children nodes being rendered.
 * @returns {JSX.Element} The rendered provider that wraps the children nodes.
 */
export const AuthProvider = ({ children }: AuthProviderProps) => {
  const [user, setUser]: [UserDataState, React.Dispatch<UserDataState>] =
    React.useState<UserDataState>(defaultProvider.user)
  const [loading, setLoading] = React.useState<boolean>(defaultProvider.loading)

  const { setAlert } = useAlert()
  const navigate = useNavigate()
  const matchProtectedRoute = useMatch('/app/*')

  const updateUser = React.useCallback(async () => {
    try {
      const { userInfo, ...rest } = await getUserData()
      const { id, attributes, attributes: { email } = {}, username } = userInfo
      const nextData = { ...rest, userInfo, attributes, id, email, username }
      setUser(nextData)
      console.debug('User data updated.', nextData)
    } catch (error) {
      setUser(null)
      // we still throw the error to make sure this causes an error in the initAuth handler
      throw error
    }
  }, [setUser])

  /**
   * Async function to check the validity of the user session and set user state.
   * @returns {Promise<void>} A promise that resolves when the user's sesson
   *  is resolved to a valid session, or rejects if no valid session exists.
   */
  /* eslint-disable react-hooks/exhaustive-deps */
  const initAuth = React.useCallback(async (): Promise<void> => {
    try {
      setLoading(true)
      await updateUser()
      setLoading(false)
      // if the user has navigated to a non-protected route
      // outside of the app, redirect them back to the app.
      if (!matchProtectedRoute) {
        navigate('/app')
      }
    } catch (error) {
      console.warn('initAuth:', error)
      setUser(null)
      setLoading(false)
      // if the unauthenticated user is trying to navigate to a
      // protected app routue, redirect them to the login page.
      if (matchProtectedRoute) {
        navigate('/login')
      }
    }
  }, [matchProtectedRoute])
  /* eslint-enable react-hooks/exhaustive-deps */

  /**
   * Initializes the AuthContext by checking for a user session and setting
   *  the user state accordingly. If no valid user session exists, it sets
   *  the user state to null and clears local storage.
   */
  /* eslint-disable react-hooks/exhaustive-deps */
  React.useEffect(() => {
    initAuth()
  }, [matchProtectedRoute])
  /* eslint-enable react-hooks/exhaustive-deps */

  /**
   * Function to handle logging the user in with the provided credentials.
   * @param params {LoginParams} The parameters to use when logging the user in.
   * @param {ErrorCallbackType} errorCallback The callback to invoke if an error
   * @returns {Promise<void>} A promise that resolves when user is logged in,
   *  or rejects if an error occurs.
   */
  const handleLogin = async (
    params: LoginParams,
    errorCallback?: ErrorCallbackType
  ): Promise<void> => {
    try {
      setLoading(true)
      await Auth.signIn(params.email, params.password)
      await initAuth()
    } catch (error) {
      console.warn(error)
      setLoading(false)
      setAlert({ message: 'Error logging in', severity: 'error' })
      if (errorCallback) errorCallback(error as Error)
    }
  }

  /**
   * Function to handle logging the user out and clearing the stored session.
   * @returns {Promise<void>} A promise that resolves when the user has been
   *  logged out, or rejects if an error occurs when making the API request.
   */
  const handleLogout = async (): Promise<void> => {
    try {
      await Auth.signOut()
      await initAuth()
    } catch (error) {
      console.warn(error)
      setLoading(false)
      setAlert({ message: 'Error logging out', severity: 'error' })
    }
  }

  // set all the AuthContext values in the context provider.
  const values = {
    user,
    loading,
    updateUser,
    setUser,
    setLoading,
    login: handleLogin,
    logout: handleLogout,
  }

  return <AuthContext.Provider value={values}>{children}</AuthContext.Provider>
}
