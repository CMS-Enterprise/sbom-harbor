/**
 * The main component that renders all routes in the application.
 * @module @cyclonedx/ui/sbom/Main
 */
import * as React from 'react'
import { useLocation, useMatch, useNavigate } from 'react-router-dom'
import { Auth } from '@aws-amplify/auth'
import CssBaseline from '@mui/material/CssBaseline'
import { ThemeProvider } from '@mui/material/styles'

import Routes from '@/Routes'
import { AlertProvider } from '@/hooks/useAlert'
import { AuthProvider, useAuth } from '@/hooks/useAuth'
import { DataProvider } from '@/hooks/useData'
import theme from '@/utils/theme'

/**
 * Root Layout component that renders the entire application,
 *  including the public (home, auth) and private (app) views.
 * @returns {JSX.Element}
 */
const Main = (): JSX.Element => {
  // ** Hooks
  const { setUser } = useAuth()
  const navigate = useNavigate()
  const location = useLocation()
  const matchProtectedRoute = useMatch('/app/*')

  const fetchSession = React.useCallback(async () => {
    try {
      const user = await Auth.currentAuthenticatedUser()
      setUser(user)
      if (!matchProtectedRoute) navigate('/app')
    } catch (error) {
      console.error(error)
      setUser(null)
      if (matchProtectedRoute) navigate('/login')
    }
  }, [matchProtectedRoute])

  React.useEffect(() => {
    fetchSession()
  }, [location.pathname])

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <DataProvider>
        <AuthProvider>
          <AlertProvider>
            <Routes />
          </AlertProvider>
        </AuthProvider>
      </DataProvider>
    </ThemeProvider>
  )
}

export default Main
