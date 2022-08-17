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
import TeamMembersTable, { TableBodyRowType } from './TeamMembersTable'
import { Project } from '@/utils/types'
import TeamViewProjectCard from './TeamViewProjectCard'

const TeamView = () => {
  // ** Hooks
  const {
    data: { teams },
  } = useData()
  const { teamId } = useParams()

  // ** Current Team
  const team = (teams || []).find(({ Id }) => Id === teamId)
  const members = (team?.members || []) as TableBodyRowType[]
  const projects = (team?.projects || []) as Project[]

  return (
    <>
      {team && (
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
                  <TeamMembersTable members={members} />
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
                {projects &&
                  projects.map((project, index) => (
                    <Grid item xs={12} md={12} key={`project-${index}`}>
                      <TeamViewProjectCard project={project} />
                    </Grid>
                  ))}
              </Grid>
            </Box>
          </Paper>
        </Container>
      )}
    </>
  )
}

export default TeamView
