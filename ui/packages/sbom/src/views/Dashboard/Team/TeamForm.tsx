/**
 * A component that renders a page with a form for editing a team.
 * URL: /teams/:teamId/edit - @see {@link @cyclone-dx/ui/sbom/Routes}.
 * Enables the user to change the members of the team, the projects
 *  the team is working on, and the codebases inside those projects.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamEdit
 */
import * as React from 'react'
import { useLoaderData, useMatch, useParams } from 'react-router-dom'
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
import { CONFIG } from '@/utils/constants'
import { Project, TeamMember, Token } from '@/types'
import { FormState, FormTeamState } from './types'
import { defaultFormState, defaultProject } from './constants'
import TeamMembersSection from './components/TeamMembersSection'
import TeamViewProjectCreateCard from './components/TeamViewProjectCreateCard'
import TeamViewProjectCreationCard from './components/TeamViewProjectCreationCard'

/**
 * Helper method that converts entries to an array of objects with ids used
 *  for generating the properties of the object in the body of the request.
 * @param {[string, Project | Token | TeamMember][]} entries key, value pairs
 *  of the projects, tokens, or team members to be added to the team.
 * @returns {Object} the projects, tokens, or team members to be added to the team,
 *  with the key set to the value of the id field.
 */
const mapEntriesToArray = (
  entries: [string, Project | Token | TeamMember][]
): Array<TeamMember | Project | Token> =>
  entries.map(([key, value]) => ({
    id: key,
    ...value,
  }))

/**
 * A component that renders a page with a form for creating/editing a team.
 * URL: /team/new - @see {@link @cyclone-dx/ui/sbom/Routes}.
 * URL: /teams/:teamId/edit - @see {@link @cyclone-dx/ui/sbom/Routes}.
 */
const TeamForm = () => {
  const { setAlert } = useAlert()
  const { jwtToken, email } = useAuthState()

  // route loader hook to fetch team data
  const team = useLoaderData() as FormTeamState

  // route match hook to determine if this is an edit or create form
  const newTeamRouteMatch = useMatch('/team/new')
  const { teamId = '' } = useParams()

  const [submitting, setSubmitting] = React.useState(false)

  const {
    control,
    // TODO: use commented out react-hook-form methods
  } = useForm({
    mode: 'all',
    shouldUnregister: true,
  })

  // form input reducer
  const [formInput, setFormInput] = React.useReducer(
    (state: FormState, newState: FormState) => ({ ...state, ...newState }),
    { ...defaultFormState, ...team }
  )

  /* eslint-disable react-hooks/exhaustive-deps */
  React.useEffect(() => {
    if (!team || !formInput) return
    setFormInput({ ...formInput, ...team })
  }, [team])
  /* eslint-enable react-hooks/exhaustive-deps */

  const admins = React.useMemo(() => {
    if (!formInput?.members) return []
    return formInput.members.filter(([, m]) => m.isTeamLead === true)
  }, [formInput])

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
    // update the form state with the new projects object
    setFormInput({
      ...formInput,
      newProjects: [...formInput.newProjects, { ...defaultProject }],
    })
  }

  /**
   * Handler for removing a team member from the form state.
   * @param {React.MouseEvent<HTMLButtonElement>} event The event triggered
   *  by clicking the remove button next to a team member email line.
   */
  const handleRemoveTeamMember = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    const email = event.currentTarget.dataset.value
    const nextData = formInput.members.filter(([, m]) => m.email !== email)
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
    if (!nextData.find(([, m]) => m.email === email)) {
      nextData.push([uuidv4(), { email, isTeamLead: admin }])
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
   * Handler for submitting the form to update a team.
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

    if (!jwtToken || submitting) {
      return () => abortController.abort()
    }

    try {
      setSubmitting(true)

      const {
        members = team.members,
        projects = team.projects,
        tokens: tokenEntries = team.tokens,
      } = formInput

      // filter out any empty email values from the projects object
      const projectEntries = projects.filter(([, p]) => !!p.name)

      // filter out any empty email values from the members object
      const membersEntries = members.filter(([, m]) => !!m.email)

      // ensure that the current user is in the members list as an admin
      // TODO: use team member endpoint instead of editing them directly
      if (members.findIndex(([, m]) => m.email === email) === -1) {
        membersEntries.push([uuidv4(), { email, isTeamLead: true }])
      }

      // create a final object representing the team and add it to the teams data.
      const updatedTeamData = {
        name: formInput.name,
        members: mapEntriesToArray(membersEntries),
        projects: mapEntriesToArray(projectEntries),
        tokens: mapEntriesToArray(tokenEntries),
      }

      // determine the endpoint to use based on if this is a create or edit form.
      const url = newTeamRouteMatch
        ? CONFIG.TEAM_API_URL
        : `${CONFIG.TEAM_API_URL}/${teamId}`

      // determine the request verb based on if this is a create or edit form.
      const method = newTeamRouteMatch ? 'POST' : 'PUT'

      // update teams data in the database
      const response = await fetch(url, {
        method,
        body: JSON.stringify(updatedTeamData),
        headers: {
          Authorization: `${jwtToken}`,
          'Content-Type': 'application/json',
        },
        signal: abortController.signal,
      })

      // if the request was unsuccessful, throw an error and go to catch block
      if (response.status !== 200) {
        throw new Error(response.statusText)
      }

      // update global app data with the new team since API call was successful
      setSubmitting(false)

      // show a success message to the user
      setAlert({
        message: 'Team updated successfully',
        severity: 'success',
      })
    } catch (error) {
      console.error(error)
      setSubmitting(false)
      setAlert({
        message: 'Something went wrong, unable to update team!',
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
                members={formInput.members.filter(([, m]) => !m.isTeamLead)}
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
            <Grid item xs={12} md={12}>
              <TeamViewProjectCreationCard onClick={handleAddProject} />
            </Grid>
            {formInput?.projects &&
              formInput.projects.map(([key, project]) => (
                <Grid item xs={12} md={12} key={key}>
                  <TeamViewProjectCreateCard project={project} />
                </Grid>
              ))}
          </Grid>

          <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
            <Button sx={{ mt: 3, ml: 1 }} variant="outlined" color="error">
              Cancel
            </Button>
            <SubmitButton disabled={submitting} />
          </Box>
        </Box>
      </Paper>
    </Container>
  )
}

export default TeamForm
