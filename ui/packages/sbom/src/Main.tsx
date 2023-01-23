/**
 * The main component that renders all routes in the application.
 * @module @cyclonedx/ui/sbom/Main
 */
import * as React from 'react'
import { Outlet } from 'react-router-dom'
import CssBaseline from '@mui/material/CssBaseline'
import { ThemeProvider } from '@mui/material/styles'
import { AlertProvider } from '@/hooks/useAlert'
import { AuthProvider } from '@/hooks/useAuth'
import DialogProvider from '@/hooks/useDialog'
import theme from '@/utils/theme'

/**
 * Root Layout component that renders the entire application,
 *  including the public (home, auth) and private (app) views.
 * @returns {JSX.Element}
 */
const Main = (): JSX.Element => (
  <main data-testid="main">
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <AuthProvider>
        <AlertProvider>
          <DialogProvider>
            <Outlet />
          </DialogProvider>
        </AlertProvider>
      </AuthProvider>
    </ThemeProvider>
  </main>
)

export default Main
