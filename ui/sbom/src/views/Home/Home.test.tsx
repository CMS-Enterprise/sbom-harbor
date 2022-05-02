import * as React from 'react'
import { render, screen } from '@testing-library/react'
import Home from './Home'

test('renders "Welcome" text', () => {
  render(<Home />)
  const textElement = screen.getByText(/welcome to the/i)
  expect(textElement).toBeInTheDocument()
})
