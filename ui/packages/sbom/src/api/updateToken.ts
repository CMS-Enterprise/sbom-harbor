import harborRequest from '@/utils/harborRequest'

/**
 * Function that makes a request to the Harbor API to delete a token.
 * @param {[AbortController]} abortController - signal to cancel the request.
 * @param {string} jwtToken - JWT token to authenticate the request.
 * @param {string} tokenId - the id of the token to delete.
 * @param {string} teamId - the id of the team to delete the token from.
 * @returns {Promise} - the response from the Harbor API.
 */
const updateToken = async ({
  abortController = new AbortController(),
  jwtToken,
  tokenId,
  teamId,
  token: body,
}: {
  abortController?: AbortController
  jwtToken: string
  teamId: string
  tokenId: string
  token: {
    name?: string
    enabled?: boolean
    expires?: string
  }
}): Promise<Response> =>
  harborRequest({
    path: `/token/${tokenId}?teamId=${teamId}`,
    method: 'PUT',
    jwtToken,
    body,
    signal: abortController.signal,
  })

export default updateToken
