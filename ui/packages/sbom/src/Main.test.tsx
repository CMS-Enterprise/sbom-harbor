import * as React from 'react'
import {
  act,
  cleanup,
  render,
  RenderResult,
  screen,
} from '@testing-library/react'
import { BrowserRouter as Router } from 'react-router-dom'
import { Auth } from 'aws-amplify'
import Main from './Main'

/**
 * Mock Auth from @aws-amplify/auth
 * @see {@link @cyclone-dx/ui-sbom/utils/configureCognito.js}
 */
jest.mock('aws-amplify')
jest.mocked(Auth.configure).mockImplementation(() => ({}))

/**
 * Mock the AuthProvider from hooks/useAuth
 * @see {@link @cyclonedx/ui-sbom/hooks/useAuth#AuthProvider}
 */
jest.mock('./hooks/useAuth', () => ({
  AuthProvider: ({ children }: React.PropsWithChildren) => (
    <div>{children}</div>
  ),
}))

describe('Main', () => {
  let component: RenderResult

  beforeAll(() => {
    act(() => {
      component = render(
        <Router>
          <Main />
        </Router>
      )
    })
  })

  afterAll(() => {
    cleanup()
    jest.resetAllMocks()
  })

  it('renders', () => {
    expect(screen.getByRole('main')).toBeVisible()
  })

  it('matches snapshot', () => {
    expect(component).toMatchSnapshot()
    expect(component.container).toMatchSnapshot()
  })
})
