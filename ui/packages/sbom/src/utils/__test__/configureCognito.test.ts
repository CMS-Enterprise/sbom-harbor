/**
 * Mock Auth from @aws-amplify/auth
 * @see {@link @cyclone-dx/ui-sbom/utils/configureCognito.js}
 */

// Mock the Amplify module to spy on the Auth.configure method
jest.mock('aws-amplify', () => ({
  Amplify: {
    configure: jest.fn(),
  },
}))

import { Amplify } from 'aws-amplify'
import configureCognito from '../configureCognito'

describe('configureAuth', () => {
  let result: null

  beforeEach(() => {
    jest.resetAllMocks()
    result = configureCognito()
  })

  afterAll(() => {
    jest.clearAllMocks()
  })

  it('should call Auth.configure', () => {
    expect(Amplify.configure).toHaveBeenCalled()
  })

  it('returns null', () => {
    expect(result).toBeNull()
  })
})
