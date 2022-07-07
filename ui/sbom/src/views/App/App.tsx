/**
 * The view that renders the authenticated user's layout.
 * @module @cyclonedx/ui/sbom/views/App/App
 */
import * as React from 'react'
import { Route, Routes } from 'react-router-dom'
import { createTheme, Theme, ThemeProvider } from '@mui/material/styles'
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Divider from '@mui/material/Divider'
import IconButton from '@mui/material/IconButton'
import List from '@mui/material/List'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import { AuthContext } from '@/providers/AuthContext'
import { useConfig } from '@/providers/ConfigContext'
import { useData } from '@/providers/DataContext'
import AppBar from '@/components/AppBar'
import AuthButton from '@/components/HeaderAuthButton'
import AppDrawer from '@/views/App/components/AppDrawer'
import MenuListItems from '@/views/App/components/AppDrawerListItems'
import Dashboard from '@/views/Dashboard/Dashboard'
import Team from '@/views/Dashboard/Team/Team'
import { ReactComponent as ChevronLeftIcon } from '@/assets/icons/ChevronLeft.svg'
import { ReactComponent as MenuIcon } from '@/assets/icons/Menu.svg'

const TEAM_ID = 'abc123'

const toolbarBackgroundColor = (theme: Theme): string =>
  theme.palette.mode === 'light' ? theme.palette.grey[100] : '#050F69'

const appTheme = createTheme({
  palette: {
    mode: 'dark',
  },
})

const App = (): JSX.Element => {
  // get the Teams API url from readonly ConfigContext
  const { TEAMS_API_URL } = useConfig()

  // get app data and the dispatch function to update it from DataContext
  const { data, setValues } = useData()

  // use the session context to get the user
  const { user } = React.useContext(AuthContext)

  // create the state for the left-side navigation drawer
  const [open, setOpen] = React.useState(true)
  const toggleDrawer = () => setOpen(!open)

  // effect to fetch the current user's team from the Teams API endpoint, which
  // currently only supports one team per user where the team ID is hardcoded.
  // TODO: fetch all of the user's teams instead of this example team.
  React.useEffect(() => {
    const abortController = new AbortController()

    /**
     * Fetches the current user's team from the Teams API endpoint.
     * @returns {Promise<void>} Promise that resolves when the team is fetched.
     */
    const fetchData = async () => {
      // return early if the user session data has not yet populated,
      // or if fetching the teams data has completed and loaded.
      if (!user || data?.teams?.length) return

      try {
        // get the user's JWT from Amplify.Auth
        const token = user
          ?.getSignInUserSession()
          ?.getAccessToken()
          ?.getJwtToken()

        // fetch the user's team from the Teams API endpoint
        const response = await fetch(`${TEAMS_API_URL}/${TEAM_ID}`, {
          signal: abortController.signal,
          headers: { Authorization: `Bearer ${token}` },
        })

        // parse the response as JSON for the team data
        const { response: team = {} } = await response.json()

        // dispatch the team data to the app data context to update the state
        setValues({
          teams: [team],
          user: user.getSignInUserSession(),
        })
      } catch (error) {
        console.error(error)
      }
    }

    // make the async call to fetch the team data
    fetchData()

    // clean up the abort controller when the component unmounts
    return () => abortController.abort()
  }, [user])

  return (
    <ThemeProvider theme={appTheme}>
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
        <AppBar position="absolute" open={open} color="secondary">
          <Toolbar
            sx={{
              pr: '24px', // keep right padding when drawer closed
              backgroundColor: toolbarBackgroundColor,
            }}
          >
            <IconButton
              edge="start"
              color="inherit"
              aria-label="open drawer"
              onClick={toggleDrawer}
              sx={{
                marginRight: '36px',
                ...(open && { display: 'none' }),
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
        <AppDrawer variant="permanent" open={open}>
          <Toolbar
            sx={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'flex-end',
              px: [1],
              filter: `brightness(80%)`,
              backgroundColor: toolbarBackgroundColor,
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
        <Container sx={{ mt: 8 }}>
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="team" element={<Team />} />
          </Routes>
        </Container>
      </Box>
    </ThemeProvider>
  )
}

export default App
