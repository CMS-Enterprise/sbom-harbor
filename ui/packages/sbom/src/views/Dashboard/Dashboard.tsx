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
import DashboardTeamCard from './Team/components/DashboardTeamCard'
import DashboardTeamCreationCard from './Team/components/DashboardTeamCreateCard'
import useAuth from '@/hooks/useAuth'

const DashboardContainer = (): JSX.Element => {
  const { user } = useAuth()
  const {
    data: { teams = {} },
    fetchTeams,
  } = useData()

  const navigate = useNavigate()

  /** Helper function that redirects the user to the new team creation view. */
  const navigateToCreateTeam = () => navigate('team/new')

  React.useEffect(() => {
    if (user?.jwtToken) {
      const controller = new AbortController()
      fetchTeams(controller)
      return () => controller.abort()
    }
  }, [user?.jwtToken])

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
            {Object.entries(teams).map(([key, team]) => (
              <Grid item xs={12} md={4} key={key}>
                <DashboardTeamCard teamId={key} />
              </Grid>
            ))}
          </Grid>
        </Container>
      </Box>
    </Box>
  )
}
export default DashboardContainer
