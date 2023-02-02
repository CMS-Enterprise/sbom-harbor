/**
 * A component that renders a page with a form for editing a team.
 * URL: /teams/:teamId/edit - @see {@link @cyclone-dx/ui/sbom/Routes}.
 * Enables the user to change the members of the team, the projects
 *  the team is working on, and the codebases inside those projects.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamEdit
 */
import * as React from 'react'
import {
  Await,
  useLoaderData,
  useMatch,
  useNavigate,
  useParams,
} from 'react-router-dom'
import { useForm } from 'react-hook-form'
import { Auth } from 'aws-amplify'
import { v4 as uuidv4 } from 'uuid'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Card from '@mui/material/Card'
import CardContent from '@mui/material/CardContent'
import Grid2 from '@mui/material/Unstable_Grid2'
import Paper from '@mui/material/Paper'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import updateTeam from '@/api/updateTeam'
import Fallback from '@/components/SimpleLoadingFallback'
import UserAutocomplete from '@/components/UserAutocomplete'
import SubmitButton from '@/components/forms/SubmitButton'
import { useAlert, DEFAULT_ALERT_TIMEOUT } from '@/hooks/useAlert'
import { useAuthState } from '@/hooks/useAuth'
import reduceArrayToMap from '@/selectors/reduceArrayToMap'
import { Project, Team, TeamMemberRole } from '@/types'
import { defaultFormState, defaultProject } from './constants'
import TeamMembersSection from './components/TeamMembersSection'
import TeamViewProjectCreateCard from './components/TeamViewProjectCreateCard'
import TeamViewProjectCreationCard from './components/TeamViewProjectCreationCard'
import { FormState } from './types'

/**
 * A component that renders a page with a form for creating/editing a team.
 * URL: /team/new - @see {@link @cyclone-dx/ui/sbom/Routes}.
 * URL: /teams/:teamId/edit - @see {@link @cyclone-dx/ui/sbom/Routes}.
 */
const TeamForm = () => {
  const { setAlert } = useAlert()

  // auth state hook for user info and token
  const { jwtToken } = useAuthState()

  // route match hook to determine if this is an edit or create form
  const newTeamRouteMatch = useMatch('/app/team/new')

  // route loader hook to fetch team data
  const { data } = useLoaderData() as {
    data: Promise<
      Team & {
        membersTableRows: {
          id: string
          email: string
          isTeamLead: boolean
          role: TeamMemberRole
          username: string
        }[]
      }
    >
  }

  // route params hook to get team id
  const { teamId = '' } = useParams()

  // route navigate hook to redirect back to team view page on cancel
  const navigate = useNavigate()

  // component state for form data
  const [isSubmitting, setSubmitting] = React.useState(false)

  // react-hook-form hook to manage form state
  const { control } = useForm({
    mode: 'all',
    shouldUnregister: true,
  })

  // form input reducer
  const [formInput, setFormInput] = React.useReducer(
    (state: FormState, newState: FormState) => ({ ...state, ...newState }),
    { ...defaultFormState }
  )

  React.useEffect(() => {
    if (newTeamRouteMatch) return
    // if this is an edit form, set the form input state to the team data
    data.then((data) => {
      setFormInput(data)
    })
  }, [newTeamRouteMatch, data])

  // memoize separating admins and regular members from the team members.
  const [admins, members] = React.useMemo(() => {
    // if there are no members, return empty arrays for both admins and members
    if (!formInput?.members) {
      return [[], []]
    }
    // get an array of values from the members map
    const values = Object.values(formInput.members)
    // return filtered arrays of admins and members
    return [
      values.filter((m) => m.isTeamLead === true),
      values.filter((m) => m.isTeamLead === false),
    ]
  }, [formInput.members])

  /**
   * Handler for change events for the team member email inputs.
   * @param {React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>} event The event.
   */
  const handleInputFieldChange = (
    event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = event.currentTarget
    setFormInput({ ...formInput, [name]: value })
  }

  /**
   * Handler for adding a new project to the team
   */
  const handleAddProject = () => {
    const id = uuidv4()
    // update the form state with the new projects object
    setFormInput({
      ...formInput,
      projects: {
        ...formInput.projects,
        [id]: { ...defaultProject, id },
      },
    })
  }

  /**
   * Handler for adding a new project to the team
   */
  const handleUpdateProject = React.useCallback(
    (payload: Project) =>
      setFormInput({
        ...formInput,
        projects: {
          ...formInput.projects,
          [payload.id]: { ...payload },
        },
      }),
    [formInput]
  )

  /**
   * Handler for removing a team member from the form state.
   * @param {React.MouseEvent<HTMLButtonElement>} event The event triggered
   *  by clicking the remove button next to a team member email line.
   */
  const handleRemoveTeamMember = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    const email = event.currentTarget.dataset.value
    setFormInput({
      ...formInput,
      members: reduceArrayToMap(
        Object.values(formInput.members).filter((m) => m.email !== email)
      ),
    })
  }

  /**
   * Generic handler for adding a new team member or admin to the form state.
   * @param {boolean} admin Whether the user being added is an admin or not.
   */
  const handleAddMember = (admin: boolean) => {
    // get the email of the user to add, either as an admin or a member.
    const email = admin ? formInput.newAdminEmail : formInput.newMemberEmail
    // return early if there is no email defined in the form state.
    // TODO: add email validation to the form
    if (!email) {
      console.warn('Tried to add a new member without an email')
      return
    }
    // get the list of members already in the form state
    const nextData = Object.values(formInput.members)
    // add the new member to the form state if not already in the list.
    if (!nextData.find((m) => m.email === email)) {
      const id = uuidv4()
      nextData.push({ id, email, isTeamLead: admin })
    }
    // update the form state with the new member and clear the email input.
    setFormInput({
      ...formInput,
      ...(admin ? { newAdminEmail: '' } : { newMemberEmail: '' }),
      members: reduceArrayToMap(nextData),
    })
  }

  /**
   * Handler for adding a new team admin member to the form state.
   * @requires TeamForm#handleAddMember
   */
  const handleAddTeamAdmin = () => handleAddMember(true)

  /**
   * Handler for adding a new team regular member to the form state.
   * @requires TeamForm#handleAddMember
   */
  const handleAddTeamMember = () => handleAddMember(false)

  /**
   * Handler for cancelling the form and returning to the team view page.
   * @param {React.MouseEvent<HTMLButtonElement>} event The event triggered
   * by clicking the cancel button.
   */
  const handleCancel = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault()
    // go back to the previous route.
    navigate(-1)
  }

  /**
   * Handler for isSubmitting the form to update a team.
   * @param {React.FormEvent<HTMLFormElement>} event Form submit event.
   * @returns {Promise<void>} Promise that resolves to void when the submit
   * request completes, or resolves to an abort signal if the request fails.
   * @todo switch to updating a single team instead of all teams
   */
  const handleSubmitForm = async (
    event: React.FormEvent<HTMLFormElement>
  ): Promise<() => void> => {
    event.preventDefault()
    const abortController = new AbortController()

    // if already submitting, return early.
    if (isSubmitting === true) {
      return () => abortController.abort()
    }

    try {
      setSubmitting(true)

      // get the JWT token from the auth context or from the Cognito session
      const token =
        jwtToken || (await Auth.currentAuthenticatedUser()).getJwtToken()

      // check that there is a jwtToken
      if (!token) {
        throw new Error('Unable to make request: no JWT token.')
      }

      // make the request
      const response = await updateTeam({
        abortController,
        formInput,
        jwtToken,
        newTeamRouteMatch: !!newTeamRouteMatch,
        teamId,
      })

      // show success alert
      setAlert({
        message: `Team ${
          newTeamRouteMatch ? 'created' : 'updated'
        } successfully`,
        severity: 'success',
        timeout: DEFAULT_ALERT_TIMEOUT,
      })

      // navigate to the team page after update is done
      // FIXME: newly created team won't load until user logs out and back in
      setTimeout(async () => {
        // get ID of the team that was created/updated
        const { id } = (await response.json()) as Team
        // navigate to view that team
        navigate(`/teams/${id}`)
      }, DEFAULT_ALERT_TIMEOUT + 10)
    } catch (error) {
      console.error(error)
      setSubmitting(false)
      setAlert({
        message: 'Something went wrong, please try again.',
        severity: 'error',
      })
    }

    // return the abort controller as the cleanup function for this handler
    return () => abortController.abort()
  }

  return (
    <Paper sx={{ p: 4 }}>
      <React.Suspense fallback={<Fallback />}>
        <Await
          resolve={data}
          errorElement={<div>Could not load teams 😬</div>}
          // eslint-disable-next-line react/no-children-prop
          children={({ name = '' }) => (
            <>
              <Typography component="h1" variant="h4" sx={{ mb: 4 }}>
                {newTeamRouteMatch ? 'New Team' : `Edit Team: "${name}"`}
              </Typography>
              <Box
                component="form"
                autoComplete="off"
                onSubmit={handleSubmitForm}
                data-testid="team-form"
              >
                <Grid2 container spacing={6}>
                  <Grid2 xs={12} sx={{ p: 3.5 }}>
                    <TextField
                      autoFocus
                      fullWidth
                      name="name"
                      id="team"
                      label="Team Name"
                      onChange={handleInputFieldChange}
                      required
                      value={formInput.name}
                      variant="standard"
                      InputProps={{
                        sx: {
                          '& .Mui-disabled': {
                            color: 'text.primary',
                          },
                        },
                      }}
                      sx={{ display: 'revert' }}
                    />
                  </Grid2>

                  <Grid2 xs={12} sx={{ mt: 2, pb: 0 }}>
                    <Typography component="h2" variant="h5">
                      Team Members
                    </Typography>
                  </Grid2>

                  <Grid2 xs={12}>
                    <Card>
                      <CardContent sx={{ p: 0 }}>
                        <Grid2 container spacing={3}>
                          <Grid2 xs={12} md={6}>
                            <TeamMembersSection
                              name="newAdminEmail"
                              title="admins"
                              handleAdd={handleAddTeamAdmin}
                              handleChange={handleInputFieldChange}
                              handleRemove={handleRemoveTeamMember}
                              members={admins}
                              newEmail={formInput.newAdminEmail}
                            />
                          </Grid2>
                          <Grid2 xs={12} md={6}>
                            <TeamMembersSection
                              name="newMemberEmail"
                              title="members"
                              handleAdd={handleAddTeamMember}
                              handleChange={handleInputFieldChange}
                              handleRemove={handleRemoveTeamMember}
                              members={members}
                              newEmail={formInput.newMemberEmail}
                            />
                          </Grid2>
                          <Grid2 xs={12}>
                            <UserAutocomplete
                              label="Search for a User"
                              name="newUserSearch"
                              control={control}
                            />
                          </Grid2>
                        </Grid2>
                      </CardContent>
                    </Card>
                  </Grid2>

                  <Grid2 xs={12} sx={{ mt: 2, pb: 0 }}>
                    <Typography component="h2" variant="h5">
                      Projects
                    </Typography>
                  </Grid2>

                  {Object.values(formInput.projects).map((project) => (
                    <Grid2 xs={12} md={12} key={project.id}>
                      <TeamViewProjectCreateCard
                        project={project}
                        onUpdate={handleUpdateProject}
                      />
                    </Grid2>
                  ))}
                  <Grid2 xs={12} md={12}>
                    <TeamViewProjectCreationCard onClick={handleAddProject} />
                  </Grid2>

                  <Grid2 xs={12}>
                    <Box
                      sx={{
                        display: 'flex',
                        justifyContent: 'space-between',
                      }}
                    >
                      <Button
                        onClick={handleCancel}
                        color="error"
                        variant="outlined"
                        sx={{ mt: 3 }}
                      >
                        Cancel
                      </Button>
                      <SubmitButton disabled={isSubmitting} />
                    </Box>
                  </Grid2>
                </Grid2>
              </Box>
            </>
          )}
        />
      </React.Suspense>
    </Paper>
  )
}

export default TeamForm
