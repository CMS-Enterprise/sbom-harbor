import { Project } from '@/types'

export const defaultProject: Project = {
  projectName: '',
  codebases: [],
  tokens: [],
}

export const defaultTeam = {
  Id: '',
  members: [],
  projects: [],
}

export const defaultFormState = {
  newAdminEmail: '',
  newMemberEmail: '',
  ...defaultTeam,
}
