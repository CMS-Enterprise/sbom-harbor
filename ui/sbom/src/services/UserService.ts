/**
 * Service responsible for making teams related API requests, and
 * for mapping the raw API response of teams data to the UI state.
 * @module @cyclonedx/ui/sbom/services/UserService
 *
 */
import { Auth } from '@aws-amplify/auth'
import { USER_API_URL } from '@/utils/constants'
import { CognitoUserInfo, Team } from '@/utils/types'

/**
 * Gets all teams for the current authenticated user.
 * @param {AbortController} [abortController] - Optional cancel request controller.
 * @returns {Promise<Team[]>} A Promise that resolves to an array of teams.
 */
export const getTeams = async (
  abortController?: AbortController
): Promise<Team[]> => {
  const [user, token]: [CognitoUserInfo, string] = await Promise.all([
    Auth.currentUserInfo(),
    Auth.currentSession().then((session) => session.getIdToken().getJwtToken()),
  ])

  // FIXME: "@" character is not allowed in valid URLs, so we need to encode it
  const url = `${USER_API_URL}/teams?user_id=${user.attributes.email}`
  /**
   * @example
   * const url = new URL(USER_API_URL)
   * url.searchParams.append('user_id', user.attributes.email)
   */

  const res = await fetch(url, {
    headers: { Authorization: `Bearer ${token}` },
    method: 'GET',
    signal: abortController?.signal,
  })
  const data = res.json()
  return data
}

export default {
  getTeams,
}
