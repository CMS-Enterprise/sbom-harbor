import { styled } from '@mui/material/styles'
import MuiDrawer from '@mui/material/Drawer'
import { MuiDrawerWidth } from '@/utils/theme'

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
  '& .MuiLink-root': {
    textDecoration: 'none',
  },
  '& .MuiListItemIcon-root': {
    marginRight: theme.spacing(1.5),
    minWidth: 'auto',
  },
}))

export default Drawer
