import { Project, TeamMember, Token } from '@/types'

export type FormTeamState = {
  projects: [string, Project][]
  members: [string, TeamMember][]
  tokens: [string, Token][]
  name: string
}

export type FormState = {
  newAdminEmail?: string
  newMemberEmail?: string
  newProjects: Project[]
} & FormTeamState

export type FormStateNewProjects = Array<Project> | []
