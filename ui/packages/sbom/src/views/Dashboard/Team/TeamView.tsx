/**
 * A component that renders a team.
 * URL: /team/:teamId - @see {@link @cyclonedx/ui/sbom/Routes}.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamView
 */
import * as React from 'react'
import { Link, useLoaderData } from 'react-router-dom'
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import Paper from '@mui/material/Paper'
import Typography from '@mui/material/Typography'
import TeamMembersTable from './components/TeamMembersTable'
import TeamViewProjectCard from './components/TeamViewProjectCard'
import { Team, UserTableRowType } from '@/types'

const TeamView = () => {
  const team = useLoaderData() as Team
  const [members, setMembers] = React.useState<UserTableRowType[]>([])

  React.useEffect(() => {
    if (!team?.members) {
      return
    }

    const newMembers = Object.entries(team.members).map(
      ([id, member]): UserTableRowType => {
        const { email = '', isTeamLead = false } = member
        return {
          id,
          email,
          isTeamLead,
          role: isTeamLead ? 'admin' : 'member',
          avatarSrc: `https://ui-avatars.com/api/?name=${email}&background=0D8ABC&color=fff`,
        }
      }
    )

    setMembers(newMembers)
  }, [team?.members])

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
            {team?.name}
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
            {team?.projects &&
              Object.entries(team.projects).map(([key, project]) => (
                <Grid item xs={12} md={12} key={key}>
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
