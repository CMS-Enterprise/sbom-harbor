import { Auth } from 'aws-amplify'
import { CONFIG } from '@/utils/constants'

const { USER_API_SEARCH_URL } = CONFIG

const getAddress = async (
  filter: string,
  abortController: AbortController = new AbortController()
): Promise<Array<string>> => {
  const session = await Auth.currentSession()
  const jwtToken = session.getAccessToken().getJwtToken()

  // TODO: use url.searchParams instead of building the url manually
  const url = `${USER_API_SEARCH_URL}?filter=${filter}`
  // const url = new URL(USER_API_SEARCH_URL)
  // url.searchParams.append('filter', filter)

  const res = await fetch(url, {
    headers: { Authorization: `${jwtToken}` },
    method: 'GET',
    signal: abortController.signal,
  })

  if (res.status === 500) {
    throw new Error('Internal server error')
  }

  return res.json()
}

export default getAddress
