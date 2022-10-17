/**
 * Component that renders all routes in the application.
 * @module @cyclonedx/ui/sbom/Routes
 * @see {@link @cyclonedx/ui/sbom/Main} for usage.
 */
// ** React Imports
import * as React from 'react'
import { Route, Routes } from 'react-router-dom'

// ** Public Views
import SignIn from '@/views/SignIn/SignIn'
import SignOut from '@/views/SignOut/SignOut'

// ** Private Views
import App from '@/views/App/App'
import Dashboard from '@/views/Dashboard/Dashboard'
import TeamForm from '@/views/Dashboard/Team/TeamForm'
import TeamView from '@/views/Dashboard/Team/TeamView'

// ** Components
import NavigateToLogin from '@/components/react-router/NavigateToLogin'

const AppRoutes = (): JSX.Element => (
  <Routes>
    {/* Public Routes */}
    <Route path="/">
      {/* Index route that redirects to the `/login` route */}
      <Route index element={<NavigateToLogin />} />
      {/* Auth routes for login and logout */}
      <Route path="login" element={<SignIn />} />
      <Route path="logout" element={<SignOut />} />
      {/* Catch-all route that redirects to `/login` if no route is matched */}
      <Route path="*" element={<NavigateToLogin />} />
    </Route>
    {/* Protected Routes */}
    <Route path="/app/*" element={<App />}>
      {/* Index route that the user sees after logging in */}
      <Route path="" element={<Dashboard />} />
      {/* Team CRUD routes */}
      <Route path="team/new" element={<TeamForm />} />
      <Route path="teams/:teamId">
        <Route path="" element={<TeamView />} />
        <Route path="edit" element={<TeamForm />} />
      </Route>
    </Route>
  </Routes>
)

export default AppRoutes
