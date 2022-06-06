/**
 * The entry point for the cyclonedx-ui-sbom frontend.
 * @module @cyclonedx/ui/sbom/index
 */
import * as React from 'react'
import * as ReactDOMClient from 'react-dom/client'
import {
  BrowserRouter as Router,
  Route,
  Routes,
  useNavigate,
  useMatch,
  useLocation,
} from 'react-router-dom'
import { CognitoUser } from 'amazon-cognito-identity-js'
import { Auth } from '@aws-amplify/auth'
import { ThemeProvider } from '@mui/material/styles'
import CssBaseline from '@mui/material/CssBaseline'
import Layout from '@/views/Layout/Layout'
import { SessionContext } from '@/services/auth'
import { AWS_REGION } from '@/utils/constants'
import reportWebVitals from '@/utils/reportWebVitals'
import App from '@/views/App/App'
import Home from '@/views/Home/Home'
import SignIn from '@/views/SignIn/SignIn'
import SignUp from '@/views/SignUp/SignUp'
import SignOut from '@/views/SignOut/SignOut'
import theme from '@/theme'

type IConfig = {
  USER_POOL_ID: string
  USER_POOL_CLIENT_ID: string
  CF_DOMAIN: string
}

const CONFIG = JSON.parse(JSON.stringify(process.env.CONFIG)) as IConfig

// configure Amazon Cognito authentication
Auth.configure({
  region: AWS_REGION,
  userPoolId: CONFIG.USER_POOL_ID || new Error('USER_POOL_ID is not defined'),
  userPoolWebClientId:
    CONFIG.USER_POOL_CLIENT_ID ||
    new Error('USER_POOL_CLIENT_ID is not defined'),
})

console.log(CONFIG)

const Main = (): JSX.Element => {
  const navigate = useNavigate()
  const location = useLocation()
  const matchProtectedRoute = useMatch('/app/*')
  const [user, setUser] = React.useState<CognitoUser | null | undefined>()

  /* eslint-disable react-hooks/exhaustive-deps */
  const fetchSession = React.useCallback(async () => {
    try {
      const user = await Auth.currentAuthenticatedUser()
      setUser(user)
      console.log(`User@${location.pathname}`, user)
      if (!matchProtectedRoute) {
        navigate('/app')
      }
    } catch (error) {
      setUser(null)
      console.log(error)
      if (matchProtectedRoute) {
        navigate('/login')
      }
    }
  }, [matchProtectedRoute, user, location.pathname])

  React.useEffect(() => {
    fetchSession()
  }, [location.pathname])
  /* eslint-enable react-hooks/exhaustive-deps */

  return (
    <SessionContext.Provider
      value={{
        user,
        setUser,
      }}
    >
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Home />} />
          <Route path="join" element={<SignUp />} />
          <Route path="login" element={<SignIn />} />
          <Route path="logout" element={<SignOut />} />
        </Route>
        <Route path="/app/*" element={<App />} />
      </Routes>
    </SessionContext.Provider>
  )
}

/**
 * IIFE that initializes the root node and renders the application.
 */
;(async function () {
  // create the element in the DOM
  const rootElement = document.createElement('div')
  rootElement.id = 'root'
  document.body.appendChild(rootElement)

  // create the React root node and render the application
  const root = ReactDOMClient.createRoot(rootElement)
  root.render(
    <React.StrictMode>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <Router>
          <Main />
        </Router>
      </ThemeProvider>
    </React.StrictMode>
  )
})()

if (process.env.NODE_ENV !== 'production') {
  /** @see https://create-react-app.dev/docs/measuring-performance/ */
  reportWebVitals(console.log)
}
