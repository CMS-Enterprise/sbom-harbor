/**
 * A component that renders a page with a form for editing a team.
 * URL: /teams/:teamId/edit - @see {@link @cyclone-dx/ui/sbom/Routes}.
 * Enables the user to change the members of the team, the projects
 *  the team is working on, and the codebases inside those projects.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamEdit
 */
import * as React from 'react'
import { useParams } from 'react-router-dom'
import { useForm } from 'react-hook-form'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import Paper from '@mui/material/Paper'
import Typography from '@mui/material/Typography'
import UserAutocomplete from '@/components/UserAutocomplete'
import SubmitButton from '@/components/forms/SubmitButton'
import { useAlert } from '@/hooks/useAlert'
import { useData } from '@/hooks/useData'
import { CONFIG } from '@/utils/constants'
import getUserData from '@/utils/get-cognito-user'
import { Codebase } from '@/types'
import TeamMembersSection from './components/TeamMembersSection'
import TeamViewProjectCreateCard from './components/TeamViewProjectCreateCard'
import TeamViewProjectCreationCard from './components/TeamViewProjectCreationCard'
import { defaultFormState, defaultProject, defaultTeam } from './constants'
import { FormState } from './types'

const TeamForm = () => {
  // hook for getting the /teams/:teamId route parameter
  const { teamId } = useParams()

  // hook for using alert toasts
  const { setAlert } = useAlert()

  // local form state to set while submitting the form
  const [submitting, setSubmitting] = React.useState(false)

  // hook to get team data from the data context
  const {
    data: { teams = [] },
    setTeams,
  } = useData()

  // find the team to edit from the data context and set it in local state.
  const [team] = React.useState(() => {
    const team = teams.find((t) => t.Id === teamId)
    if (!team) {
      // TODO: handle error where team isn't found with error boundary.
      throw new Error(`Team with ID ${teamId} not found.`)
    }
    return team || { ...defaultTeam }
  })

  // react-form hook
  const { control } = useForm({
    mode: 'all',
    shouldUnregister: true,
  })

  // reducer for the form state
  const [formInput, setFormInput] = React.useReducer(
    (state: FormState, newState: FormState) => ({ ...state, ...newState }),
    defaultFormState
  )

  // effect that updates the form state when the team is updated
  React.useEffect(() => {
    if (team?.members?.length) {
      const { members = [], projects = [], Id = '' } = team
      setFormInput({ ...formInput, members, projects, Id })
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [team])

  /**
   * Handler for change events for the team member email inputs.
   * @param {React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>} event The event.
   */
  const handleTeamMemberInputChange = (
    event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = event.currentTarget
    setFormInput({ [name]: value })
  }

  /**
   * Handler for adding a new project to the team
   */
  const handleAddProject = () => {
    const { projects = [] } = formInput
    const newProjectsToAdd = [...projects, { ...defaultProject }]
    setFormInput({ ...formInput, projects: newProjectsToAdd })
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
    const members = formInput?.members?.filter((m) => m.email !== email)
    setFormInput({ members })
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
      return
    }
    // get the list of members already in the form state.
    const { members = [] } = formInput
    // update the form state with the new member and clear the email input,
    setFormInput({
      members: [
        // filter existing members to ensure new user is not added twice,
        ...members.filter((m) => m.email !== email),
        { email, isTeamLead: admin },
      ],
      // clear the email input for the field for the right type of user.
      ...(admin ? { newAdminEmail: '' } : { newMemberEmail: '' }),
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

    try {
      // set the form to submitting state
      setSubmitting(true)

      // filter out any empty projects or those with no name, and
      // ensure that codebases are at least defined as an empty array.
      const validProjects = formInput?.projects
        ?.filter((p) => !!p.projectName)
        ?.map((p) => {
          return {
            ...p,
            codebases:
              p?.codebases?.filter((cb: Codebase) => cb.codebaseName) || [],
          }
        })

      // update the form state with the "corrected" projects
      setFormInput({ ...formInput, projects: validProjects })

      // get the user's JWT token and email from Amplify Auth
      const {
        jwtToken: token,
        userInfo: { attributes: { email = '' } = {} } = {},
      } = await getUserData()

      // get the team id and the final list of members from the form state
      const { Id = team.Id, members = team.members, projects } = formInput

      // ensure that the current user is in the members list as an admin
      if (!members.find((member) => member.email === email)) {
        members.push({ email: email, isTeamLead: true })
      }

      // create a final object representing the team
      const newTeamData = {
        ...team,
        Id,
        members,
        projects,
      }

      // fetch the user's team from the Teams API endpoint
      const response = await fetch(`${CONFIG.TEAMS_API_URL}`, {
        method: 'PUT',
        signal: abortController.signal,
        headers: { Authorization: `Bearer ${token}` },
        body: JSON.stringify(newTeamData),
      })

      // if the request was unsuccessful, throw an error
      if (response.status !== 200) {
        throw new Error(response.statusText)
      }

      // update global app data with the new team since API call was successful
      setTeams([...teams, newTeamData])
      setAlert({
        message: 'Team updated successfully',
        severity: 'success',
      })
      setSubmitting(false)
    } catch (error) {
      // if theres an error, display an error message in a toast component.
      setAlert({
        message: 'Something went wrong, unable to update team!',
        severity: 'error',
      })
      setSubmitting(false)
      console.error(error)
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
        <Typography variant="h4" sx={{ mt: 2 }}>
          {team.Id}
        </Typography>
        <Box
          component="form"
          noValidate
          autoComplete="off"
          onSubmit={handleSubmitForm}
        >
          <Grid container spacing={6}>
            <Grid item xs={6}>
              <TeamMembersSection
                name="newAdminEmail"
                title="admins"
                handleAdd={handleAddTeamAdmin}
                handleChange={handleTeamMemberInputChange}
                handleRemove={handleRemoveTeamMember}
                members={formInput.members?.filter((m) => m.isTeamLead)}
                newEmail={formInput.newAdminEmail}
              />
            </Grid>
            <Grid item xs={6}>
              <TeamMembersSection
                name="newMemberEmail"
                title="members"
                handleAdd={handleAddTeamMember}
                handleChange={handleTeamMemberInputChange}
                handleRemove={handleRemoveTeamMember}
                members={formInput.members?.filter((m) => !m.isTeamLead)}
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
              formInput.projects.map((project, index) => (
                <Grid item xs={12} md={12} key={`project-${index}`}>
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
