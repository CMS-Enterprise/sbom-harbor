import * as React from 'react'
import { Link as RouterLink } from 'react-router-dom'
import DashboardIcon from '@mui/icons-material/Dashboard'
import Link from '@mui/material/Link'
import ListItemButton from '@mui/material/ListItemButton'
import ListItemIcon from '@mui/material/ListItemIcon'
import ListItemText from '@mui/material/ListItemText'

const MenuLink = ({ title, to }: { title: string; to: string }) => (
  <Link to={to} component={RouterLink}>
    <ListItemButton>
      <ListItemIcon>
        <DashboardIcon />
      </ListItemIcon>
      <ListItemText primary={title} />
    </ListItemButton>
  </Link>
)

const MenuListItems = (
  <React.Fragment>
    {/* Dashboard */}
    <MenuLink title="Dashboard" to="" />
    {/* Team */}
    <MenuLink title="Team" to="team" />
  </React.Fragment>
)

export default MenuListItems
