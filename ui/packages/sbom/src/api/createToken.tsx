import harborRequest from '@/utils/harborRequest'
import { Token } from '@/types'

/**
 * Function that makes a request to the Harbor API to create a token.
 * @param {[AbortController]} abortController - signal to cancel the request.
 * @param {string} jwtToken - JWT token to authenticate the request.
 * @param {string} teamId - the id of the team for which to create the token.
 * @returns {Promise} - the response from the Harbor API.
 */
const createToken = async ({
  abortController = new AbortController(),
  jwtToken,
  teamId,
  name = '',
  expires,
}: {
  abortController?: AbortController
  jwtToken: string
  teamId: string
  name: string
  expires: TDateISOWithoutZ
}): Promise<Response> =>
  harborRequest({
    path: `/token?teamId=${teamId}`,
    method: 'POST',
    jwtToken,
    body: { name, expires },
    signal: abortController.signal,
  })

export default createToken
