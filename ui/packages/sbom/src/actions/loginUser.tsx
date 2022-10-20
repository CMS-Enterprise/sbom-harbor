import React from 'react'
import { Auth } from '@aws-amplify/auth'
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
      console.debug(`Login successful for ${payload.email}`)
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
