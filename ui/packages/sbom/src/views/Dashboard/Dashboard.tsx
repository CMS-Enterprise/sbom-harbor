/**
 * The default view that an authenticated user first sees when they visit the
 *  app. It renders a list of teams that the user is a member of, and a list
 *  of the api keys that the user has access to.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Dashboard
 */
import * as React from 'react'
import { useNavigate } from 'react-router-dom'
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import { useData } from '@/hooks/useData'
import DashboardTeamCard from '@/views/Dashboard/Team/DashboardTeamCard'
import DashboardTeamCreationCard from '@/views/Dashboard/Team/DashboardTeamCreateCard'

const DashboardContainer = (): JSX.Element => {
  const {
    data: { teams },
  } = useData()
  const navigate = useNavigate()
  const navigateToCreateTeam = () => navigate('team/new')

  return (
    <Box sx={{ display: 'flex' }} data-testid="Dashboard">
      <Box
        sx={{
          flexGrow: 1,
          height: 'auto',
          overflow: 'auto',
        }}
      >
        <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
          <Grid container spacing={6} className="match-height">
            <Grid item xs={12} md={4}>
              <DashboardTeamCreationCard onClick={navigateToCreateTeam} />
            </Grid>
            {teams &&
              teams.map((team) => (
                <Grid item xs={12} md={4} key={team.Id}>
                  <DashboardTeamCard team={team} />
                </Grid>
              ))}
          </Grid>
        </Container>
      </Box>
    </Box>
  )
}
export default DashboardContainer
