import * as React from 'react'
import { render, screen } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'
import SignIn from './SignIn'

test('renders "Sign In" form component', () => {
  render(<SignIn />, { wrapper: BrowserRouter })
  const linkElement = screen.getByTestId('sign-in-form')
  expect(linkElement).toBeInTheDocument()
})
