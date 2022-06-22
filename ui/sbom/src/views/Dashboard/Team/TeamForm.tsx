import * as React from 'react'
import { ReactComponent as AddCircleOutlineIcon } from '@/assets/icons/AddCircleOutline.svg'
import Grid from '@mui/material/Grid'
import IconButton from '@mui/material/IconButton'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'

type FormState = {
  teamName?: string
  newMemberEmail?: string
}

const defaultState = {
  teamName: '',
  newMemberEmail: '',
}

export default function AddressForm() {
  const [teamMembers, setTeamMembers] = React.useState<Array<string>>([])

  const [formInput, setFormInput] = React.useReducer(
    (state: FormState, newState: FormState) => ({ ...state, ...newState }),
    defaultState
  )

  const handleInput = (
    evt: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const name = evt.currentTarget.name
    const newValue = evt.currentTarget.value
    setFormInput({ [name]: newValue })
  }

  return (
    <React.Fragment>
      <Typography variant="h6" gutterBottom>
        Create your team:
      </Typography>
      <Grid container spacing={3} sx={{ mb: 6 }}>
        <Grid item xs={12}>
          <TextField
            required
            id="teamName"
            name="teamName"
            label="Team Name"
            fullWidth
            variant="standard"
            value={formInput.teamName}
            onChange={handleInput}
          />
        </Grid>
      </Grid>
      <Typography variant="h6" gutterBottom>
        Add team members:
      </Typography>
      <Grid container spacing={1} sx={{ mb: 3 }}>
        {teamMembers.map((teamMember, index) => (
          <Grid item xs={12} key={index}>
            <TextField
              id={`member-${index}`}
              name={`member-${index}`}
              value={teamMember}
              disabled
              fullWidth
              variant="standard"
              inputProps={{
                style: {
                  color: 'white !important',
                  WebkitTextFillColor: 'rgba(255, 255, 255, 1)',
                },
              }}
            />
          </Grid>
        ))}
        <Grid item xs={12}>
          <TextField
            id="newMemberEmail"
            name="newMemberEmail"
            label="Team Member Email"
            required
            fullWidth
            variant="standard"
            onChange={handleInput}
            value={formInput.newMemberEmail}
            onKeyDown={(evt) => {
              if (evt.key === 'Enter' && formInput.newMemberEmail) {
                setTeamMembers([...teamMembers, formInput.newMemberEmail])
                setFormInput({ newMemberEmail: '' })
              }
            }}
          />
          <IconButton
            aria-label="add"
            onClick={() => {
              setTeamMembers([...teamMembers])
            }}
          >
            <AddCircleOutlineIcon />
          </IconButton>
        </Grid>
      </Grid>
    </React.Fragment>
  )
}
