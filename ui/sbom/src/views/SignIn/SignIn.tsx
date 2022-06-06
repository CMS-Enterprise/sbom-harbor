/**
 * Sign in page component that renders a login form.
 * @module @cyclonedx/ui/sbom/views/SignIn/SignIn
 */
import * as React from 'react'
import { Link as RouterLink, useNavigate } from 'react-router-dom'
import Avatar from '@mui/material/Avatar'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Checkbox from '@mui/material/Checkbox'
import Container from '@mui/material/Container'
import FormControlLabel from '@mui/material/FormControlLabel'
import Grid from '@mui/material/Grid'
import Link from '@mui/material/Link'
import LockOutlinedIcon from '@mui/icons-material/LockOutlined'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import { SessionContext } from '@/services/auth'
import { Auth } from '@aws-amplify/auth'

type State = {
  email?: string
  password?: string
}

const defaultState = {
  password: '',
  email: '',
} as State

const SignIn = (): JSX.Element => {
  const navigate = useNavigate()

  const { setUser } = React.useContext(SessionContext)

  const [formInput, setFormInput] = React.useReducer(
    (state: State, newState: State) => ({ ...state, ...newState }),
    defaultState
  )

  const handleInput = (
    evt: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const name = evt.currentTarget.name
    const newValue = evt.currentTarget.value
    setFormInput({ [name]: newValue })
  }

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    try {
      if (!formInput.email || !formInput.password) {
        throw new Error('Email and password are required')
      }
      const user = await Auth.signIn(formInput.email, formInput.password)
      setUser(user)
      navigate('/app')
    } catch (error) {
      console.log(error)
    }
  }

  return (
    <Container maxWidth="xs" sx={{ mt: 2 }}>
      <Box
        sx={{
          marginTop: 8,
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
        }}
      >
        <Avatar sx={{ m: 1, bgcolor: 'secondary.main' }}>
          <LockOutlinedIcon />
        </Avatar>
        <Typography component="h1" variant="h5">
          Sign in
        </Typography>
        <Box
          data-testid="sign-in-form"
          component="form"
          noValidate
          onSubmit={handleSubmit}
          sx={{
            mt: 1,
            display: 'flex',
            flexFlow: 'column',
            width: '100%',
          }}
        >
          <TextField
            data-testid="sign-in-form__email"
            autoComplete="email"
            autoFocus
            fullWidth
            id="email"
            label="Email Address"
            margin="normal"
            name="email"
            required
            value={formInput.email}
            onChange={handleInput}
          />
          <TextField
            data-testid="sign-in-form__password"
            autoComplete="current-password"
            fullWidth
            id="password"
            label="Password"
            margin="normal"
            name="password"
            type="password"
            required
            value={formInput.password}
            onChange={handleInput}
          />
          <FormControlLabel
            data-testid="sign-in-form__remember-me"
            control={<Checkbox value="remember" color="primary" />}
            label="Remember me"
          />
          <Button
            data-testid="sign-in-form__submit"
            type="submit"
            fullWidth
            variant="contained"
            sx={{ mt: 3, mb: 2 }}
          >
            Sign In
          </Button>
          <Grid container>
            <Grid item xs display="inline-flex">
              <Link href="#" variant="body2">
                Forgot password?
              </Link>
            </Grid>
            <Grid item display="inline-flex">
              <Typography variant="body2" color="text.disabled">
                {"Don't have an account?"}
              </Typography>
              &nbsp;
              <Link component={RouterLink} to="/join" variant="body2">
                Sign Up
              </Link>
            </Grid>
          </Grid>
        </Box>
      </Box>
    </Container>
  )
}

export default SignIn
