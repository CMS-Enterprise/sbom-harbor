/**
 * Sign in page component that renders a login form.
 * @module @cyclonedx/ui/sbom/views/App/App
 */
import * as React from 'react'
import { Route, Routes } from 'react-router-dom'
import { styled, createTheme, ThemeProvider, Theme } from '@mui/material/styles'
import Box from '@mui/material/Box'
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft'
import Container from '@mui/material/Container'
import Divider from '@mui/material/Divider'
import IconButton from '@mui/material/IconButton'
import List from '@mui/material/List'
import MenuIcon from '@mui/icons-material/Menu'
import MuiDrawer from '@mui/material/Drawer'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import AppBar from '@/components/AppBar'
import AuthButton from '@/components/HeaderAuthButton'
import MenuListItems from '@/views/App/components/MenuListItems'
import Dashboard from '@/views/Dashboard/Dashboard'
import { MuiDrawerWidth } from '@/utils/constants'
import Team from '@/views/Dashboard/Team/Team'

const mdTheme = createTheme({
  palette: {
    mode: 'dark',
  },
})

const toolbarBackgroundColor = (theme: Theme): string =>
  theme.palette.mode === 'light' ? theme.palette.grey[100] : '#050F69'

const Drawer = styled(MuiDrawer, {
  shouldForwardProp: (prop) => prop !== 'open',
})(({ theme, open }) => ({
  '& .MuiDrawer-paper': {
    position: 'relative',
    whiteSpace: 'nowrap',
    width: MuiDrawerWidth,
    transition: theme.transitions.create('width', {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.enteringScreen,
    }),
    boxSizing: 'border-box',
    ...(!open && {
      overflowX: 'hidden',
      transition: theme.transitions.create('width', {
        easing: theme.transitions.easing.sharp,
        duration: theme.transitions.duration.leavingScreen,
      }),
      width: theme.spacing(7),
      [theme.breakpoints.up('sm')]: {
        width: theme.spacing(9),
      },
    }),
  },
}))

const App = (): JSX.Element => {
  const [open, setOpen] = React.useState(true)
  const toggleDrawer = () => {
    setOpen(!open)
  }

  return (
    <ThemeProvider theme={mdTheme}>
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
        <Drawer variant="permanent" open={open}>
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
          <List component="nav">{MenuListItems}</List>
        </Drawer>
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
