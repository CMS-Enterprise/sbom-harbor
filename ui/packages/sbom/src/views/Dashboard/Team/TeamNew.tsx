/**
 * A component that renders a page with a form for creating a team.
 * URL: /team/new - @see {@link @cyclonedx/ui/sbom/Routes}.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamNew
 */
import * as React from 'react'
import { useForm } from 'react-hook-form'
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import Paper from '@mui/material/Paper'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import SubmitButton from '@/components/forms/SubmitButton'
import UserAutocomplete from '@/components/UserAutocomplete'
import { useAlert } from '@/hooks/useAlert'
import { useData } from '@/hooks/useData'
import { CONFIG } from '@/utils/constants'
import { TeamMember, UserTableRowType } from '@/types'
import getUserData from '@/utils/get-cognito-user'
import TeamMembersSection from './components/TeamMembersSection'
import TeamMembersTable from './components/TeamMembersTable'
import { defaultFormState } from './constants'
import { FormState } from './types'

const TeamForm = () => {
  // hook for using alert toasts
  const { setAlert } = useAlert()

  // local form state to set while submitting the form
  const [submitting, setSubmitting] = React.useState(false)

  // hook to get team data from the data context
  const {
    data: { teams = [] },
    setTeams,
  } = useData()

  // reducer for the form state
  const [formInput, setFormInput] = React.useReducer(
    (state: FormState, newState: FormState) => ({ ...state, ...newState }),
    defaultFormState
  )

  // react-form hook
  const { control } = useForm({
    mode: 'all',
    shouldUnregister: true,
  })

  // function that handlers change events on form inputs
  const handleInput = (
    event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = event.currentTarget
    setFormInput({ [name]: value })
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
  const handleSubmitForm = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    const abortController = new AbortController()

    try {
      setSubmitting(true)

      // get the user's JWT token and email from Amplify Auth
      const {
        jwtToken: token,
        userInfo: { attributes: { email = '' } = {} } = {},
      } = await getUserData()

      // get the team id and the final list of members from the form
      // TODO: create a new team id
      const { Id = 'RANDOM', members = [] as TeamMember[] } = formInput

      // add the current user to the members list as an admin
      if (!members.find((member) => member.email === email)) {
        members.push({ email: email, isTeamLead: true })
      }

      // create an object representing the new team
      // TODO: add creating projects and codebases to the form
      const newTeamData = { Id, members, projects: [], codebases: [] }

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
    <Container component="main" maxWidth="md" data-testid="team">
      <Paper
        variant="outlined"
        sx={{ mt: { xs: 3, md: 6 }, p: { xs: 2, md: 3 } }}
      >
        <Typography variant="h4" sx={{ mt: 2 }}>
          Create a Team
        </Typography>
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
                id="Id"
                label="Team Name"
                name="Id"
                onChange={handleInput}
                required
                value={formInput.Id}
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
                handleChange={handleInput}
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
                handleChange={handleInput}
                handleRemove={handleRemoveTeamMember}
                members={formInput.members?.filter((m) => !m.isTeamLead)}
                newEmail={formInput.newMemberEmail}
              />
            </Grid>
            <Grid item xs={6}>
              <UserAutocomplete
                label="Add new User"
                name="newUserSearch"
                control={control}
              />
            </Grid>
          </Grid>
          <Grid container spacing={6} className="match-height">
            <Grid item xs={12} md={12}>
              <TeamMembersTable
                members={(formInput?.members || []) as UserTableRowType[]}
              />
            </Grid>
          </Grid>
          <Box sx={{ display: 'flex', justifyContent: 'flex-end' }}>
            <SubmitButton disabled={submitting} />
          </Box>
        </Box>
      </Paper>
    </Container>
  )
}

export default TeamForm
