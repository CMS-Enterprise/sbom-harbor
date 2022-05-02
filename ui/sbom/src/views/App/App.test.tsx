import * as React from 'react'
import { render, screen } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'
import App from './App'

test('renders the App component', () => {
  render(<App />, { wrapper: BrowserRouter })
  const app = screen.getByTestId('app')
  expect(app).toBeInTheDocument()
})
