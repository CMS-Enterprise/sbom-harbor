/**
 * Custom React Hook to use the AuthContext in a functional component
 *  in order to access the AuthContext's state and dispatch functions.
 * @module @cyclonedx/ui-sbom/hooks/useAuth
 * @exports AuthContext
 * @exports AuthProvider
 * @exports useAuth
 */
import * as React from 'react'
import { useMatch, useNavigate, useLocation } from 'react-router-dom'
import { Auth } from 'aws-amplify'
import { AuthValuesType } from './types'

type AuthProviderProps = {
  children: React.ReactNode
}

type UserData = {
  jwtToken: string
}

const INITIAL_STATE = {
  jwtToken: '',
} as UserData

/**
 * @type {AuthValuesType} The initial values provided by AuthContext.
 */
export const defaultProvider: AuthValuesType = {
  ...INITIAL_STATE,
}

enum AuthActions {
  REQUEST_LOGIN = 'REQUEST_LOGIN',
  LOGIN_SUCCESS = 'LOGIN_SUCCESS',
  LOGIN_ERROR = 'LOGIN_ERROR',
  LOGOUT = 'LOGOUT',
}

interface AuthActionParams {
  type: AuthActions
  payload: AuthValuesType
  error: Error | null
}

export const AuthReducer = (
  initialState: AuthValuesType,
  action: React.ReducerAction<React.Reducer<string, AuthActionParams>>
) => {
  switch (action.type) {
    case AuthActions.REQUEST_LOGIN:
      return {
        ...initialState,
      }
    case AuthActions.LOGIN_SUCCESS:
      return {
        ...initialState,
        jwtToken: action.payload.jwtToken,
      }
    case AuthActions.LOGOUT:
      return {
        ...initialState,
        jwtToken: '',
      }

    case AuthActions.LOGIN_ERROR:
      return {
        ...initialState,
        error: action.error,
      }

    default:
      throw new Error(`Unhandled action type: ${action.type}`)
  }
}

export const AuthStateContext = React.createContext(INITIAL_STATE)

export const AuthDispatchContext = React.createContext(
  ((value: AuthActionParams) => value) as React.Dispatch<AuthActionParams>
)

export function useAuthState() {
  const context = React.useContext(AuthStateContext)
  if (context === undefined) {
    throw new Error('useAuthState must be used within a AuthProvider')
  }

  return context
}

export function useAuthDispatch() {
  const context = React.useContext(AuthDispatchContext)
  if (context === undefined) {
    throw new Error('useAuthDispatch must be used within a AuthProvider')
  }

  return context
}

/**
 * @default {AuthValuesType=defaultProvider} The initial AuthContext
 */
export const AuthContext = React.createContext(defaultProvider)

/**
 * The AuthContextProvider is used to provide user data to components.
 * @param {AuthProviderProps} props The input props passed to the component.
 * @param {React.ReactNode} props.children The children nodes being rendered.
 * @returns {JSX.Element} The rendered provider that wraps the children nodes.
 */
export const AuthProvider = ({ children }: AuthProviderProps) => {
  const location = useLocation()
  const navigate = useNavigate()
  const matchProtectedRoute = useMatch('/app/*')
  const [user, dispatch] = React.useReducer(AuthReducer, defaultProvider)

  /**
   * Async function to check the validity of the user session and set user state.
   * @returns {Promise<void>} A promise that resolves when the user's sesson
   *  is resolved to a valid session, or rejects if no valid session exists.
   */
  const init = React.useCallback(async () => {
    try {
      const user = await Auth.currentAuthenticatedUser()
      const session = await Auth.currentSession()

      if (!user || !session) {
        throw new Error('No user or session')
      }

      const jwtToken = session.getAccessToken().getJwtToken()

      // TODO: implement refresh sessions
      // user.refreshSession(
      //   session.getRefreshToken(),
      //   async (err: any, session: CognitoUserSession) => {
      //     if (err) {
      //       throw err
      //     }
      //   }
      // )

      const payload = {
        jwtToken,
      }

      dispatch({ type: AuthActions.LOGIN_SUCCESS, payload, error: null })

      // if the unauthenticated user is trying to navigate to a
      // protected app routue, redirect them to the login page.
      if (!matchProtectedRoute && location.pathname !== '/logout') {
        navigate('/app')
      }
    } catch (error) {
      dispatch({
        type: AuthActions.LOGIN_ERROR,
        error: error as Error,
        payload: defaultProvider,
      })
      // if the unauthenticated user is trying to navigate to a
      // protected app routue, redirect them to the login page.
      if (matchProtectedRoute) {
        navigate('/login')
      }
    }
  }, [location.pathname, matchProtectedRoute, navigate])

  /**
   * Initializes the AuthContext by checking for a user session and setting
   *  the user state accordingly. If no valid user session exists, it sets
   *  the user state to null and clears local storage.
   */
  /* eslint-disable react-hooks/exhaustive-deps */
  React.useEffect(() => {
    init()
  }, [matchProtectedRoute])
  /* eslint-enable react-hooks/exhaustive-deps */

  // set all the AuthContext values in the context provider.
  return (
    <AuthStateContext.Provider value={user}>
      <AuthDispatchContext.Provider value={dispatch}>
        {children}
      </AuthDispatchContext.Provider>
    </AuthStateContext.Provider>
  )
}
