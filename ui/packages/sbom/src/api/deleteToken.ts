import harborRequest from '@/utils/harborRequest'

/**
 * Function that makes a request to the Harbor API to delete a token.
 * @param {[AbortController]} abortController - signal to cancel the request.
 * @param {string} jwtToken - JWT token to authenticate the request.
 * @param {string} tokenId - the id of the token to delete.
 * @param {string} teamId - the id of the team to delete the token from.
 * @returns {Promise} - the response from the Harbor API.
 */
const deleteToken = async ({
  abortController = new AbortController(),
  jwtToken,
  tokenId,
  teamId,
}: {
  abortController?: AbortController
  jwtToken: string
  teamId: string
  tokenId: string
}): Promise<Response> =>
  harborRequest({
    body: {},
    jwtToken,
    method: 'DELETE',
    path: `/token/${tokenId}?teamId=${teamId}`,
    signal: abortController.signal,
  })

export default deleteToken
