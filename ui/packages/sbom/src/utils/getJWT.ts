import { Auth } from 'aws-amplify'

/**
 * Get the Cognito JWT Token from the current session.
 * @module @cyclonedx/ui/sbom/utils/getJWT
 *
 * @returns {Promise<string>} The Cognito JWT Token.
 */
const getJWT = (): Promise<string> =>
  Auth.currentSession().then((session) => {
    const jwtToken = session.getAccessToken().getJwtToken()

    if (!jwtToken) {
      throw new Response('Invalid Session', { status: 401 })
    }

    return jwtToken
  })

export default getJWT
