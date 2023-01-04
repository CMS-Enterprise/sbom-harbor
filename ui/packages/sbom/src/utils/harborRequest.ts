/**
 * Factory to create a function that will make a request to the Harbor API
 * @module @cyclonedx/ui/sbom/utils/harborRequest
 */
import { CONFIG } from '@/utils/constants'

type Params = {
  body?: Record<string, unknown>
  jwtToken?: string
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE'
  path: string
  signal?: AbortSignal
  children?: true | null
}

const harborRequest = async ({
  body,
  jwtToken,
  method = 'GET',
  path,
  signal = new AbortController().signal,
  children = true,
}: Params): Promise<Response> => {
  // ensure path doesn't contain double slashes
  const url = new URL(`${CONFIG.API_URL}/v1/${path}`.replace(/\/\//g, '/'))

  // append the children=true query param to the url
  if (children) {
    url.searchParams.append('children', 'true')
  }

  // make the request and wait for the response
  return fetch(url.toString(), {
    method,
    headers: {
      'Content-Type': 'application/json',
      Authorization: `${jwtToken}`,
    },
    signal,
    body: body ? JSON.stringify(body) : null,
  })
}

export default harborRequest
