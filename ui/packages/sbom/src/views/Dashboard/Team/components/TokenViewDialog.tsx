import * as React from 'react'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogContentText from '@mui/material/DialogContentText'
import DialogTitle from '@mui/material/DialogTitle'
import Grid from '@mui/material/Grid'
import { Token } from '@/types'

type InputProps = {
  setOpen?: (open: boolean) => void
  token: Token
}

const TokenViewDialog = ({ setOpen, token }: InputProps) => {
  const handleClose = React.useCallback(() => {
    if (setOpen) setOpen(false)
  }, [])

  const handleCopyToClipboard = React.useCallback((event: React.MouseEvent) => {
    event.preventDefault()
    console.log('copy to clipboard')
  }, [])

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
            <Grid item xs={12}>
              <Alert
                severity="success"
                sx={{
                  transform: 'translateY(20px)',
                  minWidth: '50%',
                  maxWidth: '80%',
                  margin: 'auto',
                }}
                elevation={24}
              >
                Token created successfully
              </Alert>
            </Grid>
            <Grid item xs={10}>
              <code>
                <pre>{token.token}</pre>
              </code>
            </Grid>
          </Grid>
          <Grid item xs={12}>
            <Button variant="text" onClick={handleCopyToClipboard}>
              {/* TODO: add icon */}
              Copy to clipboard
            </Button>
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
