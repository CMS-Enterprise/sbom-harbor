import { Project, TeamMember, Token } from '@/types'

export type FormTeamState = {
  projects: Record<string, Project>
  members: Record<string, TeamMember>
  tokens: Record<string, Token>
  name: string
}

export type FormState = {
  newAdminEmail?: string
  newMemberEmail?: string
} & FormTeamState

export type FormStateNewProjects = Array<Project> | []
