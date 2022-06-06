/**
 * Sign in page component that renders a login form.
 * @module @cyclonedx/ui/sbom/views/SignOut/SignOut
 */
import * as React from 'react'
import { useNavigate } from 'react-router-dom'
import { Auth } from '@aws-amplify/auth'
import { SessionContext } from '@/services/auth'

const SignOut = (): JSX.Element => {
  const navigate = useNavigate()
  const { setUser } = React.useContext(SessionContext)

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
