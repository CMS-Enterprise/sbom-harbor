import * as React from 'react'
import { render, screen } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'
import App from './App'

test('renders "App" component', () => {
  render(<App />, { wrapper: BrowserRouter })
  const linkElement = screen.getByTestId('app')
  expect(linkElement).toBeInTheDocument()
})
