/**
 * The default view that an authenticated user first sees when they visit the
 *  app. It renders a list of teams that the user is a member of, and a list
 *  of the api keys that the user has access to.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Dashboard
 */
import * as React from 'react'
import { useNavigate, useLoaderData } from 'react-router-dom'
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import DashboardTeamCard from './Team/components/DashboardTeamCard'
import DashboardTeamCreationCard from './Team/components/DashboardTeamCreateCard'
import { Team } from '@/types'

const DashboardContainer = (): JSX.Element => {
  // navigation hook
  const navigate = useNavigate()

  // hook for getting the route loader data
  const teams = useLoaderData() as Team[]

  // Callback function that redirects the user to the new team creation
  // view. This callback does not depend on `navigate`, so it's not
  // necessary to include it in the `useCallback` dependency array.
  /* eslint-disable react-hooks/exhaustive-deps */
  const navigateToCreateTeam = React.useCallback(() => {
    navigate('team/new')
  }, [])
  /* eslint-enable react-hooks/exhaustive-deps */

  return (
    <Box sx={{ display: 'flex' }} data-testid="Dashboard">
      <Box
        sx={{
          flexGrow: 1,
          height: 'auto',
          overflow: 'scroll',
        }}
      >
        <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
          <Grid container spacing={6} className="match-height">
            <Grid item xs={12} md={4}>
              <DashboardTeamCreationCard onClick={navigateToCreateTeam} />
            </Grid>
            {teams?.map((team: Team) => (
              <Grid item xs={12} md={4} key={team.id}>
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
