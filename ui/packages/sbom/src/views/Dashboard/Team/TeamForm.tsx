/**
 * A component that renders a page with a form for editing a team.
 * URL: /teams/:teamId/edit - @see {@link @cyclone-dx/ui/sbom/Routes}.
 * Enables the user to change the members of the team, the projects
 *  the team is working on, and the codebases inside those projects.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamEdit
 */
import * as React from 'react'
import { useMatch, useParams } from 'react-router-dom'
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
import { useData } from '@/hooks/useData'
import { CONFIG } from '@/utils/constants'
import { Team } from '@/types'
import { FormState, FormTeamState } from './types'
import { defaultFormState, defaultProject, defaultTeam } from './constants'
import TeamMembersSection from './components/TeamMembersSection'
import TeamViewProjectCreateCard from './components/TeamViewProjectCreateCard'
import TeamViewProjectCreationCard from './components/TeamViewProjectCreationCard'
import { useAuth } from '@/hooks/useAuth'

const TeamForm = () => {
  const newTeamRouteMatch = useMatch('/team/new')
  const { setAlert } = useAlert()
  const { data: { teams = {} } = {}, setTeams } = useData()
  const { user } = useAuth()
  const [submitting, setSubmitting] = React.useState(false)

  const {
    control,
    // TODO: use commented out react-hook-form methods
    // formState: { errors },
    // handleSubmit,
    // register,
    // watch,
  } = useForm({
    mode: 'all',
    shouldUnregister: true,
  })

  // hook for getting the /teams/:teamId route parameter
  const { teamId } = useParams()

  // find the team to edit from the data context and set it in local state.
  const [team] = React.useState((): FormTeamState => {
    // if the team data is not there or this is a new team, return the default team data.
    if (newTeamRouteMatch || !teams || !teamId || !teams[teamId]) {
      return defaultTeam
    }
    // create a new team data object with arrays for the projects, members, and tokens.
    const rawTeam = { ...(teams[teamId] || defaultTeam) }
    return {
      name: rawTeam.name,
      projects: Object.entries(rawTeam.projects),
      members: Object.entries(rawTeam.members),
      tokens: Object.entries(rawTeam.tokens),
    }
  })

  // form input reducer
  const [formInput, setFormInput] = React.useReducer(
    (state: FormState, newState: FormState) => ({ ...state, ...newState }),
    { ...defaultFormState, ...team }
  )

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
   */
  const handleSubmitForm = async (
    event: React.FormEvent<HTMLFormElement>
  ): Promise<() => void> => {
    event.preventDefault()
    const abortController = new AbortController()
    if (!user || submitting) return () => abortController.abort()
    const { attributes: { email = '' } = {} } = user

    try {
      setSubmitting(true)

      const {
        name = team.name,
        members = team.members,
        projects = team.projects,
        tokens: tokenEntries = team.tokens,
      } = formInput

      // filter out any empty email values from the projects object
      const projectEntries = projects.filter(([, p]) => !!p.name)

      // filter out any empty email values from the members object
      const membersEntries = members.filter(([, m]) => !!m.email)

      // ensure that the current user is in the members list as an admin
      if (members.findIndex(([, m]) => m.email === email) === -1) {
        membersEntries.push([uuidv4(), { email, isTeamLead: true }])
      }

      // create a final object representing the team and add it to the teams data.
      const updatedTeamsData = {
        ...teams,
        [uuidv4()]: {
          ...team,
          name,
          members: Object.fromEntries(membersEntries),
          projects: Object.fromEntries(projectEntries),
          tokens: Object.fromEntries(tokenEntries),
        } as Team,
      }

      // update the team in the database
      const response = await fetch(`${CONFIG.TEAMS_API_URL}`, {
        method: newTeamRouteMatch ? 'POST' : 'PUT',
        signal: abortController.signal,
        headers: { Authorization: `${user.jwtToken}` },
        body: JSON.stringify(updatedTeamsData),
      })

      // if the request was unsuccessful, throw an error and go to catch block
      if (response.status !== 200) {
        throw new Error(response.statusText)
      }

      // update global app data with the new team since API call was successful
      setTeams(updatedTeamsData)
      setSubmitting(false)
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
                members={formInput.members.filter(([, m]) => m.isTeamLead)}
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
