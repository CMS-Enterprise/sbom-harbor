/**
 * A component that renders a team.
 * URL: /team/:teamId - @see {@link @cyclonedx/ui/sbom/Routes}.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamView
 */
import * as React from 'react'
import { Await, Link, useLoaderData } from 'react-router-dom'
import Box from '@mui/material/Box'
import Grid2 from '@mui/material/Unstable_Grid2'
import Paper from '@mui/material/Paper'
import Typography from '@mui/material/Typography'
import Fallback from '@/components/SimpleLoadingFallback'
import TeamMembersTable from './components/TeamMembersTable'
import TeamViewProjectCard from './components/TeamViewProjectCard'
import TokensTable from './components/TokensTable'
import { Project, Team, TeamMemberRole } from '@/types'

/**
 * The view that renders a team at the path `/team/:teamId`.
 * @see {@link @cyclonedx/ui/sbom/router}
 * @returns {JSX.Element} A component that renders a team.
 */
const TeamView = (): JSX.Element => {
  // @ts-ignore
  const { data } = useLoaderData()

  return (
    <Paper variant="outlined" sx={{ p: 4 }}>
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
                  <Typography component="h1" variant="h4" sx={{ mt: 1, mb: 3 }}>
                    {name}
                  </Typography>
                  <Link to={`edit`}>Edit Team</Link>
                </Box>
                <Box sx={{ mb: 4 }}>
                  <Grid2 container spacing={3}>
                    <Grid2>
                      <Typography
                        component="h2"
                        variant="h5"
                        sx={{ mt: 2, mb: 0 }}
                      >
                        Members
                      </Typography>
                    </Grid2>
                    <Grid2 xs={12} md={12}>
                      <TeamMembersTable members={membersTableRows} />
                    </Grid2>
                  </Grid2>
                </Box>
                <Box>
                  <Grid2 container spacing={3} sx={{ mb: 0 }}>
                    <Grid2>
                      <Typography
                        component="h2"
                        variant="h5"
                        sx={{ mt: 2, mb: 0 }}
                      >
                        Tokens
                      </Typography>
                    </Grid2>
                    <Grid2 xs={12} md={12}>
                      <TokensTable tokens={Object.values(tokens)} teamId={id} />
                    </Grid2>
                  </Grid2>
                </Box>
                <Box>
                  <Grid2 container spacing={3} sx={{ mb: 2 }}>
                    <Grid2>
                      <Typography
                        component="h2"
                        variant="h5"
                        sx={{ mt: 2, mb: 0 }}
                      >
                        Projects
                      </Typography>
                    </Grid2>
                    {projects &&
                      Object.values(projects as Record<string, Project>).map(
                        (project) => (
                          <Grid2 xs={12} md={12} key={project.id}>
                            <TeamViewProjectCard
                              teamId={id}
                              project={project}
                            />
                          </Grid2>
                        )
                      )}
                  </Grid2>
                </Box>
              </>
            )
          }}
        />
      </React.Suspense>
    </Paper>
  )
}

export default TeamView
