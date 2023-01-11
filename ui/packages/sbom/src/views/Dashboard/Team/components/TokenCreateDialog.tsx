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
import createToken from '@/api/createToken'
import useAlert from '@/hooks/useAlert'
import { useAuthState } from '@/hooks/useAuth'
import { useDialog } from '@/hooks/useDialog'
import formatTimestampForServer from '@/utils/formatTimestampForServer'
import TokenViewDialog from '@/views/Dashboard/Team/components/TokenViewDialog'
import { Token } from '@/types'

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
  onTokenAdded: (token: Token) => void
}

const TokenCreateDialog = ({ setOpen, teamId, onTokenAdded }: InputProps) => {
  const { jwtToken } = useAuthState()
  const { setAlert } = useAlert()
  const [openDialog] = useDialog()
  const [loading, setLoading] = React.useState(false)

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

  const handleClose = React.useCallback(
    () => {
      if (setOpen) setOpen(false)
    },
    /* eslint-disable react-hooks/exhaustive-deps */
    []
    /* eslint-enable react-hooks/exhaustive-deps */
  )

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
      const abortController = new AbortController()

      /**
       * Async function to submit the form.
       * @type {() => Promise<void>}
       */
      const doSubmit = async () => {
        try {
          // set the loading state to true
          setLoading(true)
          // calculate the expiration date for the token.
          const expires =
            ExpirationValues[
              Object.entries(ExpirationOptions).find(
                ([, value]) => value === formInput.expires
              )?.[0] as keyof typeof ExpirationValues
            ]()
          // make the create token request
          const response = await createToken({
            teamId,
            jwtToken,
            expires,
            name: formInput.name,
          })
          // throw an error if the request failed
          if (!response.ok) {
            throw new Error('Failed to create token')
          }
          // parse the response as json
          const data = await response.json()
          // add the token to the list of tokens in the table
          onTokenAdded(data)
          // show a success message
          setAlert({
            message: 'Token created successfully!',
            severity: 'success',
          })
          // set the loading state to false
          setLoading(false)
          // open the next dialog window to show the token
          // to let the user to copy it to the clipboard.
          openDialog({
            children: <TokenViewDialog token={data.token} />,
          })
        } catch (error) {
          console.error('Error creating token', error)
          setLoading(false)
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
    /* eslint-disable react-hooks/exhaustive-deps */
    [formInput]
    /* eslint-enable react-hooks/exhaustive-deps */
  )

  const ExpirationValues = React.useMemo(
    () => ({
      SEVEN_DAYS: () => formatTimestampForServer(7),
      THIRTY_DAYS: () => formatTimestampForServer(30),
      SIXTY_DAYS: () => formatTimestampForServer(60),
      NINETY_DAYS: () => formatTimestampForServer(90),
      SIX_MONTHS: () => formatTimestampForServer(182),
      ONE_YEAR: () => formatTimestampForServer(365),
      // TODO: implement datepicker for custom expiration
      // CUSTOM = 'Custom',
    }),
    []
  )

  const expirationDropdownItems = React.useMemo(
    () => Object.entries(ExpirationOptions),
    []
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
                  {expirationDropdownItems.map(([key, option]) => (
                    <MenuItem key={key} value={option}>
                      {option}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>
          </Grid>
          <DialogActions>
            <Button variant="contained" type="submit" disabled={loading}>
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
