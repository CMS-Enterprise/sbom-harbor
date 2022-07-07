/**
 * @module @cyclonedx/ui/sbom/components/Header
 */
import * as React from 'react'
import Box from '@mui/material/Box'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import Image from '@/components/Image'
import AppBar from '@/components/AppBar'
import AuthButton from '@/components/HeaderAuthButton'
import data from '@/data.json'

const { company, logo } = data

const Header = (): JSX.Element => {
  return (
    <Box>
      <AppBar position="static">
        <Toolbar>
          <Box sx={{ mr: 1 }}>
            <Image alt={company} src={logo} sx={{ height: 40, width: 40 }} />
          </Box>
          <Typography component="div" sx={{ flexGrow: 1 }} variant="h6">
            SBOM Shelter
          </Typography>
          <AuthButton />
        </Toolbar>
      </AppBar>
    </Box>
  )
}
export default Header
