/**
 * This component is used to sign out the user and redirect to the login page.
 * It does not render any visible UI and returns an empty React Fragment.
 * @module @cyclonedx/ui/sbom/views/SignOut/SignOut
 */
import * as React from 'react'
import { useNavigate } from 'react-router-dom'
import { Auth } from '@aws-amplify/auth'
import { useAuth } from '@/hooks/useAuth'

/**
 * A blank component that signs out the user and redirects to the login page.
 * @returns {JSX.Element} A component that renders an empty React Fragment.
 */
const SignOut = (): JSX.Element => {
  const navigate = useNavigate()
  const { setUser } = useAuth()

  React.useEffect(() => {
    const handleLogout = async () => {
      try {
        await Auth.signOut()
        setUser(null)
        navigate('/login')
      } catch (error) {
        console.log(error)
      }
    }
    handleLogout()
  })

  return <></>
}

export default SignOut
