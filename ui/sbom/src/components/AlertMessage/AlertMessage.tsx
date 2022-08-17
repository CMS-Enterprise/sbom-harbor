import * as React from 'react'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import CloseIcon from '@mui/icons-material/Close'
import Collapse from '@mui/material/Collapse'
import IconButton from '@mui/material/IconButton'
import { useAlert } from '@/hooks/useAlert'

export const AlertMessage = (): JSX.Element => {
  const { clearAlert, state } = useAlert()

  return (
    <>
      <Box
        sx={{
          zIndex: 10000,
          width: '100%',
          maxWidth: '500px',
          right: '-1%',
          top: '1%',
          position: 'fixed',
        }}
      >
        <Collapse in={state.isVisible}>
          <Alert
            severity={state.severity}
            action={
              <IconButton
                aria-label="close"
                color="inherit"
                size="small"
                onClick={clearAlert}
              >
                <CloseIcon fontSize="inherit" />
              </IconButton>
            }
            sx={{
              transform: 'translateY(20px)',
              minWidth: '50%',
              maxWidth: '80%',
              margin: 'auto',
            }}
            elevation={24}
          >
            {state.message}
          </Alert>
        </Collapse>
      </Box>
    </>
  )
}
