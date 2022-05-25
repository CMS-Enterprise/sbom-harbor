/**
 * Sign in page component that renders a login form.
 * @module @cyclonedx/ui/sbom/views/App/App
 */
import * as React from 'react'
import { Outlet } from 'react-router-dom'
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Footer from '@/components/Footer'
import Header from '@/components/Header'
import { SessionContext } from '@/services/auth'

const App = (): JSX.Element => {
  const session = React.useContext(SessionContext)

  return (
    <Box
      component="main"
      data-testid="app"
      sx={{ display: 'flex', flexDirection: 'column', height: '100vh' }}
    >
      <Header session={session} />

      <Container sx={{ mt: 2 }}>
        <h1>App</h1>
        <Outlet />
      </Container>

      <Footer />
    </Box>
  )
}

export default App
