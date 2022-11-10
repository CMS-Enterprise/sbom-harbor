import { BuildTool, Codebase, CodebaseLanguage, Project } from '@/types'
import { FormState, FormTeamState } from './types'

export const defaultProject: Project = {
  id: '',
  name: '',
  fisma: '',
  codebases: {},
}

export const defaultCodebase: Codebase = {
  id: '',
  name: '',
  language: CodebaseLanguage.NONE,
  buildTool: BuildTool.NONE,
}

export const defaultTeam: FormTeamState = {
  name: '',
  projects: {},
  members: [],
  tokens: [],
}

export const defaultFormState: FormState = {
  newAdminEmail: '',
  newMemberEmail: '',
  newProjects: {},
  projects: {},
  members: [],
  tokens: [],
  name: '',
}

export const defaultFormStateNewProjects: FormState['newProjects'] = {}
