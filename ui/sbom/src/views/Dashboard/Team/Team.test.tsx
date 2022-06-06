import * as React from 'react'
import { render, screen } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'
import Team from './Team'

test('renders "Team" component', () => {
  render(<Team />, { wrapper: BrowserRouter })
  const linkElement = screen.getByTestId('team')
  expect(linkElement).toBeInTheDocument()
})
