import { TeamMember, Project } from '@/types'

export type FormState = {
  Id?: string
  newAdminEmail?: string
  newMemberEmail?: string
  members?: TeamMember[]
  projects?: Project[]
}
