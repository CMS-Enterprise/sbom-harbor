/**
 * The view at the /login route that renders the sign in form.
 * It's the next version of {@link @cyclonedx/views/SignIn/SignIn}.
 */
import * as React from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { useForm, Controller } from 'react-hook-form'
import * as yup from 'yup'
import { yupResolver } from '@hookform/resolvers/yup'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Checkbox from '@mui/material/Checkbox'
import FormControl from '@mui/material/FormControl'
import FormHelperText from '@mui/material/FormHelperText'
import IconButton from '@mui/material/IconButton'
import InputAdornment from '@mui/material/InputAdornment'
import InputLabel from '@mui/material/InputLabel'
import MuiLink from '@mui/material/Link'
import OutlinedInput from '@mui/material/OutlinedInput'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import useMediaQuery from '@mui/material/useMediaQuery'
import { useTheme } from '@mui/material/styles'
import EyeOutline from 'mdi-material-ui/EyeOutline'
import EyeOffOutline from 'mdi-material-ui/EyeOffOutline'
import loginUser from '@/actions/loginUser'
import LinearIndeterminate from '@/components/mui/LinearLoadingBar'
import { useAuthDispatch } from '@/hooks/useAuth'
import useAlert from '@/hooks/useAlert'
import BlankLayout from '@/layouts/BlankLayout'
import {
  BoxWrapper,
  FormControlLabel,
  LoginIllustrationWrapper,
  RightWrapper,
  TypographyStyled,
} from '@/views/SignIn/SignIn.components'
import SignInGraphic from '@/views/SignIn/SignInGraphic'

const defaultValues = {
  email: '',
  password: '',
}

const schema = yup.object().shape({
  email: yup.string().email().required(),
  password: yup.string().min(5).required(),
})

interface FormData {
  email: string
  password: string
}

/**
 * Component that renders the page containing the sign in form.
 * @returns {JSX.Element} component that renders the the sign in form.
 */
const LoginPage = () => {
  // theming hooks
  const theme = useTheme()
  const hidden = useMediaQuery(theme.breakpoints.down('md'))

  const navigate = useNavigate()
  const { setAlert } = useAlert()
  const dispatch = useAuthDispatch()

  // local state
  const [loading, setLoading] = React.useState<boolean>(false)
  const [showPassword, setShowPassword] = React.useState<boolean>(false)

  const {
    control,
    handleSubmit,
    formState: { errors },
  } = useForm({
    defaultValues,
    mode: 'onBlur',
    resolver: yupResolver(schema),
  })

  const onSubmit = async (data: FormData) => {
    const { email, password } = data
    setLoading(true)
    try {
      await loginUser(dispatch, { email, password })
      setLoading(false)
      navigate('/app')
    } catch (error) {
      setLoading(false)
      setAlert({
        message: 'There was an error logging in. Please try again.',
        severity: 'error',
      })
    }
  }

  return (
    <Box className="content-right">
      {!hidden ? (
        <Box
          sx={{
            flex: 1,
            display: 'flex',
            position: 'relative',
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <LoginIllustrationWrapper>
            {/* TODO: add graphics for the login page */}
          </LoginIllustrationWrapper>
        </Box>
      ) : null}
      <RightWrapper
        sx={!hidden ? { borderLeft: `1px solid ${theme.palette.divider}` } : {}}
      >
        <Box
          sx={{
            p: 7,
            height: '100%',
            display: 'flex',
            flexFlow: 'column',
            alignItems: 'center',
            justifyContent: 'center',
            backgroundColor: 'background.paper',
          }}
        >
          <BoxWrapper>
            <Box
              sx={{
                top: 30,
                left: 40,
                display: 'flex',
                position: 'absolute',
                alignItems: 'center',
                justifyContent: 'center',
              }}
            >
              <SignInGraphic />
              <Typography
                variant="h6"
                sx={{
                  ml: 2,
                  lineHeight: 1,
                  fontWeight: 700,
                  fontSize: '1.5rem !important',
                }}
              >
                SBOM Harbor
              </Typography>
            </Box>
            <Box sx={{ mb: 6 }}>
              <TypographyStyled variant="h5">{`Welcome to the Harbor! 👋🏻`}</TypographyStyled>
              <Typography variant="body2">
                Please sign-in to your account.
              </Typography>
            </Box>
            <form
              noValidate
              autoComplete="off"
              onSubmit={handleSubmit(onSubmit)}
            >
              <FormControl fullWidth sx={{ mb: 4 }}>
                <Controller
                  name="email"
                  control={control}
                  rules={{ required: true }}
                  render={({ field: { value, onChange, onBlur } }) => (
                    <TextField
                      autoFocus
                      label="Email"
                      value={value}
                      onBlur={onBlur}
                      onChange={onChange}
                      error={Boolean(errors.email)}
                      placeholder="admin@materialize.com"
                    />
                  )}
                />
                {errors.email && (
                  <FormHelperText sx={{ color: 'error.main' }}>
                    {errors.email.message}
                  </FormHelperText>
                )}
              </FormControl>
              <FormControl fullWidth>
                <InputLabel
                  htmlFor="auth-login-v2-password"
                  error={Boolean(errors.password)}
                >
                  Password
                </InputLabel>
                <Controller
                  name="password"
                  control={control}
                  rules={{ required: true }}
                  render={({ field: { value, onChange, onBlur } }) => (
                    <OutlinedInput
                      value={value}
                      onBlur={onBlur}
                      label="Password"
                      onChange={onChange}
                      id="auth-login-v2-password"
                      error={Boolean(errors.password)}
                      type={showPassword ? 'text' : 'password'}
                      endAdornment={
                        <InputAdornment position="end">
                          <IconButton
                            edge="end"
                            onMouseDown={(e) => e.preventDefault()}
                            onClick={() => setShowPassword(!showPassword)}
                          >
                            {showPassword ? <EyeOutline /> : <EyeOffOutline />}
                          </IconButton>
                        </InputAdornment>
                      }
                    />
                  )}
                />
                {errors.password && (
                  <FormHelperText sx={{ color: 'error.main' }} id="">
                    {errors.password.message}
                  </FormHelperText>
                )}
              </FormControl>
              <Box
                sx={{
                  mb: 4,
                  display: 'flex',
                  alignItems: 'center',
                  flexWrap: 'wrap',
                  justifyContent: 'space-between',
                }}
              >
                <FormControlLabel
                  label="Remember Me"
                  control={<Checkbox />}
                  sx={{
                    '& .MuiFormControlLabel-label': { color: 'text.primary' },
                  }}
                />
                <MuiLink
                  component={Link}
                  to="/login"
                  variant="body2"
                  sx={{ color: 'primary.main' }}
                >
                  Forgot Password?
                </MuiLink>
              </Box>
              <Button
                fullWidth
                size="large"
                type="submit"
                variant="contained"
                disabled={loading}
                sx={{ mb: 5 }}
              >
                Login
              </Button>
            </form>
            {loading && <LinearIndeterminate />}
          </BoxWrapper>
        </Box>
      </RightWrapper>
    </Box>
  )
}

LoginPage.getLayout = (page: React.ReactNode) => (
  <BlankLayout>{page}</BlankLayout>
)

LoginPage.guestGuard = true

export default LoginPage
