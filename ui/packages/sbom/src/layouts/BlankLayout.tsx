import * as React from 'react'
import { styled } from '@mui/material/styles'
import Box, { BoxProps } from '@mui/material/Box'
import { BlankLayoutProps } from './types'

// Styled component for Blank Layout component
const BlankLayoutWrapper = styled(Box)<BoxProps>(({ theme }) => ({
  height: '100%',
}))

/**
 * Layout component renders a basic wrapper container for other components.
 * @param {BlankLayoutProps} props The props for the BlankLayout component.
 * @param {React.ReactNode} props.children The child elements to render.
 * @returns {JSX.Element} The BlankLayout component.
 */
const BlankLayout = ({ children }: BlankLayoutProps) => {
  return (
    <BlankLayoutWrapper className="layout-wrapper">
      <Box
        className="app-content"
        sx={{
          minHeight: '100%',
          overflowX: 'hidden',
          position: 'relative',
          marginRight: '10%',
        }}
      >
        {children}
      </Box>
    </BlankLayoutWrapper>
  )
}

export default BlankLayout
