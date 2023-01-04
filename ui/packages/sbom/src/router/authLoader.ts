/**
 * Auth state loader for react-router data routes.
 * @module @cyclonedx/ui/sbom/loaders/authLoader
 * @see {@link @cyclonedx/ui/sbom/Routes}
 */
import { Auth } from 'aws-amplify'
import { defer } from 'react-router-dom'

const authLoader = async () => {
  const session = Auth.currentSession()
  return defer({
    jwtToken: session.then((session) => {
      const jwtToken = session.getAccessToken().getJwtToken()
      if (!jwtToken) {
        throw new Response('Invalid Session', { status: 401 })
      }
      return jwtToken
    }),
  })
}

export default authLoader
