/**
 * Auth state loader for react-router data routes.
 * @module @cyclonedx/ui/sbom/loaders/authLoader
 * @see {@link @cyclonedx/ui/sbom/Routes}
 */
import { Auth } from 'aws-amplify'

const authLoader = async () => {
  const session = await Auth.currentSession()
  const jwtToken = session.getAccessToken().getJwtToken()
  if (!jwtToken) {
    throw new Response('Invalid Session', { status: 401 })
  }
  return jwtToken
}

export default authLoader
