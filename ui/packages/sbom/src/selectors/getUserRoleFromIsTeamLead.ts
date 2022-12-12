import { TeamMember, TeamMemberRole } from '@/types'

const getUserRoleFromIsTeamLead = ({
  isTeamLead = false,
}: TeamMember): TeamMemberRole =>
  isTeamLead ? TeamMemberRole.TEAM_LEAD : TeamMemberRole.MEMBER

export default getUserRoleFromIsTeamLead
