/**
 * @module @cyclonedx/ui/sbom/hooks/useData
 */
import * as React from 'react'
import { AppState, Team } from '@/types'

const INITIAL_STATE = {
  teams: [],
  setTeams: () => null,
  setData: () => null,
} as AppState

const DataContext = React.createContext<{
  data: AppState
  setData: (values: AppState) => void
  setTeams: (teams: Team[]) => void
}>({
  data: INITIAL_STATE,
  setData: () => ({}),
  setTeams: () => ({}),
})

export const DataProvider = ({
  children,
  initialState = INITIAL_STATE,
}: {
  children: JSX.Element
  initialState?: AppState
}) => {
  // XXX: make this into a reducer
  const [data, dispatchSetData] = React.useState<AppState>(initialState)

  const setData = (values: AppState) => {
    dispatchSetData((prevData) => ({
      ...prevData,
      ...values,
    }))
  }

  // dispatches update to the user data state in the context provider.
  const setTeams = (teams: Team[] = []) => {
    setData({ teams })
  }

  const value = {
    data,
    setData,
    setTeams,
  }

  return <DataContext.Provider value={value}>{children}</DataContext.Provider>
}

export const useData = () => React.useContext(DataContext)
