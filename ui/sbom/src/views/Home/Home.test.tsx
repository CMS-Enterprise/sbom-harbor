import * as React from 'react'
import { render, screen } from '@testing-library/react'
import Home from './Home'

test('renders "Welcome" text', () => {
  render(<Home />)
  const textElement = screen.getByText(/Welcome to/i)
  expect(textElement).toBeInTheDocument()
})
