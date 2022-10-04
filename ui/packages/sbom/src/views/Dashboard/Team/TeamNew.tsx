/**
 * A component that renders a page with a form for creating a team.
 * URL: /team/new - @see {@link @cyclonedx/ui/sbom/Routes}.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamNew
 */
import * as React from 'react'
import { useForm } from 'react-hook-form'
import { Auth } from '@aws-amplify/auth'

// ** MUI Components
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Container from '@mui/material/Container'
import FormControl from '@mui/material/FormControl'
import Grid from '@mui/material/Grid'
import IconButton from '@mui/material/IconButton'
import Input from '@mui/material/Input'
import InputAdornment from '@mui/material/InputAdornment'
import InputLabel from '@mui/material/InputLabel'
import Paper from '@mui/material/Paper'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'

// ** Icon Components
import AddCircleIcon from '@mui/icons-material/AddCircleOutline'
import RemoveCircleIcon from '@mui/icons-material/RemoveCircleOutline'

// ** App Components
import { useAlert } from '@/hooks/useAlert'
import { useData } from '@/hooks/useData'
import { TEAMS_API_URL } from '@/utils/constants'
import { CognitoUserInfo, TeamMember } from '@/utils/types'
import UserSearchInput from '@/components/UserAutocomplete'
import TeamMembersTable, { TableBodyRowType } from './TeamMembersTable'

type FormState = {
  Id?: string
  newAdminEmail?: string
  newMemberEmail?: string
  members?: TeamMember[]
}

const defaultFormState = {
  Id: '',
  newAdminEmail: '',
  newMemberEmail: '',
  members: [],
}

const TeamMemberReadOnly = ({
  index,
  email,
  handleRemove,
}: {
  index: number
  email: string
  handleRemove: (event: React.MouseEvent<HTMLButtonElement>) => void
}) => (
  <Input
    id={`member-${index}`}
    name={`member-${index}`}
    disabled
    readOnly
    fullWidth
    value={email}
    sx={(theme) => ({
      '& .Mui-disabled': {
        color:
          theme.palette.mode === 'dark'
            ? 'white !important'
            : 'black !important',
        WebkitTextFillColor:
          theme.palette.mode === 'dark'
            ? 'rgba(255, 255, 255, 1) !important'
            : 'rgba(0, 0, 0, 1) !important',
      },
    })}
    endAdornment={
      <InputAdornment position="end" sx={{ pr: 1 }}>
        <IconButton
          aria-label="remove"
          data-value={email}
          onClick={handleRemove}
          onMouseDown={handleRemove}
          edge="end"
        >
          <RemoveCircleIcon />
        </IconButton>
      </InputAdornment>
    }
  />
)

const TeamMembersSection = ({
  members = [],
  newEmail = '',
  title,
  name,
  handleAdd,
  handleRemove,
  handleChange,
}: {
  members?: TeamMember[]
  newEmail?: string
  title: string
  name: string
  handleAdd: () => void
  handleRemove: (event: React.MouseEvent<HTMLButtonElement>) => void
  handleChange: (event: React.ChangeEvent<HTMLInputElement>) => void
}): JSX.Element => {
  /**
   * Wrapper for the handleAdd function that accepts a keyboard event argument
   *  to prevent the event from bubbling up to the form and causing the form to
   *  submit if the enter key is pressed. Instead, when the enter key is pressed,
   *  this adds the new email to the list of member emails in the team edit form.
   * @param {KeyboardEvent} event keyboard event to check if enter key was pressed.
   */
  const handleAddWrapper = (
    event: React.KeyboardEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    if (event.key === 'Enter') {
      event?.preventDefault()
      handleAdd()
    }
  }

  return (
    <>
      <Typography
        sx={{ textTransform: 'capitalize' }}
        gutterBottom
        variant="h5"
      >
        {title}:
      </Typography>
      <Grid container spacing={1} sx={{ mb: 3 }}>
        {members.map(({ email }, index) => (
          <Grid item xs={12} key={index}>
            <FormControl fullWidth variant="standard" disabled margin="none">
              <TeamMemberReadOnly
                index={index}
                email={email}
                handleRemove={handleRemove}
              />
            </FormControl>
          </Grid>
        ))}
        <Grid item xs={12}>
          <FormControl fullWidth variant="standard" size="small">
            <InputLabel htmlFor={`${name}`}>
              <Typography sx={{ textTransform: 'capitalize' }}>
                Add email
              </Typography>
            </InputLabel>
            <Input
              autoComplete="off"
              margin="none"
              id={`${name}`}
              name={`${name}`}
              required
              fullWidth
              onChange={handleChange}
              value={newEmail}
              onKeyDown={handleAddWrapper}
              endAdornment={
                <InputAdornment position="end" sx={{ pr: 1 }}>
                  <IconButton
                    aria-label="add"
                    data-value={newEmail}
                    onClick={handleAdd}
                    onMouseDown={handleAdd}
                    edge="end"
                  >
                    <AddCircleIcon />
                  </IconButton>
                </InputAdornment>
              }
            />
          </FormControl>
        </Grid>
      </Grid>
    </>
  )
}

const TeamForm = () => {
  const { setAlert } = useAlert()
  const {
    data: { teams },
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
    evt: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = evt.currentTarget
    setFormInput({ [name]: value })
  }

  // function that handles removing a team member from the form state
  const handleRemoveTeamMember = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    const email = event.currentTarget.dataset.value
    const members = formInput?.members?.filter((m) => m.email !== email)
    setFormInput({ members })
  }

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
    event.preventDefault()
    const abortController = new AbortController()

    try {
      // get the user's info and token from Amplify Auth
      const [cognitoUser, token]: [CognitoUserInfo, string] = await Promise.all(
        [
          Auth.currentUserInfo(),
          Auth.currentSession().then((session) =>
            session.getIdToken().getJwtToken()
          ),
        ]
      )

      const {
        attributes: { email },
      } = cognitoUser

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
      setTeams([...teams, newTeamData])
      setAlert({
        message: 'Team updated successfully',
        severity: 'success',
      })
    } catch (error) {
      setAlert({
        message: 'Something went wrong, unable to update team!',
        severity: 'error',
      })
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
              <UserSearchInput
                label="Add new User"
                name="newUserSearch"
                control={control}
              />
            </Grid>
          </Grid>
          <Grid container spacing={6} className="match-height">
            <Grid item xs={12} md={12}>
              <TeamMembersTable
                members={(formInput?.members || []) as TableBodyRowType[]}
              />
            </Grid>
          </Grid>
          <Box sx={{ display: 'flex', justifyContent: 'flex-end' }}>
            <Button type="submit" sx={{ mt: 3, ml: 1 }} variant="contained">
              Save
            </Button>
          </Box>
        </Box>
      </Paper>
    </Container>
  )
}

export default TeamForm
