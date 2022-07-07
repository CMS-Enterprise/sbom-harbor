/**
 * This component is used to sign out the user and redirect
 *  her to the login page. It does not render anything.
 * @module @cyclonedx/ui/sbom/views/SignOut/SignOut
 */
import * as React from 'react'
import { useNavigate } from 'react-router-dom'
import { Auth } from '@aws-amplify/auth'
import { AuthContext } from '@/providers/AuthContext'

const SignOutContainer = (): JSX.Element => {
  const navigate = useNavigate()
  const { setUser } = React.useContext(AuthContext)

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

export default SignOutContainer
