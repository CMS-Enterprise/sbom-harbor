/**
 * @module @cyclonedx/ui/sbom/components/AppBar
 */
import { styled } from '@mui/material/styles'
import MuiAppBar, { AppBarProps as MuiAppBarProps } from '@mui/material/AppBar'
import { MuiDrawerWidth } from '@/utils/constants'

interface AppBarProps extends MuiAppBarProps {
  open?: boolean
}

const AppBar = styled(MuiAppBar, {
  shouldForwardProp: (prop) => prop !== 'open',
})<AppBarProps>(({ theme, open }) => ({
  zIndex: theme.zIndex.drawer + 1,
  transition: theme.transitions.create(['width', 'margin'], {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.leavingScreen,
  }),
  ...(open && {
    marginLeft: MuiDrawerWidth,
    width: `calc(100% - ${MuiDrawerWidth}px)`,
    transition: theme.transitions.create(['width', 'margin'], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.enteringScreen,
    }),
  }),
}))

export default AppBar
