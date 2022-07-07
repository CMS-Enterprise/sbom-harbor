import * as React from 'react'
import { CognitoUserSession } from 'amazon-cognito-identity-js'
import { AppState } from '@/utils/types'

const INITIAL_STATE = {
  teams: [],
  user: {} as CognitoUserSession,
} as AppState

const DataContext = React.createContext<{
  data: AppState
  setValues: (values: AppState) => void
}>({
  data: INITIAL_STATE,
  setValues: () => ({}),
})

export const DataProvider = ({
  children,
  initialState = INITIAL_STATE,
}: {
  children: JSX.Element
  initialState?: AppState
}) => {
  const [data, setData] = React.useState<AppState>(initialState)

  const setValues = (values: AppState) => {
    setData((prevData) => ({
      ...prevData,
      ...values,
    }))
  }

  return (
    <DataContext.Provider value={{ data, setValues }}>
      {children}
    </DataContext.Provider>
  )
}

export const useData = () => React.useContext(DataContext)
