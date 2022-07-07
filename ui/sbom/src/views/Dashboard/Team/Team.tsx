import * as React from 'react'
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
import AddCircleIcon from '@mui/icons-material/AddCircle'
import RemoveCircleIcon from '@mui/icons-material/RemoveCircleOutlineTwoTone'
import { useData } from '@/providers/DataContext'
import { Team, TeamMember } from '@/utils/types'
import { useConfig } from '@/providers/ConfigContext'
import { AuthContext } from '@/providers/AuthContext'

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
              <Input
                id={`member-${index}`}
                name={`member-${index}`}
                disabled
                fullWidth
                value={email}
                inputProps={{
                  style: {
                    color: 'white !important',
                    WebkitTextFillColor: 'rgba(255, 255, 255, 1)',
                  },
                }}
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
              onKeyDown={(evt) => {
                if (evt.key === 'Enter') handleAdd()
              }}
              endAdornment={
                <InputAdornment position="end" sx={{ pr: 1 }}>
                  <IconButton
                    aria-label="add"
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
  // get the Teams API url from readonly ConfigContext
  const { TEAMS_API_URL } = useConfig()

  // use the session context to get the user
  const { user } = React.useContext(AuthContext)

  // get the global app data from the DataContext
  const {
    data,
    data: { teams: [team = {} as Team] = [] },
    setValues,
  } = useData()

  const [formInput, setFormInput] = React.useReducer(
    (state: FormState, newState: FormState) => ({ ...state, ...newState }),
    defaultFormState
  )

  React.useEffect(() => {
    if (team?.members?.length) {
      const { members = [], Id = '' } = team
      setFormInput({ members, Id })
    }
  }, [team])

  const handleInput = (
    evt: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const name = evt.currentTarget.name
    const newValue = evt.currentTarget.value
    setFormInput({ [name]: newValue })
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

  const handleRemoveTeamMember = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    const email = event.currentTarget.dataset.value
    const members = formInput?.members?.filter((m) => m.email !== email)
    setFormInput({ members })
  }

  const handleSubmitForm = async (event: React.FormEvent<HTMLFormElement>) => {
    // prevent the default for submission behavior
    event.preventDefault()

    // create abort controller to cancel the request if the component unmounts
    const abortController = new AbortController()

    try {
      // get the team id and the final list of members from the form
      const { Id = team.Id, members = team.members } = formInput

      // create the new team object to send to the API
      const newTeamData = { ...team, Id, members }

      // get the bearer token from the session context
      const token = user
        ?.getSignInUserSession()
        ?.getAccessToken()
        ?.getJwtToken()

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
      setValues({ ...data, teams: [newTeamData] })
    } catch (error) {
      // TODO: show error message to the user and handle the error gracefully
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
        <Typography variant="h4" gutterBottom>
          Your Team
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
