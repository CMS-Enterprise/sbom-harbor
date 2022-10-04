import { CognitoUser } from '@aws-amplify/auth'

export type AppConfig = {
  AWS_REGION: string | 'us-east-1'
  API_URL: string
  USER_API_URL: string
  TEAMS_API_URL: string
  CF_DOMAIN: string
  USER_POOL_ID: string
  USER_POOL_CLIENT_ID: string
}

export type AppState = {
  teams: Team[] | []
}

export type CognitoUserInfo = {
  attributes: {
    email: string
    sub?: string
  }
  id?: string
  username: string
}

export type SessionContextType = {
  user: CognitoUser | null | undefined
  setUser: React.Dispatch<React.SetStateAction<CognitoUser | null | undefined>>
}

export type Codebase = {
  codebaseName: string
  language: string
  buildTool: string
}

export type Token = {
  token: string
  expires: string
  created: string
  enabled: boolean
}

export type Project = {
  projectName: string
  codebases?: Array<Codebase>
  tokens?: Array<Token>
}

export type TeamMember = {
  isTeamLead: boolean
  email: string
}

export type Team = {
  Id: string
  members: Array<TeamMember>
  projects?: Array<Project>
}

export enum CodebaseLanguage {
  C = 'C',
  CPP = 'C++',
  DOTNET = '.NET',
  GO = 'GO',
  JAVA = 'JAVA',
  JAVASCRIPT = 'JAVASCRIPT',
  NODE = 'NODE',
  PHP = 'PHP',
  PYTHON = 'PYTHON',
  RUBY = 'RUBY',
  RUST = 'RUST',
  OTHER = 'OTHER',
}

export enum BuildTool {
  ANT = 'ANT',
  GRADLE = 'GRADLE',
  MAVEN = 'MAVEN',
  NPM = 'NPM',
  PIP = 'PIP',
  VISUAL_STUDIO_BUILD_TOOLS = 'VISUAL_STUDIO_BUILD_TOOLS',
  OTHER = 'OTHER',
}
