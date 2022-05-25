import * as React from 'react'
import { render, screen } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'
import Layout from './Layout'

test('renders the Layout component', () => {
  render(<Layout />, { wrapper: BrowserRouter })
  const layout = screen.getByTestId('layout')
  expect(layout).toBeInTheDocument()
})
