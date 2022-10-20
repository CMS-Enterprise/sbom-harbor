/**
 * Component that renders all routes in the application.
 * @module @cyclonedx/ui/sbom/Routes
 * @see {@link @cyclonedx/ui/sbom/Main} for usage.
 */
import * as React from 'react'
import {
  createHashRouter,
  Params,
  useNavigate,
  useRouteError,
} from 'react-router-dom'
import { Auth } from '@aws-amplify/auth'
import Container from '@mui/material/Container'

// ** Public Views
import SignIn from '@/views/SignIn/SignIn'
import SignOut from '@/views/SignOut/SignOut'

// ** Private Views
import App from '@/views/App/App'
import Dashboard from '@/views/Dashboard/Dashboard'
import TeamForm from '@/views/Dashboard/Team/TeamForm'
import TeamView from '@/views/Dashboard/Team/TeamView'

// ** Components
import Main from '@/Main'
import NavigateToLogin from '@/components/react-router/NavigateToLogin'

// ** Utils
import { CONFIG } from '@/utils/constants'
import configureCognito from '@/utils/configureCognito'
import { Team, TeamMember, UserTableRowType } from './types'

const ErrorBoundary = () => {
  const error = useRouteError()
  const navigate = useNavigate()
  console.error('Error boundary:', error)

  React.useEffect(() => {
    const timeout = setTimeout(() => navigate('/logout'), 5000)
    return () => clearTimeout(timeout)
  })

  return (
    <Container sx={{ m: 3 }}>
      Your session has expired, please login again.
    </Container>
  )
}

const teamsLoader = async ({ request }: { request: Request }) => {
  const session = await Auth.currentSession()
  const jwtToken = session.getAccessToken().getJwtToken()

  if (!jwtToken) {
    throw new Error('No JWT token found')
  }

  const url = new URL(`${CONFIG.API_URL}/v1/teams`)
  url.searchParams.append('children', 'true')

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `${jwtToken}`,
    },
    signal: request.signal,
  })

  if (response.status === 401 || response.status === 403) {
    throw new Error(response.statusText)
  }

  return await response.json()
}

const teamLoader = async ({
  request,
  params: { teamId = '' },
}: {
  request: Request
  params: Params<string>
}) => {
  const session = await Auth.currentSession()
  const jwtToken = session.getAccessToken().getJwtToken()

  if (!jwtToken) {
    throw new Error('No JWT token found')
  }

  const url = new URL(`${CONFIG.API_URL}/v1/team/${teamId}`)
  url.searchParams.append('children', 'true')

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `${jwtToken}`,
    },
    signal: request.signal,
  })

  if (response.status === 401 || response.status === 403) {
    throw new Error(response.statusText)
  }

  const {
    [teamId]: team = {
      name: '',
      members: {},
      projects: {},
      tokens: {},
    },
  }: { [teamId: string]: Team } = await response.json()

  const newMembers = Object.entries(team.members).map(
    ([id, member]: [string, TeamMember]): UserTableRowType => {
      const { email = '', isTeamLead = false } = member as TeamMember
      return {
        id,
        email,
        isTeamLead,
        role: isTeamLead ? 'admin' : 'member',
      }
    }
  )

  return {
    name: team.name,
    members: Object.entries(team.members),
    projects: Object.entries(team.projects),
    tokens: Object.entries(team.tokens),
    memberTableRows: newMembers,
  }
}

export const router = createHashRouter([
  {
    path: '/',
    element: <Main />,
    loader: configureCognito,
    children: [
      {
        index: true,
        element: <NavigateToLogin />,
      },
      {
        path: 'login',
        element: <SignIn />,
      },
      {
        path: 'logout',
        element: <SignOut />,
      },
      {
        path: '*',
        element: <NavigateToLogin />,
      },
      {
        path: 'app/*',
        element: <App />,
        errorElement: <ErrorBoundary />,
        children: [
          {
            index: true,
            element: <Dashboard />,
            loader: teamsLoader,
            errorElement: <ErrorBoundary />,
          },
          {
            path: 'team',
            children: [
              {
                path: 'new',
                element: <TeamForm />,
              },
            ],
          },
          {
            path: 'teams/:teamId',
            loader: teamLoader,
            errorElement: <ErrorBoundary />,
            children: [
              {
                path: '',
                element: <TeamView />,
                loader: teamLoader,
                errorElement: <ErrorBoundary />,
              },
              {
                path: 'edit',
                element: <TeamForm />,
                loader: teamLoader,
                errorElement: <ErrorBoundary />,
              },
            ],
          },
        ],
      },
    ],
  },
])
