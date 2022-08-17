/**
 * Custom React Hook to use the AuthContext in a functional component
 *  in order to access the AuthContext's state and dispatch functions.
 * @see {@link @cyclonedx/ui-sbom/context/AuthContext}
 * @module @cyclonedx/ui-sbom/hooks/useAuth
 */
import { useContext } from 'react'
import { AuthContext } from '@/providers/AuthProvider'

/**
 * The custom hook to use the AuthContext in a functional component.
 * @returns {AuthValuesType} Hook that returns the current AuthContext value.
 */
export const useAuth = () => useContext(AuthContext)
