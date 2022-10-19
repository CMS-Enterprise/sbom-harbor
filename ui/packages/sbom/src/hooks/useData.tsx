/**
 * @module @cyclonedx/ui/sbom/hooks/useData
 */
import * as React from 'react'
import { AppState, Team, UserDataType, UserState } from '@/types'

// DEBUG: remove this when we fetch from the real api
import data from '@/data.json'
import { CONFIG } from '@/utils/constants'
import useAuth from './useAuth'

// DEBUG: remove this when we fetch from the real api
const {
  devData: { teams: teamsDevData },
} = data

const INITIAL_STATE = {
  teams: teamsDevData,
  fetchTeams: () => Promise<null>,
  setTeams: () => null,
  setData: () => null,
} as AppState

const DataContext = React.createContext<{
  data: AppState
  fetchTeams: (controller: AbortController) => Promise<Record<string, Team>>
  setData: (values: AppState) => void
  setTeams: (teams: Record<string, Team>) => void
}>({
  data: INITIAL_STATE,
  fetchTeams: () => Promise.resolve({}),
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
  const { user } = useAuth()

  const setData = (values: AppState) => {
    dispatchSetData((prevData) => ({
      ...prevData,
      ...values,
    }))
  }

  // dispatches update to the user data state in the context provider.
  const setTeams = (teams = {}) => {
    setData({ teams })
  }

  const fetchTeams = async (controller: AbortController) => {
    if (!user || !user?.jwtToken) {
      console.error('useData#fetchTeams', 'No user or user token found.')
      return
    }
    const url = new URL(`${CONFIG.API_URL}/v1/teams`)
    url.searchParams.append('children', 'true')

    const teams = await fetch(url, {
      signal: controller.signal,
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `${user?.jwtToken}`,
      },
    })

    const data = await teams.json()
    return data
  }

  const value = {
    data,
    fetchTeams,
    setData,
    setTeams,
  }

  return <DataContext.Provider value={value}>{children}</DataContext.Provider>
}

export const useData = () => React.useContext(DataContext)
