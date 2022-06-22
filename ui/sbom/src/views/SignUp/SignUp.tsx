/**
 * Sign Up page component that renders an account sign up form.
 * @module @cyclonedx/ui/sbom/views/SignUp/SignUp
 */
import * as React from 'react'
import { Link as RouterLink } from 'react-router-dom'
import Avatar from '@mui/material/Avatar'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Checkbox from '@mui/material/Checkbox'
import Container from '@mui/material/Container'
import FormControlLabel from '@mui/material/FormControlLabel'
import Grid from '@mui/material/Grid'
import Link from '@mui/material/Link'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import { ReactComponent as LockOutlinedIcon } from '@/assets/icons/LockOutlined.svg'

const LegalDocsLabel = (): JSX.Element => (
  <Typography variant="body2" color="textSecondary" align="center">
    {'I agree to the '}
    <Link href="#">Terms of Service</Link>
    {' and '}
    <Link href="#">Privacy Policy</Link>
    {'.'}
  </Typography>
)

const SignUp = (): JSX.Element => {
  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    const data = new FormData(event.currentTarget)
    // TODO: Send data to server
    console.log({
      email: data.get('email'),
      password: data.get('password'),
    })
  }

  return (
    <Container maxWidth="xs" sx={{ mt: 2 }}>
      <Box
        sx={{
          alignItems: 'center',
          display: 'flex',
          flexDirection: 'column',
          marginTop: 8,
        }}
      >
        <Avatar sx={{ m: 1, bgcolor: 'secondary.main' }}>
          <LockOutlinedIcon />
        </Avatar>
        <Typography component="h1" variant="h5">
          Sign up
        </Typography>
        <Box
          data-testid="sign-up-form"
          component="form"
          noValidate
          onSubmit={handleSubmit}
          sx={{ mt: 3 }}
        >
          <Grid container spacing={2}>
            <Grid item xs={12} sm={6}>
              <TextField
                data-testid="sign-up-form__email"
                autoComplete="given-name"
                autoFocus
                fullWidth
                id="firstName"
                label="First Name"
                name="firstName"
                required
              />
            </Grid>
            <Grid item xs={12} sm={6}>
              <TextField
                data-testid="sign-up-form__last-name"
                autoComplete="family-name"
                fullWidth
                id="lastName"
                label="Last Name"
                name="lastName"
                required
              />
            </Grid>
            <Grid item xs={12}>
              <TextField
                autoComplete="email"
                fullWidth
                id="email"
                label="Email Address"
                name="email"
                required
              />
            </Grid>
            <Grid item xs={12}>
              <TextField
                data-testid="sign-up-form__password"
                autoComplete="new-password"
                fullWidth
                id="password"
                label="Password"
                name="password"
                type="password"
                required
              />
            </Grid>
            <Grid item xs={12}>
              <FormControlLabel
                data-testid="sign-up-form__remember-me"
                control={<Checkbox value="allowExtraEmails" color="primary" />}
                label={<LegalDocsLabel />}
              />
            </Grid>
          </Grid>
          <Button
            data-testid="sign-up-form__submit"
            type="submit"
            fullWidth
            variant="contained"
            sx={{ mt: 3, mb: 2 }}
          >
            Sign Up
          </Button>
          <Grid container justifyContent="flex-end">
            <Grid item display="inline-flex">
              <Typography variant="body2" color="text.disabled">
                Already have an account?
              </Typography>
              &nbsp;
              <Link component={RouterLink} to="/login" variant="body2">
                Sign In
              </Link>
            </Grid>
          </Grid>
        </Box>
      </Box>
    </Container>
  )
}

export default SignUp
