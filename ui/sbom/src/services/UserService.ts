import { Auth } from '@aws-amplify/auth'
import { USER_API_URL } from '@/utils/constants'
import { CognitoUserInfo, Team } from '@/utils/types'

export const getTeams = async (
  abortController?: AbortController
): Promise<Team[]> => {
  const [user, token]: [CognitoUserInfo, string] = await Promise.all([
    Auth.currentUserInfo(),
    Auth.currentSession().then((session) => session.getIdToken().getJwtToken()),
  ])

  // FIXME: "@" character is not allowed in the URL
  const url = `${USER_API_URL}/teams?user_id=${user.attributes.email}`

  // TODO: instead use url.addQueryParameter() to add the email param
  //* const url = new URL(USER_API_URL)
  //* url.searchParams.append('user_id', user.attributes.email)

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
