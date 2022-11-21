import { Project, TeamMember, Token } from '@/types'

export type FormTeamState = {
  projects: Record<string, Project>
  members: Array<TeamMember>
  tokens: Array<Token>
  name: string
}

export type FormState = {
  newAdminEmail?: string
  newMemberEmail?: string
} & FormTeamState

export type FormStateNewProjects = Array<Project> | []
