/**
 * Component that renders all routes in the application.
 * @module @cyclonedx/ui/sbom/router
 * @see {@link @cyclonedx/ui/sbom/Main} for usage.
 */
import * as React from 'react'
import { createHashRouter } from 'react-router-dom'
import { RouteIds } from '@/types'

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
import ErrorBoundary from '@/components/ErrorBoundary'
import NavigateToLogin from '@/components/react-router/NavigateToLogin'

// ** Loaders, Utils
import authLoader from '@/router/authLoader'
import teamLoader from '@/router/teamLoader'
import teamsLoader from '@/router/teamsLoader'
import configureCognito from '@/utils/configureCognito'

/**
 * The hash router for the application that defines routes
 *  and specifies the loaders for routes with dynamic data.
 * @see {@link https://reactrouter.com/web/api/HashRouter HashRouter}
 * @see {@link https://reactrouter.com/en/main/route/loader loader}
 */
const router = createHashRouter([
  {
    id: RouteIds.MAIN,
    path: '/',
    loader: configureCognito,
    element: <Main />,
    children: [
      {
        index: true,
        element: <NavigateToLogin />,
      },
      {
        id: RouteIds.LOGIN,
        path: 'login',
        element: <SignIn />,
      },
      {
        id: RouteIds.LOGOUT,
        path: 'logout',
        element: <SignOut />,
      },
      {
        path: '*',
        element: <NavigateToLogin />,
      },
      {
        path: 'app/*',
        id: RouteIds.AUTHED_APP,
        element: <App />,
        errorElement: <ErrorBoundary />,
        loader: authLoader,
        children: [
          {
            index: true,
            id: RouteIds.DASHBOARD,
            loader: teamsLoader,
            element: <Dashboard />,
            errorElement: <ErrorBoundary />,
          },
          {
            path: 'team',
            children: [
              {
                id: RouteIds.TEAM_NEW,
                loader: () => ({
                  data: Promise.resolve({}),
                }),
                path: 'new',
                element: <TeamForm />,
              },
            ],
          },
          {
            id: RouteIds.TEAM,
            path: 'teams/:teamId',
            loader: teamLoader,
            errorElement: <ErrorBoundary />,
            children: [
              {
                id: RouteIds.TEAM_VIEW,
                path: '',
                loader: teamLoader,
                element: <TeamView />,
                errorElement: <ErrorBoundary />,
              },
              {
                id: RouteIds.TEAM_EDIT,
                path: 'edit',
                loader: teamLoader,
                element: <TeamForm />,
                errorElement: <ErrorBoundary />,
              },
            ],
          },
        ],
      },
    ],
  },
])

export default router
