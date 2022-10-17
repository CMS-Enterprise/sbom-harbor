import { Project } from '@/types'
import { FormState, FormTeamState } from './types'

export const defaultProject: Project = {
  name: '',
  fisma: '',
  codebases: {},
}

export const defaultTeam: FormTeamState = {
  name: '',
  projects: [],
  members: [],
  tokens: [],
}

export const defaultFormState: FormState = {
  newAdminEmail: '',
  newMemberEmail: '',
  newProjects: [],
  projects: [],
  members: [],
  tokens: [],
  name: '',
}

export const defaultFormStateNewProjects: FormState['newProjects'] = []
