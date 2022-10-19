/**
 * A component used in the app header that renders either a login
 * or logout button depending on the user's authentication status.
 * @module @cyclonedx/ui/sbom/components/HeaderAuthButton
 * @exports HeaderAuthButton
 */
import * as React from 'react'
import { Link as RouterLink } from 'react-router-dom'
import { styled } from '@mui/material/styles'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import useAuth from '@/hooks/useAuth'

// ** Styled Components
const ButtonBox = styled(Box)({ mr: 1, ml: 1, pr: 1, pl: 1 })
const ButtonContainerBox = styled(Box)({ mr: 2, ml: 2, pr: 2, pl: 2 })

/**
 * Internal component for a login button to be rendered HeaderAuthButton.
 * @returns {JSX.Element} Button that navigates to login view on click.
 */
const LoginButton = (): JSX.Element => (
  <ButtonBox>
    <Button component={RouterLink} to="/login" color="inherit">
      Login
    </Button>
  </ButtonBox>
)

/**
 * Internal component for a logout button to be rendered by HeaderAuthButton.
 * @returns {JSX.Element} Button that navigates to the logout route on click.
 */
const LogoutButton = (): JSX.Element => {
  return (
    <ButtonBox>
      <Button component={RouterLink} to="/logout" color="inherit">
        Logout
      </Button>
    </ButtonBox>
  )
}

/**
 * A component that conditionally renders a login or a logout button based on
 * the auth state. If auth state is not yet known, renders an empty container.
 * @returns {JSX.Element} Component containing a login or logout button.
 */
const HeaderAuthButton = (): JSX.Element => {
  const { user } = useAuth()

  // AuthContext.initAuth hasn't resolved yet, so don't render anything
  if (user === null) {
    return <ButtonContainerBox />
  }

  // session is defined, so render logout button
  if (user && user.jwtToken) {
    return (
      <ButtonContainerBox>
        <LogoutButton />
      </ButtonContainerBox>
    )
  }

  // session doesn't exist, so render login button
  return (
    <ButtonContainerBox>
      <LoginButton />
    </ButtonContainerBox>
  )
}

export default HeaderAuthButton
