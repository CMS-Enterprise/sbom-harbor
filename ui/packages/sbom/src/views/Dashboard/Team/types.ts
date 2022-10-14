import { TeamMember } from '@/types'

export type FormState = {
  Id?: string
  newAdminEmail?: string
  newMemberEmail?: string
  members?: TeamMember[]
}
