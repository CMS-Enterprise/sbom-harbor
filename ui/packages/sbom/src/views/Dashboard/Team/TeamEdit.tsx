/**
 * A component that renders a page with a form for editing a team.
 * URL: /teams/:teamId/edit - @see {@link @cyclone-dx/ui/sbom/Routes}.
 * Enables the user to change the members of the team, the projects
 *  the team is working on, and the codebases inside those projects.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamEdit
 */
// ** React Imports
import * as React from 'react'
import { useParams } from 'react-router-dom'
import { useForm } from 'react-hook-form'

// ** AWS Imports
import { Auth } from '@aws-amplify/auth'

// ** MUI Components
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import Paper from '@mui/material/Paper'
import Typography from '@mui/material/Typography'

// ** App Imports
import UserSearchInput from '@/components/UserAutocomplete'
import { useAlert } from '@/hooks/useAlert'
import { useData } from '@/hooks/useData'
import { TEAMS_API_URL } from '@/utils/constants'
import { CognitoUserInfo, Project, TeamMember } from '@/types'

// ** Local Imports
import TeamViewProjectCreationCard from './TeamViewProjectCreationCard'
import TeamViewProjectCreateCard from './TeamViewProjectCreateCard'
import TeamMembersSection from './components/TeamMembersSection'

const defaultProject: Project = {
  projectName: '',
  codebases: [],
  tokens: [],
}

type FormState = {
  Id?: string
  newAdminEmail?: string
  newMemberEmail?: string
  members?: TeamMember[]
  projects?: Project[]
}

const defaultTeam = {
  Id: '',
  members: [],
  projects: [],
}

const defaultFormState = {
  newAdminEmail: '',
  newMemberEmail: '',
  ...defaultTeam,
}

const TeamForm = () => {
  // hook for using alert toasts
  const { setAlert } = useAlert()

  // hook for getting the /teams/:teamId route parameter
  const { teamId } = useParams()

  // local form state to set while submitting the form
  const [submitting, setSubmitting] = React.useState(false)

  // get the auth context
  const { data, setTeams } = useData()

  // get the users teams from the auth context and the team being edited
  const teams = data?.teams || []
  const team = teams.find((t) => t.Id === teamId) || { ...defaultTeam }

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

  // react-form hook
  const { control } = useForm({
    mode: 'all',
    shouldUnregister: true,
  })

  // function that handlers change events on form inputs
  const handleInput = (
    evt: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = evt.currentTarget
    setFormInput({ [name]: value })
  }

  // function that handles adding a new project to the team
  const handleAddProject = () => {
    const { projects = [] } = formInput
    const newProjectsToAdd = [...projects, { ...defaultProject }]
    setFormInput({ ...formInput, projects: newProjectsToAdd })
  }

  // function that handles removing a team member from the form state
  const handleRemoveTeamMember = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    const email = event.currentTarget.dataset.value
    const members = formInput?.members?.filter((m) => m.email !== email)
    setFormInput({ members })
  }

  // function that handles adding a new team admin member to the form state.
  // TODO: dedupe these handleAdd functions
  const handleAddTeamAdmin = () => {
    const { newAdminEmail: email, members = [] } = formInput
    if (!email) return
    setFormInput({
      // update the members array with the new admin
      members: [
        // filter existing members to ensure new user is not added twice
        ...members.filter((m) => m.email !== email),
        // and combine that list with the new user object
        { email, isTeamLead: true },
      ],
      // clear the new admin email field
      newAdminEmail: '',
    })
  }

  // function that handles adding a new team member to the form state.
  // TODO: dedupe these handleAdd functions
  const handleAddTeamMember = () => {
    const { newMemberEmail: email, members = [] } = formInput
    if (!email) return
    setFormInput({
      // update the members array with the new member
      members: [
        // filter existing members to ensure new user is not added twice
        ...members.filter((m) => m.email !== email),
        // and combine that list with the new user object
        { email, isTeamLead: false },
      ],
      // clear the new member email field
      newMemberEmail: '',
    })
  }

  const handleSubmitForm = async (event: React.FormEvent<HTMLFormElement>) => {
    // prevent the default form submission behavior
    event.preventDefault()

    // signal to abort the fetch in progress if the view is unmounted
    const abortController = new AbortController()

    try {
      // set the form to submitting state
      setSubmitting(true)

      // filter out any empty projects or those with no name.
      // also ensure that codebases are at least defined as an empty array.
      const validProjects = formInput?.projects
        ?.filter((p) => !!p.projectName)
        ?.map((p) => {
          return {
            ...p,
            codebases: p?.codebases?.filter((cb) => cb.codebaseName) || [],
          }
        })

      // update the form state with the "corrected" projects
      setFormInput({
        ...formInput,
        projects: validProjects,
      })

      // get the user's info and token from Amplify Auth
      const [cognitoUser, token]: [CognitoUserInfo, string] = await Promise.all(
        [
          Auth.currentUserInfo(),
          Auth.currentSession().then((session) =>
            session.getIdToken().getJwtToken()
          ),
        ]
      )

      // get the current user's email from the cognito user info
      const {
        attributes: { email },
      } = cognitoUser

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
      const response = await fetch(`${TEAMS_API_URL}`, {
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
      setTeams([...(data?.teams || []), newTeamData])
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
      console.error(error)
      setSubmitting(false)
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
              <UserSearchInput
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
            <Button
              type="submit"
              sx={{ mt: 3, ml: 1 }}
              variant="contained"
              color="primary"
              disabled={submitting}
            >
              Save
            </Button>
          </Box>
        </Box>
      </Paper>
    </Container>
  )
}

export default TeamForm
