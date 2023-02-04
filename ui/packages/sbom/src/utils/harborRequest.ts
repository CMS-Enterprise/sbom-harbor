/**
 * Factory to create a function that will make a request to the Harbor API
 * @module @cyclonedx/ui/sbom/utils/harborRequest
 */
import { CONFIG } from '@/utils/constants'
import sanitizeUrl from '@/utils/sanitizeUrl'

type HarborRequestParams = {
  body?: Record<string, unknown>
  children?: boolean
  jwtToken: string
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE'
  path: string
  signal?: AbortSignal
}

/**
 * Make a request to the Harbor API.
 * @param {[Object]} body Optional request body.
 * @param {[boolean]} children Required value of the children query param.
 * @param {string} jwtToken The Cognito JWT Token.
 * @param {[string='GET']} method Optional HTTP method, defaults to 'GET'.
 * @param {string} path The path to the API endpoint.
 * @param {[AbortSignal]} signal Optional AbortSignal to cancel the request.
 * @returns {Promise<Response>} Promise that resolves to the API response.
 */
const harborRequest = async ({
  body,
  children,
  jwtToken,
  method = 'GET',
  path,
  signal = new AbortController().signal,
}: HarborRequestParams): Promise<Response> => {
  // ensure a jwtToken was provided, otherwise throw an error
  if (!jwtToken || typeof jwtToken === 'undefined') {
    throw new Error('No JWT token provided')
  }

  // create the url object with the path
  const url = sanitizeUrl(`${CONFIG.API_URL}/${path}`)

  // append the children=true query param to the url
  if (typeof children !== 'undefined' && children !== null) {
    url.searchParams.append('children', `${children}`)
  }

  // create the headers object
  const headers = new Headers()
  headers.append('Authorization', `${jwtToken}`)
  headers.append('Content-Type', 'application/json')

  // return a promise for the fetch request
  return fetch(url, {
    body: body ? JSON.stringify(body) : null,
    headers: Object.fromEntries(headers.entries()),
    method,
    signal,
  })
}

export default harborRequest
