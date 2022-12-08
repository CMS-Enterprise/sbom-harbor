import React from 'react'
import { Auth } from 'aws-amplify'
import { LoginParams } from '@/hooks/types'

export default async function loginUser(
  dispatch: React.Dispatch<any>,
  payload: LoginParams
) {
  try {
    dispatch({ type: 'REQUEST_LOGIN' })
    const user = await Auth.signIn(payload.email, payload.password)
    const jwtToken = await (await Auth.currentSession())
      .getAccessToken()
      .getJwtToken()

    if (jwtToken) {
      const data = {
        jwtToken,
        email: user.attributes.email,
        teams: user.attributes['custom:teams'],
        username: user.getUsername(),
      }
      dispatch({ type: 'LOGIN_SUCCESS', payload: data })
      return data
    }

    return
  } catch (error) {
    console.warn(`Login failed for ${payload.email}`, error)
    dispatch({ type: 'LOGIN_ERROR', error: error })
  }
}
