import { Project, TeamMember, Token } from '@/types'

export type FormTeamState = {
  projects: Record<string, Project>
  members: Array<TeamMember>
  tokens: [string, Token][]
  name: string
}

export type FormState = {
  newAdminEmail?: string
  newMemberEmail?: string
  newProjects: Record<string, Project>
} & FormTeamState

export type FormStateNewProjects = Array<Project> | []
