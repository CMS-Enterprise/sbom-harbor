import * as React from 'react'
import { CognitoUser } from '@aws-amplify/auth'
import { ICognitoUserSessionData } from 'amazon-cognito-identity-js'

export type User = {
  email: string
  familyName: string
  givenName: string
  picture?: string
  phoneNumber?: string
  country?: string
  city?: string
  address?: string
  isAdmin?: boolean
}

export type SessionType = ICognitoUserSessionData | null | undefined

export type SessionContextType = {
  user: CognitoUser | null | undefined
  setUser: React.Dispatch<React.SetStateAction<CognitoUser | null | undefined>>
}

export const SessionContext = React.createContext<SessionContextType>({
  user: undefined,
  setUser: () => undefined,
})
