/**
 * State loader for react-router data routes that require user's all teams.
 * @module @cyclonedx/ui/sbom/loaders/teamsLoader
 * @see {@link @cyclonedx/ui/sbom/Routes}
 */
import { json } from 'react-router-dom'
import { TeamApiResponse } from '@/types'
import authLoader from '@/router/authLoader'
import harborRequest from '@/utils/harborRequest'

const teamsLoader = async ({ request }: { request: Request }) => {
  return json<Promise<TeamApiResponse>>(
    await harborRequest({
      path: `teams`,
      jwtToken: await authLoader(),
      signal: request.signal,
    })
  )
}

export default teamsLoader
