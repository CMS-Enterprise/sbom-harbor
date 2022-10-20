/**
 * This component is used to sign out the user and redirect to the login page.
 * It does not render any visible UI and returns an empty React Fragment.
 * @module @cyclonedx/ui/sbom/views/SignOut/SignOut
 */
import * as React from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuthDispatch } from '@/hooks/useAuth'
import logoutUser from '@/actions/logoutUser'

/**
 * A blank component that signs out the user and redirects to the login page.
 * @returns {JSX.Element} A component that renders an empty React Fragment.
 */
const SignOut = (): JSX.Element => {
  const dispatch = useAuthDispatch()
  const navigate = useNavigate()

  React.useEffect(() => {
    logoutUser(dispatch).then(() => {
      navigate('/login')
    })
  }, [])

  return <></>
}

export default SignOut
