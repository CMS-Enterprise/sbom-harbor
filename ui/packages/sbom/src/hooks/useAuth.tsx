/**
 * Custom React Hook to use the AuthContext in a functional component
 *  in order to access the AuthContext's state and dispatch functions.
 * @module @cyclonedx/ui-sbom/hooks/useAuth
 * @exports AuthContext
 * @exports AuthProvider
 * @exports useAuth
 */
import * as React from 'react'
import { useMatch, useNavigate, useLocation, redirect } from 'react-router-dom'
import { Auth } from '@aws-amplify/auth'
import { getUserData } from '@/utils/get-cognito-user'
import { AuthValuesType, LoginParams } from './types'
import useAlert from '@/hooks/useAlert'
import { UserDataType } from '@/types'

type AuthProviderProps = {
  children: React.ReactNode
}

/**
 * @type {AuthValuesType} The initial values provided by AuthContext.
 */
const defaultProvider: AuthValuesType = {
  user: null,
  loading: false,
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
  const [user, setUser]: [UserDataType, React.Dispatch<UserDataType>] =
    React.useState<UserDataType>(defaultProvider.user)
  const [loading, setLoading] = React.useState<boolean>(defaultProvider.loading)
  const [initialized, setInitialized] = React.useState<boolean>(false)

  const { setAlert } = useAlert()
  const location = useLocation()
  const navigate = useNavigate()
  const matchProtectedRoute = useMatch('/app/*')

  /**
   * Async function to check the validity of the user session and set user state.
   * @returns {Promise<void>} A promise that resolves when the user's sesson
   *  is resolved to a valid session, or rejects if no valid session exists.
   */
  const init = React.useCallback(async () => {
    setLoading(true)
    try {
      const { userInfo, ...rest } = await getUserData()

      const {
        id,
        attributes,
        attributes: { email, 'custom:teams': userTeams = '' } = {},
        username,
      } = userInfo

      const nextData = {
        ...rest,
        userInfo,
        attributes,
        id,
        email,
        username,
        teams: userTeams.split(','),
      }
      setUser(nextData)
      // @ts-ignore
      window.user = nextData
      setLoading(false)
      setInitialized(true)
      // if the unauthenticated user is trying to navigate to a
      // protected app routue, redirect them to the login page.
      if (!matchProtectedRoute && location.pathname !== '/logout') {
        navigate('/app')
      }
    } catch (error) {
      console.warn('init:', error)
      setUser(null)
      setLoading(false)
      setInitialized(true)
      // if the unauthenticated user is trying to navigate to a
      // protected app routue, redirect them to the login page.
      if (matchProtectedRoute) {
        navigate('/login')
      }
    }
  }, [loading, location.pathname, matchProtectedRoute])

  /**
   * Initializes the AuthContext by checking for a user session and setting
   *  the user state accordingly. If no valid user session exists, it sets
   *  the user state to null and clears local storage.
   */
  React.useEffect(() => {
    if (loading || initialized) return
    init()
  }, [matchProtectedRoute])

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
      await init()
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
      await init()
    } catch (error) {
      console.error(error)
      setAlert({ message: 'Error logging out', severity: 'error' })
    }
  }

  // set all the AuthContext values in the context provider.
  const values = {
    user,
    loading,
    setUser,
    setLoading,
    login: handleLogin,
    logout: handleLogout,
  }

  return <AuthContext.Provider value={values}>{children}</AuthContext.Provider>
}

export default useAuth
