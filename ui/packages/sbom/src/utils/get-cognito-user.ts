import { Auth } from '@aws-amplify/auth'
import {
  CognitoUser,
  CognitoIdToken,
  CognitoUserSession,
} from 'amazon-cognito-identity-js'
import { CognitoUserInfo } from '@/utils/types'

type UserDataType = {
  user: CognitoUser
  userInfo: CognitoUserInfo
  userSession: CognitoUserSession
  idToken: CognitoIdToken
}

export const getUserData = async (): Promise<UserDataType> => {
  // get the current user's data and id token from Cognito.
  const [user, userInfo, userSession]: [
    CognitoUser,
    CognitoUserInfo,
    CognitoUserSession
  ] = await Promise.all([
    Auth.currentAuthenticatedUser(),
    Auth.currentUserInfo(),
    Auth.currentSession(),
  ])

  return {
    user,
    userInfo,
    userSession,
    idToken: userSession.getIdToken(),
  }
}

export default getUserData
