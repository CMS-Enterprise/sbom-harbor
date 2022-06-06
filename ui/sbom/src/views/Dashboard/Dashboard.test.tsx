import * as React from 'react'
import { render, screen } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'
import Dashboard from './Dashboard'

test('renders "Dashboard" component', () => {
  render(<Dashboard />, { wrapper: BrowserRouter })
  const linkElement = screen.getByTestId('Dashboard')
  expect(linkElement).toBeInTheDocument()
})
