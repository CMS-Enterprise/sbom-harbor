/**
 * @module @cyclonedx/ui/sbom/views/Dashboard/Dashboard
 */
import * as React from 'react'
import { styled } from '@mui/material/styles'
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import Paper from '@mui/material/Paper'
import { MuiDrawerWidth } from '@/utils/constants'

const Section = styled(Paper)(({ theme }) => ({
  padding: `${theme.spacing(1)} ${theme.spacing(2)}`,
  display: 'flex',
  flexDirection: 'column',
  height: MuiDrawerWidth,
  backgroundColor:
    theme.palette.mode === 'light'
      ? theme.palette.grey[100]
      : theme.palette.grey[800],
}))

function DashboardContent() {
  return (
    <Box sx={{ display: 'flex' }} data-testid="Dashboard">
      <Box
        sx={{
          flexGrow: 1,
          height: '100vh',
          overflow: 'auto',
        }}
      >
        <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
          <Grid container spacing={3}>
            {/* Team */}
            <Grid item xs={12} md={12} lg={7}>
              <Section>
                <h1>Team</h1>
              </Section>
            </Grid>

            {/* API Tokens */}
            <Grid item xs={12} md={12} lg={5}>
              <Section>
                <h1>API Tokens</h1>
              </Section>
            </Grid>
          </Grid>
        </Container>
      </Box>
    </Box>
  )
}

export default function Dashboard() {
  return <DashboardContent />
}
