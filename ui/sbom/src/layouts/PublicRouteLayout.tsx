/**
 * The layout for rendering any public routes like the landing page.
 * @module @cyclonedx/ui/sbom/index
 */
import * as React from 'react'
import { Outlet } from 'react-router-dom'
import Box from '@mui/material/Box'
import Footer from '@/components/Footer'
import Header from '@/components/Header'

const PublicRouteLayout = (): JSX.Element => (
  <Box
    component="main"
    data-testid="layout"
    sx={{ display: 'flex', flexDirection: 'column', height: '100vh' }}
  >
    <Header />
    <Outlet />
    <Footer />
  </Box>
)

export default PublicRouteLayout
