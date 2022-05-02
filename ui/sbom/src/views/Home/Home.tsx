/**
 * Home page component that renders a welcome message and logo.
 * @module @cyclonedx/ui/sbom/views/Home/Home
 */
import * as React from 'react'
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import logo from '@/assets/images/logo.svg'
import classes from './Home.module.css'

const Template = (): JSX.Element => (
  <Container maxWidth="xs" sx={{ mt: 2 }}>
    <Box
      sx={{
        alignItems: 'center',
        display: 'flex',
        flexDirection: 'column',
        marginTop: 8,
      }}
    >
      <p>Welcome to the</p>
      <h1 className={classes.headerTitle}>SBOM Shelter</h1>
      <img src={logo} className={classes.logo} alt="logo" loading="lazy" />
    </Box>
  </Container>
)

export default Template
