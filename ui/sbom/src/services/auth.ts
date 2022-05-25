import * as React from 'react'
import { Auth, CognitoUser } from '@aws-amplify/auth'
import { CognitoUserSession } from 'amazon-cognito-identity-js'

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

export const login = (
  username: string,
  password: string
): Promise<CognitoUser> => Auth.signIn(username, password)

export const logout = (): Promise<any> => Auth.signOut()

export const getSession = (): Promise<CognitoUserSession | null> =>
  Auth.currentSession()

export const SessionContext = React.createContext<
  CognitoUserSession | null | undefined
>(undefined)
