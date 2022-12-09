/**
 * @module @cyclonedx/ui/sbom/hooks/useData
 */
import * as React from 'react'
import { useAuthState } from '@/hooks/useAuth'
import { CONFIG } from '@/utils/constants'
import { AppState, TeamModel } from '@/types'

const INITIAL_STATE = {
  teams: [] as TeamModel[],
  setData: () => null,
  fetchTeams: () => Promise<null>,
  setTeams: () => null,
} as AppState

const DataContext = React.createContext<{
  data: AppState
  fetchTeams: (controller: AbortController) => Promise<TeamModel[]>
  setData: (values: AppState) => void
  setTeams: (teams: TeamModel[]) => void
}>({
  data: INITIAL_STATE,
  fetchTeams: () => Promise.resolve([]),
  setData: () => ({}),
  setTeams: () => [],
})

// TODO: make this into a reducer
export const DataProvider = ({
  children,
  initialState = INITIAL_STATE,
}: {
  children: JSX.Element
  initialState?: AppState
}) => {
  const { jwtToken } = useAuthState()
  const [data, dispatchSetData] = React.useState<AppState>(initialState)

  const setData = (values: AppState) => {
    dispatchSetData((prevData) => ({
      ...prevData,
      ...values,
    }))
  }

  // dispatches update to the user data state in the context provider.
  const setTeams = React.useCallback((teams: TeamModel[] = []) => {
    setData({ ...data, teams })
  }, [])

  const fetchTeams = React.useCallback(
    async (controller: AbortController) => {
      if (!jwtToken) {
        console.warn('useData#fetchTeams', 'No user or user token found.')
        return
      }

      const url = new URL(`${CONFIG.API_URL}/v1/teams`)
      url.searchParams.append('children', 'true')

      const teams = await fetch(url, {
        signal: controller.signal,
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${jwtToken}`,
        },
      })

      const data = await teams.json()
      return data
    },
    [jwtToken]
  )

  const value = React.useMemo(() => {
    return {
      data,
      fetchTeams,
      setData,
      setTeams,
    }
  }, [data, fetchTeams, setTeams])

  return <DataContext.Provider value={value}>{children}</DataContext.Provider>
}

export const useData = () => React.useContext(DataContext)
