import { TeamMember } from '@/utils/types'

export type FormState = {
  Id?: string
  newAdminEmail?: string
  newMemberEmail?: string
  members?: TeamMember[]
}
