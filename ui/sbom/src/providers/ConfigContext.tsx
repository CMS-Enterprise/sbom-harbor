import * as React from 'react'
import { AppConfig } from '@/utils/types'

const INITIAL_STATE = {
  AWS_REGION: '',
  API_URL: '',
  TEAMS_API_URL: '',
  CF_DOMAIN: '',
  USER_POOL_ID: '',
  USER_POOL_CLIENT_ID: '',
} as AppConfig

const ConfigContext = React.createContext<AppConfig>(INITIAL_STATE)

export const ConfigProvider = ({
  children,
  initialState,
}: {
  children: JSX.Element
  initialState: AppConfig
}) => {
  const [data] = React.useState<AppConfig>(initialState)

  return (
    <ConfigContext.Provider value={data}>{children}</ConfigContext.Provider>
  )
}

export const useConfig = () => React.useContext(ConfigContext)
