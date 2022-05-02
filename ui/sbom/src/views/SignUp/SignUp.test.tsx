import * as React from 'react'
import { render, screen } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'
import SignUp from './SignUp'

test('renders "Sign Up" form component', () => {
  render(<SignUp />, { wrapper: BrowserRouter })
  const linkElement = screen.getByTestId('sign-up-form')
  expect(linkElement).toBeInTheDocument()
})
