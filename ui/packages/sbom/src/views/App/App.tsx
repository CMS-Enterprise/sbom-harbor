/**
 * The view that renders the authenticated user's layout.
 * @module @cyclonedx/ui/sbom/views/App/App
 */
// ** React Imports
import * as React from 'react'
import { Await, Outlet, useAsyncValue, useLoaderData } from 'react-router-dom'

// ** MUI Components
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Divider from '@mui/material/Divider'
import IconButton from '@mui/material/IconButton'
import List from '@mui/material/List'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'

// ** Icons
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft'
import MenuIcon from '@mui/icons-material/Menu'

// ** App Imports
import AlertMessage from '@/components/AlertMessage'
import AppBar from '@/components/AppBar'
import AppDrawer from '@/components/AppDrawer'
import AuthButton from '@/components/HeaderAuthButton'

// ** Local Imports
import MenuListItems from './AppDrawerListItems'

import { AuthContext } from '@/hooks/useAuth'

const AppContent = (): JSX.Element => {
  const jwtToken = useAsyncValue() as string
  const [drawerOpen, setDrawerOpen] = React.useState(true)
  const toggleDrawer = () => setDrawerOpen(!drawerOpen)

  return (
    <AuthContext.Provider value={{ jwtToken }}>
      <Box
        data-testid="app"
        sx={{
          display: 'flex',
          backgroundColor: (theme) =>
            theme.palette.mode === 'light'
              ? theme.palette.grey[100]
              : theme.palette.grey[900],
          flexGrow: 1,
          height: '100vh',
          overflow: 'hidden',
        }}
      >
        <AlertMessage />

        {/* top nav bar */}
        <AppBar position="absolute" open={drawerOpen} color="secondary">
          <Toolbar>
            <IconButton
              edge="start"
              color="inherit"
              aria-label="open drawer"
              onClick={toggleDrawer}
              sx={{
                marginRight: '36px',
                ...(drawerOpen && { display: 'none' }),
              }}
            >
              <MenuIcon />
            </IconButton>
            <Typography
              component="h1"
              variant="h6"
              color="inherit"
              noWrap
              sx={{ flexGrow: 1 }}
            >
              Dashboard
            </Typography>
            <AuthButton />
          </Toolbar>
        </AppBar>

        {/* drawer */}
        <AppDrawer variant="permanent" open={drawerOpen}>
          <Toolbar
            sx={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'flex-end',
              filter: `brightness(80%)`,
            }}
          >
            <IconButton onClick={toggleDrawer}>
              <ChevronLeftIcon />
            </IconButton>
          </Toolbar>
          <Divider />

          <List component="nav">
            <MenuListItems />
          </List>
        </AppDrawer>

        {/* main content outlet for child routes */}
        <Container sx={{ mt: 8, overflow: 'scroll' }}>
          {/* @ts-ignore */}
          <Outlet />
        </Container>
      </Box>
    </AuthContext.Provider>
  )
}

/**
 * The main component that renders the application layout.
 * @returns {JSX.Element} The main application layout component.
 */
const App = (): JSX.Element => {
  const jwtToken = useLoaderData() as string

  return (
    <Await resolve={jwtToken}>
      <AppContent />
    </Await>
  )
}

export default App
