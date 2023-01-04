import * as React from 'react'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogContentText from '@mui/material/DialogContentText'
import DialogTitle from '@mui/material/DialogTitle'
import Grid from '@mui/material/Grid'
import useCopyToClipboard from '@/hooks/useCopyToClupboard'
import Icon from '@mui/material/Icon'
import CheckCircleIcon from '@mui/icons-material/CheckCircle'

type InputProps = {
  setOpen?: (open: boolean) => void
  token: string
}

const TokenViewDialog = ({ setOpen, token }: InputProps) => {
  const [value, copy, clear] = useCopyToClipboard()

  const [showCheckMark, setShowCheckMark] = React.useState(false)

  React.useEffect(() => {
    // if the value is falsy, return early.
    if (!value) return
    // show the checkmark
    setShowCheckMark(true)
    // set a timeout to hide the checkmark.
    const timeout = setTimeout(() => {
      setShowCheckMark(false)
      clear()
    }, 2000)
    // clear the timeout on unmount.
    return () => clearTimeout(timeout)
  }, [value])

  const handleClose = React.useCallback(() => {
    if (setOpen) setOpen(false)
  }, [])

  const handleCopyToClipboard = React.useCallback(
    (event: React.MouseEvent) => {
      event.preventDefault()
      copy(token)
    },
    [copy, token]
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
        <Box sx={{ mt: 4, width: '100%' }}>
          <Grid
            container
            spacing={0}
            direction="row"
            alignItems="flex-end"
            sx={{ mb: 4 }}
          >
            <Grid item xs={10}>
              <code>
                <pre>{token}</pre>
              </code>
            </Grid>
          </Grid>
          <Grid item xs={12}>
            <Button variant="text" onClick={handleCopyToClipboard}>
              {/* TODO: add icon */}
              Copy to Clipboard
            </Button>
            {showCheckMark && (
              <Icon color="success">
                <CheckCircleIcon />
              </Icon>
            )}
          </Grid>
          <DialogActions>
            <Button variant="outlined" onClick={handleClose}>
              Close
            </Button>
          </DialogActions>
        </Box>
      </DialogContent>
    </>
  )
}

export default TokenViewDialog
