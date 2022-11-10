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
  json,
  isRouteErrorResponse,
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
import { TeamApiResponse } from './types'

const ErrorBoundary = () => {
  const error = useRouteError()
  const navigate = useNavigate()
  console.error('Error boundary:', error)

  if (isRouteErrorResponse(error) && error.status === 401) {
    setTimeout(() => navigate('/logout'), 5000)
    return (
      <div>
        <h1>
          {error.status} {error.statusText}
        </h1>
        <h2>{error.data}</h2>
        <p>Your session has expired. Please login again.</p>
      </div>
    )
  }

  // generic error
  return (
    <Container sx={{ m: 3 }}>Something went wrong. Please try again.</Container>
  )
}

const authLoader = async () => {
  const session = await Auth.currentSession()
  const jwtToken = session.getAccessToken().getJwtToken()
  if (!jwtToken) {
    throw new Response('Invalid Session', { status: 401 })
  }
  return jwtToken
}

const teamsLoader = async ({ request }: { request: Request }) => {
  const jwtToken = await authLoader()

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

  if (!response.ok) {
    throw new Response(response.statusText, { status: response.status })
  }

  return json<Promise<TeamApiResponse>>(await response.json())
}

const teamLoader = async ({
  request,
  params: { teamId = '' },
}: {
  request: Request
  params: Params<string>
}) => {
  const jwtToken = await authLoader()

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

  if (!response.ok) {
    throw new Response(response.statusText, { status: response.status })
  }

  return json<Promise<TeamApiResponse>>(await response.json())
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
        loader: authLoader,
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
