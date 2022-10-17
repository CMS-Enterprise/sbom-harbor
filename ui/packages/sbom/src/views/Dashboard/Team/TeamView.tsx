/**
 * A component that renders a team.
 * URL: /team/:teamId - @see {@link @cyclonedx/ui/sbom/Routes}.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamView
 */
import * as React from 'react'
import { Link, useParams } from 'react-router-dom'
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import Paper from '@mui/material/Paper'
import Typography from '@mui/material/Typography'
import { useData } from '@/hooks/useData'
import TeamMembersTable from './components/TeamMembersTable'
import TeamViewProjectCard from './components/TeamViewProjectCard'

const TeamView = () => {
  // get teams from data context
  const { data: { teams = [] } = {} } = useData()

  // get teamId from URL params
  const { teamId } = useParams()

  // find the team to edit from the data context and set it in local state.
  const [team] = React.useState(() => {
    const team = teams.find((t) => t.Id === teamId)
    if (!team) {
      // TODO: handle error where team isn't found with error boundary.
      throw new Error(`Team with ID ${teamId} not found.`)
    }
    return team
  })

  return (
    <Container component="main" maxWidth="md" data-testid="team">
      <Paper
        variant="outlined"
        sx={{ mt: { xs: 3, md: 6 }, p: { xs: 2, md: 3 } }}
      >
        <Box
          sx={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
          }}
        >
          <Typography variant="h4" sx={{ mt: 1, mb: 3 }}>
            {team?.Id}
          </Typography>
          <Link to={`edit`}>Edit Team</Link>
        </Box>
        <Box sx={{ mb: 6 }}>
          <Grid container spacing={3}>
            <Grid item>
              <Typography variant="h5" sx={{ mt: 2, mb: 0 }}>
                Members
              </Typography>
            </Grid>
            <Grid item xs={12} md={12}>
              <TeamMembersTable members={team.members} />
            </Grid>
          </Grid>
        </Box>
        <Box>
          <Grid container spacing={3} sx={{ mb: 6 }}>
            <Grid item>
              <Typography variant="h5" sx={{ mt: 2, mb: 0 }}>
                Projects
              </Typography>
            </Grid>
            {team.projects &&
              team.projects.map((project, index) => (
                <Grid item xs={12} md={12} key={`project-${index}`}>
                  <TeamViewProjectCard project={project} />
                </Grid>
              ))}
          </Grid>
        </Box>
      </Paper>
    </Container>
  )
}

export default TeamView
