/**
 * A component that renders a blank state team card in the dashboard
 * with animations where the team's name, projects, and members will be.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/DashboardTeamLoadingCard
 */
import * as React from 'react'
import Box from '@mui/material/Box'
import Card from '@mui/material/Card'
import CardContent from '@mui/material/CardContent'
import Typography from '@mui/material/Typography'

const DashboardTeamLoadingCard = (): JSX.Element => (
  <Card sx={{ position: 'relative' }}>
    <CardContent>
      <Typography variant="h5">
        <Box
          component="span"
          sx={{ color: 'primary.main', fontWeight: 'bold' }}
        >
          - Team Name
        </Box>
      </Typography>
      <Typography variant="h6">- Projects</Typography>
      <Typography variant="body2" sx={{ mb: 3.25 }}>
        - Members
      </Typography>
    </CardContent>
  </Card>
)

export default DashboardTeamLoadingCard
