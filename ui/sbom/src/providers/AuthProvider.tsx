/**
 * The AuthContext is used to provide (1) the current user data to components
 *  that need it, (2) methods for handling logging the user in and out, and (3)
 *  methods for dispatching updates to the user data state in the AuthContext.
 * @module @cyclonedx/ui-sbom/context/AuthContext
 */
// TODO: refactor to use fetch instead of axios
import axios from 'axios'

// ** React Imports
import * as React from 'react'
import { useMatch, useNavigate } from 'react-router-dom'

// ** AWS Imports
import { Auth } from '@aws-amplify/auth'
import { CognitoIdToken, CognitoUser } from 'amazon-cognito-identity-js'

// ** App Imports
import authConfig from '@/configs/auth'
import { useData } from '@/hooks/useData'
import { CONFIG } from '@/utils/constants'
import { getUserData } from '@/utils/get-cognito-user'
import { CognitoUserInfo, Team } from '@/utils/types'
import {
  AuthProviderProps,
  AuthValuesType,
  ErrCallbackType,
  LoginParams,
  UserDataState,
} from './types'

/** @constant {string} MY_TEAMS_ENDPOINT The API endpoint for fetching teams */
const MY_TEAMS_ENDPOINT = `${CONFIG.API_URL}/${authConfig.myTeamsEndpoint}`

/** @type {AuthValuesType} The initial values provided by AuthContext. */
const defaultProvider: AuthValuesType = {
  user: null,
  loading: true,
  setUser: () => null,
  setLoading: () => Boolean,
  initialized: false,
  login: () => Promise.resolve(),
  logout: () => Promise.resolve(),
  setInitialized: () => Boolean,
}

/** @default {AuthValuesType=defaultProvider} The initial AuthContext */
const AuthContext = React.createContext(defaultProvider)

/**
 * The AuthContextProvider is used to provide user data to components.
 * @param {AuthProviderProps} props The input props passed to the component.
 * @param {React.ReactNode} props.children The children nodes being rendered.
 * @returns {JSX.Element} The rendered provider that wraps the children nodes.
 */
const AuthProvider = ({ children }: AuthProviderProps) => {
  // state
  const [user, setUser] = React.useState<UserDataState>(defaultProvider.user)
  const [loading, setLoading] = React.useState<boolean>(defaultProvider.loading)
  const [initialized, setInitialized] = React.useState<boolean>(
    defaultProvider.initialized
  )

  // hooks
  const { setTeams } = useData()
  const navigate = useNavigate()
  const matchProtectedRoute = useMatch('/app/*')

  /**
   * Initializes the AuthContext by checking for a user session and setting the
   *  user state accordingly. If a valid user session exists, this fetches the
   *  user's teams and sets the user.teams state accordingly. If no valid user
   *  session exists, this sets the user state to null and clears local storage.
   * This effect executes before child components render, and never again after.
   */
  React.useEffect(() => {
    /**
     * Async function to initialize the AuthContext and fetch the user's data.
     * @returns {Promise<void>} A promise that resolves when the user's teams
     *  data has been fetched, or rejects if no valid session exists, or if
     *  an error occurs when making the API call to fetch the user data.
     */
    const initAuth = async (): Promise<void> => {
      try {
        setLoading(true)

        // get the current user's data and id token from Cognito.
        const { user: cognitoUser, userInfo, idToken } = await getUserData()

        // get users teams data from the API before rendering the app.
        const response = await axios.get(
          `${MY_TEAMS_ENDPOINT}?user_id=${userInfo.attributes.email}`,
          { headers: { Authorization: idToken.getJwtToken() } }
        )

        // set user and teams data in the context provider.
        setUser({
          cognitoUser,
          userInfo,
          id: userInfo.id,
          email: userInfo.attributes.email,
          idToken,
          jwt: idToken.getJwtToken(),
        })
        setTeams((response.data || []) as Team[])

        // set the loading state to false to indicate the data is fetched.
        setLoading(false)
        // set initialized to true to prevent this effect from running again.
        setInitialized(true)

        // if the user has navigated to a non-protected route
        // outside of the app, redirect them back to the app.
        if (!matchProtectedRoute) {
          navigate('/app')
        }
      } catch (error) {
        // set the loading flag in the context provider to `false`.
        setLoading(false)
        // if there is no valid session, clear any user data in
        // the auth context and remove any localStorage items.
        setUser(null)
        localStorage.removeItem('userData')
        localStorage.removeItem('refreshToken')
        localStorage.removeItem('accessToken')
        // if the unauthenticated user is trying to navigate to a
        // protected app routue, redirect them to the login page.
        if (matchProtectedRoute) {
          navigate('/login')
        }
      }
    }
    // execute the async function to initialize the AuthContext.
    initAuth()
  }, [])

  /**
   * Function to handle logging the user in with the provided credentials.
   * @param params {LoginParams} The parameters to use when logging the user in.
   * @param {ErrCallbackType} errorCallback The callback to invoke if an error
   * @returns {Promise<void>} A promise that resolves when user is logged in,
   *  or rejects if an error occurs.
   */
  const handleLogin = async (
    params: LoginParams,
    errorCallback?: ErrCallbackType
  ): Promise<void> => {
    try {
      // set the loading state to true to indicate the request is in progress.
      setLoading(true)

      // use the Amplify SDK's Auth.signIn API to sign in the user.
      await Auth.signIn(params.email, params.password)

      // get the current user's data and id token from Cognito.
      const [cognitoUser, userInfo, idToken]: [
        CognitoUser,
        CognitoUserInfo,
        CognitoIdToken
      ] = await Promise.all([
        Auth.currentAuthenticatedUser(),
        Auth.currentUserInfo(),
        Auth.currentSession().then((session) => session.getIdToken()),
      ])

      // dispatch update to the user data state in the context provider.
      setUser({
        cognitoUser,
        userInfo,
        id: userInfo.id,
        email: userInfo.attributes.email,
        idToken,
        jwt: idToken.getJwtToken(),
      })

      // set the user data in local storage.
      await localStorage.setItem('userData', JSON.stringify(user))

      // fetch users teams data from the API before rendering the app.
      const requestUrl = `${MY_TEAMS_ENDPOINT}?user_id=${userInfo.attributes.email}`
      const response = await axios.get(requestUrl, {
        headers: { Authorization: idToken.getJwtToken() },
      })

      // dispatch update to the teams data state in the Data Provider
      setTeams((response.data || []) as Team[])

      // set the loading state to false to indicate the data is fetched.
      setLoading(false)

      // navigate to the main app that the user is trying to access.
      navigate('/app')
    } catch (error) {
      // set the loading state to false since the login failed.
      setLoading(false)
      // call the error callback if it was provided.
      if (errorCallback) errorCallback(error as Error)
      // TODO: show error message to user if there is an error.
      console.error(error)
    }
  }

  /**
   * Function to handle logging the user out and clearing the stored session.
   * @returns {Promise<void>} A promise that resolves when the user has been
   *  logged out, or rejects if an error occurs when making the API request.
   */
  const handleLogout = async (): Promise<void> => {
    try {
      // make the API call to log the user out.
      await Auth.signOut()
      // dispatch an update to set AuthContext.initialized flag to false.
      setInitialized(false)
      // clear the user data in the context provider.
      setUser(null)
      // clear the user data in local storage.
      localStorage.removeItem('userData')
      localStorage.removeItem(authConfig.storageTokenKeyName)
      localStorage.removeItem('refreshToken')
      localStorage.removeItem('accessToken')
      // set the loading flag in the context provider back to `false`.
      setLoading(false)
      // finally, navigate to the login page.
      navigate('/login')
    } catch (error) {
      // TODO: show error message to user if there is an error.
      console.error(error)
    }
  }

  // set all the AuthContext values in the context provider.
  const values = {
    user,
    loading,
    setUser,
    setLoading,
    initialized,
    setInitialized,
    login: handleLogin,
    logout: handleLogout,
  }

  return <AuthContext.Provider value={values}>{children}</AuthContext.Provider>
}

export { AuthContext, AuthProvider }
