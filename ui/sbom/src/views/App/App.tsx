/**
 * Default application template component that renders the header, main content, and footer.
 * @module @cyclonedx/ui/sbom/views/App/App
 */
import * as React from 'react'
import { Outlet } from 'react-router-dom'
import Box from '@mui/material/Box'
import Footer from '@/components/Footer'
import Header from '@/components/Header'

const App = (): JSX.Element => (
  <Box
    component="main"
    data-testid="app"
    sx={{ display: 'flex', flexDirection: 'column', height: '100vh' }}
  >
    <Header />
    <Outlet />
    <Footer />
  </Box>
)

export default App
