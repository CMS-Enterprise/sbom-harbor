/**
 * A component for rendering a team as a card in the dashboard with
 * the team's name. It can be clicked to navigate to the team view.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/DashboardTeamCard
 */
import * as React from 'react'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Card from '@mui/material/Card'
import CardContent from '@mui/material/CardContent'
import Typography from '@mui/material/Typography'
import { Team } from '@/types'

const DashboardTeamCard = ({ team }: { team: Team }): JSX.Element =>
  team ? (
    <Card sx={{ position: 'relative' }}>
      <CardContent>
        <Typography variant="h5">
          <Box
            component="span"
            sx={{ color: 'primary.main', fontWeight: 'bold' }}
          >
            {team.name}
          </Box>
        </Typography>
        <Typography variant="h6">
          <>{Object.values(team.projects).length || 0} Projects</>
        </Typography>
        <Typography variant="body2" sx={{ mb: 3.25 }}>
          <>{Object.values(team.members).length || 0} Members</>
        </Typography>
        <Box sx={{ display: 'flex', justifyContent: 'flex-end' }}>
          <Link to={`teams/${team.id}`}>View</Link>
        </Box>
      </CardContent>
    </Card>
  ) : (
    <Card sx={{ position: 'relative' }}>
      <CardContent>
        <Typography variant="h5">
          <Box
            component="span"
            sx={{ color: 'primary.main', fontWeight: 'bold' }}
          >
            -
          </Box>
        </Typography>
        <Typography variant="h6">- Projects</Typography>
        <Typography variant="body2" sx={{ mb: 3.25 }}>
          - Members
        </Typography>
        <Box sx={{ display: 'flex', justifyContent: 'flex-end' }}>
          <Link to={`teams`}>View</Link>
        </Box>
      </CardContent>
    </Card>
  )

export default DashboardTeamCard
