/**
 * Auth state loader for react-router data routes.
 * @module @cyclonedx/ui/sbom/loaders/authLoader
 * @see {@link @cyclonedx/ui/sbom/Routes}
 */
import { defer } from 'react-router-dom'
import getJWT from '@/utils/getJWT'

const authLoader = async () => {
  return defer({
    jwtToken: getJWT(),
  })
}

export default authLoader
