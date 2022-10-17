/**
 * Helper function that returns user details from Cognito Auth.
 * @module @cyclonedx/ui/sbom/utils/get-cognito-user
 */
import { Auth, CognitoUser } from '@aws-amplify/auth'
import { CognitoUserSession } from 'amazon-cognito-identity-js'
import { UserState, CognitoUserInfo } from '@/types'

export const getUserData = async (): Promise<UserState> => {
  // get the current user's data and id token from Cognito.
  const [user, userSession, userInfo]: [
    CognitoUser,
    CognitoUserSession,
    CognitoUserInfo
  ] = await Promise.all([
    Auth.currentAuthenticatedUser(),
    Auth.currentSession(),
    Auth.currentUserInfo(),
  ])

  return {
    user,
    userInfo,
    userSession,
    idToken: userSession.getIdToken(),
    jwtToken: userSession.getAccessToken().getJwtToken(),
  }
}

export default getUserData
