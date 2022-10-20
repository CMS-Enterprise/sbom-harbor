/**
 * @module @cyclonedx/ui/sbom/hooks/useAlert
 */
import * as React from 'react'
import { AlertColor } from '@mui/material/Alert'

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

export const useAlert = () => React.useContext(AlertContext)

export default useAlert
