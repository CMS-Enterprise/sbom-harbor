/**
 * The main component that renders all routes in the application.
 * @module @cyclonedx/ui/sbom/Main
 */
import * as React from 'react'
import CssBaseline from '@mui/material/CssBaseline'
import { ThemeProvider } from '@mui/material/styles'

import Routes from '@/Routes'
import { AlertProvider } from '@/hooks/useAlert'
import { AuthProvider } from '@/hooks/useAuth'
import { DataProvider } from '@/hooks/useData'
import theme from '@/utils/theme'

/**
 * Root Layout component that renders the entire application,
 *  including the public (home, auth) and private (app) views.
 * @returns {JSX.Element}
 */
const Main = (): JSX.Element => (
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

export default Main
