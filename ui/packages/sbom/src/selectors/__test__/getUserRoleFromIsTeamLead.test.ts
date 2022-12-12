import getUserRoleFromIsTeamLead from '../getUserRoleFromIsTeamLead'
import { TeamMember, TeamMemberRole } from '@/types'

const user = {
  id: 'user-0',
  isTeamLead: true,
  email: 'user-0@gmail.com',
} as TeamMember

test('returns the correct user role', () => {
  expect(getUserRoleFromIsTeamLead(user)).toBe(TeamMemberRole.TEAM_LEAD)
})
