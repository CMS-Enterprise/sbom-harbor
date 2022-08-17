import * as React from 'react'
import Alert, { AlertColor } from '@mui/material/Alert'
import Box from '@mui/material/Box'
import CloseIcon from '@mui/icons-material/Close'
import Collapse from '@mui/material/Collapse'
import IconButton from '@mui/material/IconButton'

const DEFAULT_TIMEOUT = 3000

type AlertProps = {
  severity?: AlertColor
  message?: string
  autoHide?: boolean
  timeout?: number
}

type AlertState = {
  isVisible: boolean
  message: string
  severity: AlertColor
}

const INITIAL_STATE = {
  isVisible: false,
  message: '',
  severity: 'info' as AlertColor,
}

const AlertContext = React.createContext<{
  state: AlertState
  clearAlert: () => void
  setAlert: (values: AlertProps) => void
  setData: (values: AlertState) => void
}>({
  state: INITIAL_STATE,
  setData: () => ({}),
  setAlert: () => ({}),
  clearAlert: () => ({}),
})

export const AlertProvider = ({
  children,
  initialState = INITIAL_STATE,
}: {
  children: JSX.Element
  initialState?: AlertState
}) => {
  const [state, setState] = React.useState<AlertState>(initialState)

  const clearAlert = () => {
    setState({
      ...state,
      isVisible: false,
      message: '',
      severity: 'info' as AlertColor,
    })
  }

  const setAlert = ({
    severity = 'info',
    message = '',
    autoHide = true,
    timeout = DEFAULT_TIMEOUT,
  }: AlertProps) => {
    console.log(severity, message, state)
    setState({
      ...state,
      isVisible: true,
      message,
      severity,
    })
    if (!autoHide) return
    setTimeout(clearAlert, timeout)
  }

  const setData = (values: AlertState) => {
    console.log('setData', values)
    setState((prevData: AlertState) => ({
      ...prevData,
      ...values,
    }))
  }

  return (
    <AlertContext.Provider value={{ state, setData, setAlert, clearAlert }}>
      {children}
    </AlertContext.Provider>
  )
}

export const AlertMessage = (): JSX.Element => {
  const { state, clearAlert } = React.useContext(AlertContext)

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

export const useAlert = () => React.useContext(AlertContext)
