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
import { useDialog } from '@/hooks/useDialog'
import TokenViewDialog from '@/views/Dashboard/Team/components/TokenViewDialog'
import createToken from '@/api/createToken'
import authLoader from '@/router/authLoader'
import useAlert from '@/hooks/useAlert'
import dateAsISOWithoutZ from '@/utils/dateAsISOWithoutZ'

enum ExpirationOptions {
  SEVEN_DAYS = '7 days',
  THIRTY_DAYS = '30 days',
  SIXTY_DAYS = '60 days',
  NINETY_DAYS = '90 days',
  SIX_MONTHS = '6 months',
  ONE_YEAR = '1 year',
  // TODO: implement datepicker for custom expiration
  // CUSTOM = 'Custom',
}

const getExpirationDate = (daysToAdd: number): TDateISOWithoutZ => {
  const date = new Date()
  return dateAsISOWithoutZ(new Date(date.setDate(date.getDate() + daysToAdd)))
}

// FIXME: copy pasta shame
const ExpirationValues = {
  SEVEN_DAYS: () => getExpirationDate(7),
  THIRTY_DAYS: () => getExpirationDate(30),
  SIXTY_DAYS: () => getExpirationDate(60),
  NINETY_DAYS: () => getExpirationDate(90),
  SIX_MONTHS: () => getExpirationDate(182),
  ONE_YEAR: () => getExpirationDate(365),
  // TODO: implement datepicker for custom expiration
  // CUSTOM = 'Custom',
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
  const [openDialog] = useDialog()
  const { setAlert } = useAlert()

  /**
   * React reducer hook to manage the form input state.
   */
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

  /**
   * React callback hook to handle the expiration select change.
   */
  const handleExpiresChange = React.useCallback(
    (event: SelectChangeEvent<typeof formInput.expires>) => {
      setFormInput({
        ...formInput,
        expires: event.target.value,
      })
    },
    [formInput]
  )

  /**
   * React callback hook to handle the form input change.
   */
  const handleFormChange = React.useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      setFormInput({
        ...formInput,
        [event.target.name]: event.target.value,
      })
    },
    [formInput]
  )

  /**
   * React callback hook to handle the submission of the form.
   */
  const handleSubmit = React.useCallback(
    (event: React.FormEvent<HTMLFormElement>) => {
      event.preventDefault()
      // create an abort controller to cancel the
      // request if the user closes the dialog.
      const abortController = new AbortController()
      /**
       * Async function to submit the form.
       * @type {() => Promise<void>}
       */
      const doSubmit = async () => {
        try {
          // get the jwt token from the auth loader
          const jwtToken = await authLoader()
          /**
           * Calculate the expiration date for the token.
           * @type {TDateISOString}
           */
          const expires =
            ExpirationValues[
              Object.entries(ExpirationOptions).find(
                ([, value]) => value === formInput.expires
              )?.[0] as keyof typeof ExpirationValues
            ]()

          const params = {
            teamId,
            jwtToken,
            expires,
            name: formInput.name,
          }

          // make the create token request
          const response = await createToken(params)

          // show a success message
          setAlert({
            message: 'Token created successfully!',
            severity: 'success',
          })
          // open the next dialog window to show the token
          // to let the user to copy it to the clipboard.
          openDialog({ children: <TokenViewDialog token={response.token} /> })
        } catch (error) {
          console.error('Error creating token', error)
          setAlert({
            message: 'Failed to create a token',
            severity: 'error',
          })
        }
      }
      // actually submit the form asyncronously.
      doSubmit()
      // return a cleanup function to abort the submit request.
      return () => abortController.abort()
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
