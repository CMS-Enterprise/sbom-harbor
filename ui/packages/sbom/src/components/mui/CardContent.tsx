/**
 * Custom MuiCardContent components
 * @module @cyclonedx/ui/sbom/components/mui/CardContent
 */
import { styled } from '@mui/system'
import CardContent from '@mui/material/CardContent'

export const CenteredCardContent = styled(CardContent)({
  height: '100%',
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  justifyContent: 'center',
  textAlign: 'center',
})
