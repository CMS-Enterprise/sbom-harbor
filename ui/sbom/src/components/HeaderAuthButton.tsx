/**
 * @module @cyclonedx/ui/sbom/components/Header
 */
import * as React from 'react'
import { Link as RouterLink } from 'react-router-dom'
import { styled } from '@mui/material/styles'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import { AuthContext } from '@/providers/AuthContext'

const ButtonBox = styled(Box)({
  ml: 1,
  mr: 1,
  pl: 1,
  pr: 1,
})

const LoginButton = (): JSX.Element => (
  <ButtonBox>
    <Button component={RouterLink} to="/login" color="inherit">
      Login
    </Button>
  </ButtonBox>
)

const LogoutButton = (): JSX.Element => {
  return (
    <ButtonBox>
      <Button component={RouterLink} to="/logout" color="inherit">
        Logout
      </Button>
    </ButtonBox>
  )
}

const sx = { ml: 2, mr: 2, pl: 2, pr: 2 }

const AuthButton = (): JSX.Element => {
  const { user } = React.useContext(AuthContext)

  // session doesn't exist, so render login button
  if (user === null) {
    return (
      <Box sx={sx}>
        <LoginButton />
      </Box>
    )
  }

  // session is defined, so render logout button
  if (user !== null && typeof user !== 'undefined') {
    return (
      <Box sx={sx}>
        <LogoutButton />
      </Box>
    )
  }

  // getSession hasn't resolved yet, so don't render anything
  return <Box sx={sx} />
}

export default AuthButton
