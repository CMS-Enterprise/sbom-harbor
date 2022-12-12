import * as React from 'react'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogContentText from '@mui/material/DialogContentText'
import DialogTitle from '@mui/material/DialogTitle'
import FormControl from '@mui/material/FormControl'
import Grid from '@mui/material/Grid'
import InputLabel from '@mui/material/InputLabel'
import MenuItem from '@mui/material/MenuItem'
import Select, { SelectChangeEvent } from '@mui/material/Select'
import TextField from '@mui/material/TextField'

enum ExpirationOptions {
  SEVEN_DAYS = '7 days',
  THIRTY_DAYS = '30 days',
  SIXTY_DAYS = '60 days',
  NINETY_DAYS = '90 days',
  CUSTOM = 'Custom',
  NEVER = 'Never',
}

const expirationOptionsList = Object.values(ExpirationOptions)

type DialogFormState = {
  name: string
  expires: string
}

const defaultFormState = {
  name: '',
  expires: ExpirationOptions.SEVEN_DAYS,
}

type InputProps = {
  setOpen?: (open: boolean) => void
  teamId: string
}

const TokenCreateDialog = ({ setOpen, teamId }: InputProps) => {
  const [formInput, setFormInput] = React.useReducer(
    (state: DialogFormState, newState: DialogFormState) => ({
      ...state,
      ...newState,
    }),
    { ...defaultFormState }
  )

  const handleClose = React.useCallback(() => {
    if (setOpen) setOpen(false)
  }, [])

  const handleExpiresChange = React.useCallback(
    (event: SelectChangeEvent<typeof formInput.expires>) => {
      setFormInput({
        ...formInput,
        expires: event.target.value,
      })
    },
    [formInput]
  )

  const handleFormChange = React.useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      setFormInput({
        ...formInput,
        [event.target.name]: event.target.value,
      })
    },
    [formInput]
  )

  const handleSubmit = React.useCallback(
    (event: React.FormEvent<HTMLFormElement>) => {
      event.preventDefault()
      console.log('Submit TokenCreateDialog', 'formInput', formInput)
    },
    [formInput]
  )

  return (
    <>
      <DialogTitle>New token</DialogTitle>
      <DialogContent>
        <DialogContentText>
          Tokens function as API keys. They allow you to authenticate with the
          API in order to upload SBOMs. You can create a token to be used by a
          specific team member, or by an automated process like a CI/CD
          pipeline.
        </DialogContentText>
        <Box
          component="form"
          onSubmit={handleSubmit}
          sx={{ mt: 4, width: '100%' }}
        >
          <Grid
            container
            spacing={0}
            direction="row"
            alignItems="flex-end"
            sx={{ mb: 4 }}
          >
            <Grid item xs={1}></Grid>
            <Grid item xs={6}>
              <TextField
                autoFocus
                margin="none"
                id="name"
                name="name"
                label="Name"
                value={formInput.name}
                onChange={handleFormChange}
                type="text"
                fullWidth
                required
                InputProps={{
                  placeholder: 'e.g. CI/CD Pipeline',
                  'aria-label': 'Token name',
                }}
                InputLabelProps={{ shrink: true }}
              />
            </Grid>
            <Grid item xs={1}></Grid>
            <Grid item xs={4}>
              <FormControl margin="none" required>
                <InputLabel htmlFor="expires">Expiration</InputLabel>
                <Select
                  autoFocus
                  value={formInput.expires}
                  onChange={handleExpiresChange}
                  label="expires"
                  inputProps={{
                    name: 'expires',
                    id: 'expires',
                  }}
                >
                  {expirationOptionsList.map((option) => (
                    <MenuItem key={option} value={option}>
                      {option}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>
          </Grid>
          <DialogActions>
            <Button variant="contained" type="submit">
              Generate Token
            </Button>
            <Button variant="outlined" onClick={handleClose}>
              Cancel
            </Button>
          </DialogActions>
        </Box>
      </DialogContent>
    </>
  )
}

export default TokenCreateDialog
