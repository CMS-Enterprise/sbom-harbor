import * as React from 'react'
import { CognitoUser } from '@aws-amplify/auth'

export type IAuthContext = {
  user: CognitoUser | null | undefined
  setUser: React.Dispatch<React.SetStateAction<CognitoUser | null | undefined>>
}

export const AuthContext = React.createContext<IAuthContext>({
  user: undefined,
  setUser: () => undefined,
})
