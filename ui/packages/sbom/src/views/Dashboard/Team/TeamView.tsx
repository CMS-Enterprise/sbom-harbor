/**
 * A component that renders a team.
 * URL: /team/:teamId - @see {@link @cyclonedx/ui/sbom/Routes}.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamView
 */
import * as React from 'react'
import { Await, Link, useLoaderData } from 'react-router-dom'
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import Paper from '@mui/material/Paper'
import Typography from '@mui/material/Typography'
import TeamMembersTable from './components/TeamMembersTable'
import TeamViewProjectCard from './components/TeamViewProjectCard'
import TokensTable from './components/TokensTable'
import { Project, Team, TeamMemberRole } from '@/types'

const Fallback = (): JSX.Element => <Container>Loading...</Container>

/**
 * The view that renders a team at the path `/team/:teamId`.
 * @see {@link @cyclonedx/ui/sbom/router}
 * @returns {JSX.Element} A component that renders a team.
 */
const TeamView = (): JSX.Element => {
  // @ts-ignore
  const { data } = useLoaderData()

  return (
    <Container component="main" maxWidth="md" data-testid="team">
      <Paper
        variant="outlined"
        sx={{ mt: { xs: 3, md: 6 }, p: { xs: 2, md: 3 } }}
      >
        <React.Suspense fallback={<Fallback />}>
          <Await
            resolve={data}
            errorElement={<div>Could not load teams 😬</div>}
            // eslint-disable-next-line react/no-children-prop
            children={(resolvedTeamData) => {
              const {
                id = '',
                name = '',
                projects = {},
                tokens = {},
                membersTableRows = [],
              } = resolvedTeamData as Team & {
                membersTableRows: {
                  id: string
                  email: string
                  isTeamLead: boolean
                  role: TeamMemberRole
                  username: string
                }[]
              }

              return (
                <>
                  <Box
                    sx={{
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center',
                    }}
                  >
                    <Typography variant="h4" sx={{ mt: 1, mb: 3 }}>
                      {name}
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
                        <TeamMembersTable members={membersTableRows} />
                      </Grid>
                    </Grid>
                  </Box>
                  <Box>
                    <Grid container spacing={3} sx={{ mb: 6 }}>
                      <Grid item>
                        <Typography variant="h5" sx={{ mt: 2, mb: 0 }}>
                          Tokens
                        </Typography>
                      </Grid>
                      <Grid item xs={12} md={12}>
                        <TokensTable
                          tokens={Object.values(tokens)}
                          teamId={id}
                        />
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
                        Object.values(projects as Record<string, Project>).map(
                          (project) => (
                            <Grid item xs={12} md={12} key={project.id}>
                              <TeamViewProjectCard
                                teamId={id}
                                project={project}
                              />
                            </Grid>
                          )
                        )}
                    </Grid>
                  </Box>
                </>
              )
            }}
          />
        </React.Suspense>
      </Paper>
    </Container>
  )
}

export default TeamView
