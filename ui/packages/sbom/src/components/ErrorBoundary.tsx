/**
 * Generic React error boundary component.
 * @module @cyclonedx/ui/sbom/components/ErrorBoundary
 */
import * as React from 'react'
import {
  isRouteErrorResponse,
  useNavigate,
  useRouteError,
} from 'react-router-dom'
import Container from '@mui/material/Container'

/**
 * A generic React error boundary component.
 * @returns {JSX.Element}
 */
const ErrorBoundary = () => {
  const navigate = useNavigate()
  const error = useRouteError()

  // handle auth session errors
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

  // otherwise, handle generic errors
  return (
    <Container sx={{ m: 3 }}>Something went wrong. Please try again.</Container>
  )
}

export default ErrorBoundary
