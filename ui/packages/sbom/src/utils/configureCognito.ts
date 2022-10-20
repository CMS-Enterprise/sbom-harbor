import { Auth } from '@aws-amplify/auth'
import { CONFIG } from '@/utils/constants'

/**
 * Initializes and configures Amazon Cognito authentication.
 */
export function configureCognito() {
  Auth.configure({
    region: CONFIG.AWS_REGION,
    userPoolId: CONFIG.USER_POOL_ID || new Error('USER_POOL_ID is not defined'),
    userPoolWebClientId:
      CONFIG.USER_POOL_CLIENT_ID ||
      new Error('USER_POOL_CLIENT_ID is not defined'),
  })
}

export default configureCognito
