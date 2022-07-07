/**
 * Material UI theme object at the root of the application.
 * @module @cyclonedx/ui/sbom/theme
 * @see {@link @cyclonedx/ui/sbom/index} for where this is imported.
 * @see https://material-ui.com/customization/themes/ for documentation.
 */
import { red } from '@mui/material/colors'
import { createTheme } from '@mui/material/styles'

export const MuiDrawerWidth = 200

const theme = createTheme({
  palette: {
    mode: 'light',
    primary: {
      main: '#556cd6',
    },
    secondary: {
      main: '#19857b',
    },
    error: {
      main: red.A400,
    },
    text: {
      primary: 'rgba(0, 0, 0, 0.87)',
      secondary: 'rgba(0, 0, 0, 0.6)',
      disabled: 'rgba(0, 0, 0, 0.38)',
    },
  },
})

export default theme
