/**
 * The parent layout component that renders all other layouts in the application.
 * @module @cyclonedx/ui/sbom/index
 */
import * as React from 'react'
import {
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
import { AuthContext } from '@/providers/AuthContext'
import { DataProvider } from '@/providers/DataContext'
import PublicRouteLayout from '@/layouts/PublicRouteLayout'
import App from '@/views/App/App'
import Home from '@/views/Home/Home'
import SignIn from '@/views/SignIn/SignIn'
import SignOut from '@/views/SignOut/SignOut'
import SignUp from '@/views/SignUp/SignUp'
import theme from '@/utils/theme'

const RootLayout = (): JSX.Element => {
  const navigate = useNavigate()
  const location = useLocation()
  const matchProtectedRoute = useMatch('/app/*')

  // TODO: implement a `useUser` hook to take hooks out of this component
  const [user, setUser] = React.useState<CognitoUser | null | undefined>()

  const fetchSession = React.useCallback(async () => {
    try {
      const user = await Auth.currentAuthenticatedUser()
      setUser(user)
      if (!matchProtectedRoute) {
        navigate('/app')
      }
    } catch (error) {
      console.error(error)
      setUser(null)
      if (matchProtectedRoute) {
        navigate('/login')
      }
    }
  }, [matchProtectedRoute])

  React.useEffect(() => {
    fetchSession()
  }, [location.pathname])

  return (
    <>
      <ThemeProvider theme={theme}>
        <AuthContext.Provider
          value={{
            user,
            setUser,
          }}
        >
          <CssBaseline />
          <DataProvider>
            <Routes>
              <Route path="/" element={<PublicRouteLayout />}>
                <Route index element={<Home />} />
                <Route path="join" element={<SignUp />} />
                <Route path="login" element={<SignIn />} />
                <Route path="logout" element={<SignOut />} />
              </Route>
              <Route path="/app/*" element={<App />} />
            </Routes>
          </DataProvider>
        </AuthContext.Provider>
      </ThemeProvider>
    </>
  )
}

export default RootLayout
