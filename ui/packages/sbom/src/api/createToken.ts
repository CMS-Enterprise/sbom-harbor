import harborRequest from '@/utils/harborRequest'

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
  expires: TDateISO
}): Promise<Response> =>
  harborRequest({
    body: { name, expires },
    jwtToken,
    method: 'POST',
    path: `/token?teamId=${teamId}`,
    signal: abortController.signal,
  })

export default createToken
