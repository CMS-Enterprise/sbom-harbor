/**
 * The view that renders the authenticated user's layout.
 * @module @cyclonedx/ui/sbom/views/App/App
 */
// ** React Imports
import * as React from 'react'
import { Outlet } from 'react-router-dom'

// ** MUI Components
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Divider from '@mui/material/Divider'
import IconButton from '@mui/material/IconButton'
import List from '@mui/material/List'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'

// ** Icon
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft'
import MenuIcon from '@mui/icons-material/Menu'

// ** App Imports
import { useAlert, AlertMessage } from '@/hooks/useAlert'
import { useAuth } from '@/hooks/useAuth'
import { useData } from '@/hooks/useData'
import { getTeams } from '@/services/UserService'
import AppBar from '@/components/AppBar'
import AppDrawer from '@/components/AppDrawer'
import AuthButton from '@/components/HeaderAuthButton'

// ** Local Imports
import MenuListItems from './AppDrawerListItems'

/**
 * The main component that renders the application layout.
 * @returns {JSX.Element} The main application layout component.
 */
const App = (): JSX.Element => {
  // ** State
  const [drawerOpen, setDrawerOpen] = React.useState(true)
  const toggleDrawer = () => setDrawerOpen(!drawerOpen)

  // ** Hooks
  const { setAlert } = useAlert()
  const { user } = useAuth()
  const { data, setTeams } = useData()

  /**
   * Fetches the current users teams user whenever the current user changes.
   * @returns {() => AbortController.abort()} A function that cancels the request.
   */
  React.useEffect(() => {
    const abortController = new AbortController()

    /**
     * Fetches the current users teams user.
     * @async
     * @function fetchData
     * @returns {Promise<void>} The promise that resolves when the teams are fetched.
     * @throws {Error} If the teams could not be fetched.
     * @throws {AbortError} If the request was aborted.
     */
    const fetchTeamsData = async () => {
      if (!user || data.teams?.length) {
        console.debug('fetchTeamsData: user or data.teams is null')
        return
      }
      try {
        const teams = await getTeams(abortController)
        setTeams(teams || [])
      } catch (error) {
        console.error(error)
        setAlert({
          message: 'Error fetching teams',
          severity: 'error',
        })
      }
    }

    // call `fetchTeamsData` on mount and when the user changes.
    fetchTeamsData()

    // cleanup the abort controller when the component unmounts.
    return () => abortController.abort()
  }, [user])

  return (
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
        overflow: 'auto',
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
      <Container sx={{ mt: 8 }}>
        {/* @ts-ignore */}
        <Outlet />
      </Container>
    </Box>
  )
}

export default App
