/**
 * The default view that an authenticated user
 * first sees when they visit the app. It renders
 * a list of teams that the user is a member of, and
 * a list of the api keys that the user has access to.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Dashboard
 */
import * as React from 'react'
import { styled } from '@mui/material/styles'
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import List from '@mui/material/List'
import ListItem from '@mui/material/ListItem'
import Paper from '@mui/material/Paper'
import Stack from '@mui/material/Stack'
import { useData } from '@/providers/DataContext'

const DashboardCard = styled(Paper)(({ theme }) => ({
  padding: `${theme.spacing(1)} ${theme.spacing(2)}`,
  display: 'flex',
  flexDirection: 'column',
  height: 'auto',
  backgroundColor:
    theme.palette.mode === 'light'
      ? theme.palette.grey[100]
      : theme.palette.grey[800],
}))

const DashboardContainer = (): JSX.Element => {
  const { data } = useData()

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
          <Grid container spacing={3}>
            {/* Team */}
            <Grid item xs={12} md={12} lg={7}>
              <DashboardCard>
                <h1 style={{ letterSpacing: 2, textTransform: 'uppercase' }}>
                  Team
                </h1>
                <Stack>
                  {data.teams &&
                    data.teams.map((team) => (
                      <Box key={team.Id}>
                        <h2 style={{ fontWeight: 'bolder' }}>{team.Id}</h2>
                        <hr />
                        {team.members && (
                          <>
                            <h3>Members</h3>
                            {team.members.map((member) => (
                              <List key={member.email}>
                                <ListItem>{member.email}</ListItem>
                              </List>
                            ))}
                          </>
                        )}
                      </Box>
                    ))}
                </Stack>
              </DashboardCard>
            </Grid>

            {/* API Tokens */}
            <Grid item xs={12} md={12} lg={5}>
              <DashboardCard>
                <h1 style={{ letterSpacing: 2, textTransform: 'uppercase' }}>
                  API Tokens
                </h1>
              </DashboardCard>
            </Grid>
          </Grid>
        </Container>
      </Box>
    </Box>
  )
}
export default DashboardContainer
