/**
 * A component that renders a page with a form for editing a team.
 * URL: /teams/:teamId/edit - @see {@link @cyclone-dx/ui/sbom/Routes}.
 * Enables the user to change the members of the team, the projects
 *  the team is working on, and the codebases inside those projects.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamEdit
 */
import * as React from 'react'
import {
  useLoaderData,
  useMatch,
  useNavigate,
  useParams,
} from 'react-router-dom'
import { useForm } from 'react-hook-form'
import { v4 as uuidv4 } from 'uuid'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import Paper from '@mui/material/Paper'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import UserAutocomplete from '@/components/UserAutocomplete'
import SubmitButton from '@/components/forms/SubmitButton'
import { useAlert } from '@/hooks/useAlert'
import { useAuthState } from '@/hooks/useAuth'
import harborRequest from '@/utils/harborRequest'
import { Project } from '@/types'
import { FormState, FormTeamState } from './types'
import { defaultFormState, defaultProject } from './constants'
import TeamMembersSection from './components/TeamMembersSection'
import TeamViewProjectCreateCard from './components/TeamViewProjectCreateCard'
import TeamViewProjectCreationCard from './components/TeamViewProjectCreationCard'

/**
 * A component that renders a page with a form for creating/editing a team.
 * URL: /team/new - @see {@link @cyclone-dx/ui/sbom/Routes}.
 * URL: /teams/:teamId/edit - @see {@link @cyclone-dx/ui/sbom/Routes}.
 */
const TeamForm = () => {
  const { setAlert } = useAlert()

  // auth state hook for user info and token
  const { jwtToken, email } = useAuthState()

  // route loader hook to fetch team data
  const team = useLoaderData() as FormTeamState

  // route params hook to get team id
  const { teamId = '' } = useParams()

  // route match hook to determine if this is an edit or create form
  const newTeamRouteMatch = useMatch('/team/new')

  // route navigate hook to redirect back to team view page on cancel
  const navigate = useNavigate()

  // component state for form data
  const [isSubmitting, setSubmitting] = React.useState(false)

  const { control } = useForm({
    mode: 'all',
    shouldUnregister: true,
  })

  // form input reducer
  const [formInput, setFormInput] = React.useReducer(
    (state: FormState, newState: FormState) => ({ ...state, ...newState }),
    { ...defaultFormState, ...team }
  )

  const admins = React.useMemo(() => {
    if (!formInput?.members) return []
    return formInput?.members.filter((m) => m.isTeamLead === true) || []
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
    (payload: Project) => {
      return setFormInput({
        ...formInput,
        projects: {
          ...formInput.projects,
          [payload.id]: { ...payload },
        },
      })
    },
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
    const nextData = formInput.members.filter((m) => m.email !== email)
    setFormInput({ ...formInput, members: nextData })
  }

  /**
   * Generic handler for adding a new team member or admin to the form state.
   * @param {boolean} admin Whether the user being added is an admin or not.
   */
  const handleAddMember = (admin: boolean) => {
    // get the email of the user to add, either as an admin or a member.
    const email = admin ? formInput.newAdminEmail : formInput.newMemberEmail
    // return early if there is no email defined in the form state.
    if (!email) {
      console.warn('Tried to add a new member without an email')
      return
    }
    // get the list of members already in the form state, and add the
    // new member to the form state if they are not already in the list.
    const nextData = [...formInput.members]
    if (!nextData.find((m) => m.email === email)) {
      const id = uuidv4()
      nextData.push({ id, email, isTeamLead: admin })
    }
    // update the form state with the new member and clear the email input.
    setFormInput({
      ...formInput,
      ...(admin ? { newAdminEmail: '' } : { newMemberEmail: '' }),
      members: nextData,
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

    if (!jwtToken || isSubmitting) {
      return () => abortController.abort()
    }

    try {
      setSubmitting(true)

      const {
        members = team.members,
        projects = team.projects,
        tokens: tokenEntries = team.tokens,
      } = formInput

      // filter out projects with empty name values from the projects object
      // TODO: add project schema validation to the form to prevent this from happening
      const projectEntries = Object.entries(projects)
        .filter(([, p]) => !!p.name)
        .map(([id, p]) => ({ ...p, id, codebases: Object.values(p.codebases) }))

      // filter out any empty email values from the projects object
      // TODO: add emailvalidation to the form to prevent this from happening
      const membersEntries = members.filter((m) => !!m.email)

      // ensure that the current user is in the members list as an admin with uuid.
      // TODO: a user shouldn't be able to remove themselves from the team
      if (members.findIndex((m) => m.email === email) === -1) {
        membersEntries.push({ id: uuidv4(), email, isTeamLead: true })
      }

      // make request to update teams data in the database.
      await harborRequest({
        // determine the endpoint to use based on if this is a create or edit form.
        path: newTeamRouteMatch ? '/teams' : `/team/${teamId}`,
        // determine the request verb based on if this is a create or edit form.
        method: newTeamRouteMatch ? 'POST' : 'PUT',
        // pass the jwt token from the authLoader to the request to authenticate the user.
        jwtToken,
        // create a final object representing the team data to send to the server.
        body: {
          name: formInput.name,
          members: Object.values(membersEntries),
          projects: Object.values(projectEntries),
          tokens: Object.values(tokenEntries),
        },
        // pass the abort controller to the request to allow for cancelling the request.
        signal: abortController.signal,
      })

      setSubmitting(false)
      setAlert({
        message: 'Team updated successfully',
        severity: 'success',
      })
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
    <Container component="main" maxWidth="xl" data-testid="team">
      <Paper
        variant="outlined"
        sx={{ mt: { xs: 3, md: 6 }, p: { xs: 2, md: 3 } }}
      >
        <Box
          component="form"
          noValidate
          autoComplete="off"
          onSubmit={handleSubmitForm}
        >
          <Grid container spacing={6}>
            <Grid item xs={12} sx={{ mt: 4 }}>
              <TextField
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
                sx={{
                  display: 'revert',
                }}
              />
            </Grid>
            <Grid item xs={6}>
              <TeamMembersSection
                name="newAdminEmail"
                title="admins"
                handleAdd={handleAddTeamAdmin}
                handleChange={handleInputFieldChange}
                handleRemove={handleRemoveTeamMember}
                members={admins}
                newEmail={formInput.newAdminEmail}
              />
            </Grid>
            <Grid item xs={6}>
              <TeamMembersSection
                name="newMemberEmail"
                title="members"
                handleAdd={handleAddTeamMember}
                handleChange={handleInputFieldChange}
                handleRemove={handleRemoveTeamMember}
                members={formInput.members.filter((m) => !m.isTeamLead)}
                newEmail={formInput.newMemberEmail}
              />
            </Grid>
            <Grid item xs={6}>
              <UserAutocomplete
                label="Search for a User"
                name="newUserSearch"
                control={control}
              />
            </Grid>
          </Grid>

          <Typography variant="h6" sx={{ mt: 6, mb: 4 }}>
            Projects
          </Typography>

          <Grid container spacing={6} sx={{ mb: 6 }}>
            {Object.values(formInput.projects).map((project) => (
              <Grid item xs={12} md={12} key={project.id}>
                <TeamViewProjectCreateCard
                  project={project}
                  onUpdate={handleUpdateProject}
                />
              </Grid>
            ))}
            <Grid item xs={12} md={12}>
              <TeamViewProjectCreationCard onClick={handleAddProject} />
            </Grid>
          </Grid>

          <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
            <Button
              onClick={handleCancel}
              color="error"
              variant="outlined"
              sx={{ mt: 3, ml: 1 }}
            >
              Cancel
            </Button>
            <SubmitButton disabled={isSubmitting} />
          </Box>
        </Box>
      </Paper>
    </Container>
  )
}

export default TeamForm
