/**
 * Sticky footer component.
 * @module @cyclonedx/ui/sbom/components/Footer
 */
import * as React from 'react'
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Copyright from '@/components/Copyright'

const StickyFooter = (): JSX.Element => (
  <Box
    component="footer"
    sx={{
      px: 2,
      py: 3,
      mt: 'auto',
      backgroundColor: (theme) =>
        theme.palette.mode === 'light'
          ? theme.palette.grey[200]
          : theme.palette.grey[800],
    }}
  >
    <Container maxWidth="sm">
      <Copyright />
    </Container>
  </Box>
)

export default StickyFooter
