/**
 * Service responsible for making teams related API requests, and
 * for mapping the raw API response of teams data to the UI state.
 * @module @cyclonedx/ui/sbom/services/UserService
 *
 */
import { Auth } from '@aws-amplify/auth'
import { CONFIG } from '@/utils/constants'
import { Team } from '@/types'

// XXX: move this functionality into an auth reducer

/**
 * Gets all teams for the current authenticated user.
 * @param {AbortController} [abortController] - Optional cancel request controller.
 * @returns {Promise<Team[]>} A Promise that resolves to an array of teams.
 */
export const getTeams = async (
  abortController?: AbortController
): Promise<Team[]> => {
  const session = await Auth.currentSession()
  const jwtToken = session.getIdToken().getJwtToken()

  const url = `${CONFIG.USER_API_URL}/teams`
  /**
   * @example
   * const url = new URL(USER_API_URL)
   * url.searchParams.append('user_id', user.attributes.email)
   */

  const res = await fetch(url, {
    headers: { Authorization: `Bearer ${jwtToken}` },
    method: 'GET',
    signal: abortController?.signal,
  })
  const data = res.json()
  return data
}

export default {
  getTeams,
}
