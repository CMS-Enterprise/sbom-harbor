/**
 * Component that renders the react-router Navigate component to
 *  redirect the user to the login page at the `/login` route.
 * @module @cyclonedx/ui/sbom/components/react-router/NavigateToLogin
 * @see {@link @cyclonedx/ui/sbom/Routes} for usage.
 * @see {@link https://reactrouter.com/web/api/Navigate} for documentation.
 */
import * as React from 'react'
import { Navigate } from 'react-router-dom'

/**
 * Component that renders the Navigate component to redirect to the login page.
 * @returns {JSX.Element}
 */
const NavigateToLogin = (): JSX.Element => (
  <Navigate to="/login" replace={true} />
)

export default NavigateToLogin
