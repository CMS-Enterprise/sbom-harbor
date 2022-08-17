/**
 * Styled Components for the SignIn view component.
 * @module @cyclonedx/ui/sbom/views/SignIn/SignIn.components
 */
import { styled } from '@mui/material/styles'
import Box, { BoxProps } from '@mui/material/Box'
import MuiFormControlLabel, {
  FormControlLabelProps,
} from '@mui/material/FormControlLabel'
import Typography, { TypographyProps } from '@mui/material/Typography'

export const LoginIllustrationWrapper = styled(Box)<BoxProps>(({ theme }) => ({
  padding: theme.spacing(10),
  paddingRight: '0 !important',
  [theme.breakpoints.down('lg')]: {
    padding: theme.spacing(10),
  },
}))

export const RightWrapper = styled(Box)<BoxProps>(({ theme }) => ({
  width: '100%',
  marginLeft: 'auto',
  marginRight: theme.spacing(16),
  [theme.breakpoints.up('md')]: {
    maxWidth: 400,
  },
  [theme.breakpoints.up('lg')]: {
    maxWidth: 450,
  },
}))

export const BoxWrapper = styled(Box)<BoxProps>(({ theme }) => ({
  width: '100%',
  [theme.breakpoints.down('md')]: {
    maxWidth: 400,
  },
}))

export const TypographyStyled = styled(Typography)<TypographyProps>(
  ({ theme }) => ({
    fontWeight: 600,
    letterSpacing: '0.18px',
    marginBottom: theme.spacing(1.5),
    [theme.breakpoints.down('md')]: { marginTop: theme.spacing(8) },
  })
)

export const FormControlLabel = styled(
  MuiFormControlLabel
)<FormControlLabelProps>(({ theme }) => ({
  '& .MuiFormControlLabel-label': {
    fontSize: '0.875rem',
    color: theme.palette.text.secondary,
  },
}))
