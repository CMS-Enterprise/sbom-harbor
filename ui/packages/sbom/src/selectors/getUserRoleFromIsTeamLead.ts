import { User, UserRole } from '@/types'

const getUserRoleFromIsTeamLead = ({ isTeamLead = false }: User): UserRole =>
  isTeamLead ? UserRole.ADMIN : UserRole.MEMBER

export default getUserRoleFromIsTeamLead
