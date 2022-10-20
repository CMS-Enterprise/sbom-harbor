import { Auth } from '@aws-amplify/auth'
import React from 'react'

export default async function logoutUser(dispatch: React.Dispatch<any>) {
  await Auth.signOut()
  dispatch({ type: 'LOGOUT' })
}
