import getUserRoleFromIsTeamLead from '../getUserRoleFromIsTeamLead'
import { User, UserRole } from '@/types'

const user = {
  id: 'user-0',
  isTeamLead: true,
  email: 'user-0@gmail.com',
} as User

describe('getUserRoleFromIsTeamLead', () => {
  it('returns the correct user role', () => {
    expect(getUserRoleFromIsTeamLead(user)).toBe(UserRole.ADMIN)
  })
})
