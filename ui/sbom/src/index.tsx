/**
 * The entry point for the cyclonedx-ui-sbom frontend.
 * @module @cyclonedx/ui/sbom/index
 */
import * as React from 'react'
import * as ReactDOMClient from 'react-dom/client'
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom'
import { useNavigate, useMatch, useLocation } from 'react-router-dom'
import { CognitoUserSession } from 'amazon-cognito-identity-js'
import { Auth } from '@aws-amplify/auth'
import CssBaseline from '@mui/material/CssBaseline'
import { ThemeProvider } from '@mui/material/styles'
import Layout from '@/views/Layout/Layout'
import App from '@/views/App/App'
import Home from '@/views/Home/Home'
import SignIn from '@/views/SignIn/SignIn'
import SignUp from '@/views/SignUp/SignUp'
import { SessionContext, getSession } from '@/services/auth'
import reportWebVitals from '@/utils/reportWebVitals'
import theme from '@/theme'
import {
  AWS_REGION,
  AWS_USER_POOL_ID,
  AWS_USER_POOL_WEB_CLIENT_ID,
} from '@/utils/constants'

// configure Amazon Cognito authentication
Auth.configure({
  region: AWS_REGION,
  userPoolId: AWS_USER_POOL_ID,
  userPoolWebClientId: AWS_USER_POOL_WEB_CLIENT_ID,
})

const Main = (): JSX.Element => {
  const navigate = useNavigate()
  const match = useMatch({ path: '/app' })
  const location = useLocation()

  const [session, setSession] = React.useState<
    CognitoUserSession | null | undefined
  >(undefined)

  React.useEffect(() => {
    const fetchSession = async (): Promise<void> => {
      try {
        const res = await getSession()
        if (!res) throw new Error('No session')
        setSession(res)
        console.log('Session:', res)
      } catch (error) {
        console.log('Session error:', error)
        if (match) {
          setSession(null)
          navigate('/login')
        }
      }
    }

    fetchSession()
  }, [location.pathname])

  return (
    <SessionContext.Provider value={session}>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Home />} />
          <Route path="join" element={<SignUp />} />
          <Route path="login" element={<SignIn />} />
        </Route>
        <Route path="/app" element={<App />} />
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
