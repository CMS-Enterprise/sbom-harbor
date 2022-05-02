/**
 * @module @cyclonedx/ui/sbom/components/Header
 */
import * as React from 'react'
import { Link as RouterLink } from 'react-router-dom'
import AppBar from '@mui/material/AppBar'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import Image from '@/components/Image'
import data from '@/data.json'

const { company, logo } = data

const Header = (): JSX.Element => (
  <Box>
    <AppBar position="static">
      <Toolbar>
        <Box sx={{ mr: 1 }}>
          <Button component={RouterLink} to="/login">
            <Image alt={company} src={logo} sx={{ height: 40, width: 40 }} />
          </Button>
        </Box>
        <Typography component="div" sx={{ flexGrow: 1 }} variant="h6">
          SBOM Shelter
        </Typography>
        <Button component={RouterLink} to="/login" color="inherit">
          Login
        </Button>
      </Toolbar>
    </AppBar>
  </Box>
)

export default Header
