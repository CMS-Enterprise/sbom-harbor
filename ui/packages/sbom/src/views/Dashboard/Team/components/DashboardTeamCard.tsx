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
import { useData } from '@/hooks/useData'

type InputProps = {
  teamId: string
}

const DashboardTeamCard = ({ teamId }: InputProps): JSX.Element => {
  const { data: { teams: { [teamId]: team } = {} } = {} } = useData()

  if (team) {
    const { name = '', projects = {}, members = {} } = team
    return (
      <Card sx={{ position: 'relative' }}>
        <CardContent>
          <Typography variant="h5">
            <Box
              component="span"
              sx={{ color: 'primary.main', fontWeight: 'bold' }}
            >
              {name}
            </Box>
          </Typography>
          <Typography variant="h6">
            <>{Object.keys(projects).length || 0} Projects</>
          </Typography>
          <Typography variant="body2" sx={{ mb: 3.25 }}>
            <>{Object.keys(members).length || 0} Members</>
          </Typography>
          <Box sx={{ display: 'flex', justifyContent: 'flex-end' }}>
            <Link to={`teams/${teamId}`}>View</Link>
          </Box>
        </CardContent>
      </Card>
    )
  }

  return (
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
}

export default DashboardTeamCard
