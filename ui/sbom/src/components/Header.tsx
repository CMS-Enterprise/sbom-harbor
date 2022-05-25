/**
 * @module @cyclonedx/ui/sbom/components/Header
 */
import * as React from 'react'
import { Link as RouterLink } from 'react-router-dom'
import { CognitoUserSession } from 'amazon-cognito-identity-js'
import AppBar from '@mui/material/AppBar'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import Image from '@/components/Image'
import data from '@/data.json'
import { logout } from '@/services/auth'

const { company, logo } = data

type HeaderProps = {
  session?: CognitoUserSession | null
}

const LoginButton = (): JSX.Element => (
  <Button component={RouterLink} to="/login" color="inherit">
    Login
  </Button>
)

const LogoutButton = (): JSX.Element => (
  <Button component={RouterLink} to="/login" color="inherit" onClick={logout}>
    Logout
  </Button>
)

const AuthButton = ({ session }: HeaderProps): JSX.Element => {
  // session doesn't exist, so render login button
  if (session === null) return <LoginButton />
  // session is defined, so render logout button
  if (typeof session !== 'undefined') return <LogoutButton />
  // getSession hasn't resolved yet, so don't render anything
  return <></>
}

const Header = ({ session }: HeaderProps): JSX.Element => (
  <Box>
    <AppBar position="static">
      <Toolbar>
        <Box sx={{ mr: 1 }}>
          <Button component={RouterLink} to="/login">
            <Image alt={company} src={logo} sx={{ height: 40, width: 40 }} />
          </Button>
        </Box>
        <Typography component="div" sx={{ flexGrow: 1 }} variant="h6">
          SBOM Shelter
        </Typography>
        <AuthButton session={session} />
      </Toolbar>
    </AppBar>
  </Box>
)

export default Header
