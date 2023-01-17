import * as React from 'react'
import { render, screen } from '@testing-library/react'
import Fallback from '@/components/SimpleLoadingFallback'

test('renders', () => {
  render(<Fallback />)
  expect(screen.getByTestId('simple-loading-fallback')).toBeInTheDocument()
})
