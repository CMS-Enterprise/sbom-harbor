/**
 * @module @cyclone-dx/ui/sbom/utils/types
 */
import { CognitoUser } from '@aws-amplify/auth'
import { CognitoIdToken, CognitoUserSession } from 'amazon-cognito-identity-js'

// ** App

export type AppConfig = {
  AWS_REGION: string | 'us-east-1'
  CF_DOMAIN: string
  API_URL: string
  USER_API_URL: string
  TEAM_API_URL: string
  TEAMS_API_URL: string
  USER_POOL_ID: string
  USER_POOL_CLIENT_ID: string
}

export type AppStateSlice<P> = {
  [k: string]: P
}

export type AppState = {
  teams: Record<string, Team>
}

export type AppStateTeam = {
  name: string
  members: [string, TeamMember][]
  projects: [string, Project][]
  tokens: [string, Token][]
  memberTableRows: UserTableRowType[]
}

export type ThemeColor =
  | 'primary'
  | 'secondary'
  | 'error'
  | 'warning'
  | 'info'
  | 'success'

// ** Users

export type CognitoUserInfo = {
  attributes: {
    email: string
    sub?: string
    'custom:teams': string
  }
  id?: string
  username: string
}

export type UserState = {
  user: CognitoUser
  userInfo: CognitoUserInfo
  userSession: CognitoUserSession
  idToken: CognitoIdToken
  jwtToken: string
}

export type UserDataType =
  | (UserState &
      CognitoUserInfo & {
        role?: string
        avatar?: string | null
        email?: string
        fullName?: string
        password?: string
        teams?: string[]
      })
  | null

export type UserTableRowType = {
  id: string
  email: string
  isTeamLead: boolean
  avatarSrc?: string
  name?: string
  role?: 'admin' | 'member'
  username?: string
}

// ** Teams

// a team as it is stored in the database
export type Team = {
  name: string
  members: Record<string, TeamMember>
  projects: Record<string, Project>
  tokens: Record<string, Token>
}

export type TeamMember = {
  email: string
  isTeamLead: boolean
}

export type Token = {
  name: string
  created: string | number
  expires: string | number
  enabled: boolean
  token: string
}

export type Project = {
  name: string
  fisma: string
  codebases: Record<string, Codebase>
}

export type Codebase = {
  name: string
  language: CodebaseLanguage
  buildTool: BuildTool
}

// TODO: some of these are frameworks, not languages
export enum CodebaseLanguage {
  C = 'C',
  CPP = 'C++',
  DOTNET = '.NET',
  GO = 'go',
  JAVA = 'Java',
  JAVASCRIPT = 'Javascript',
  TYPESCRIPT = 'Typescript',
  NODE = 'Node.js',
  PHP = 'PHP',
  PYTHON = 'Python',
  RUBY = 'Ruby',
  RUST = 'Rust',
  OTHER = 'Other',
  NONE = '',
}

export enum BuildTool {
  ANT = 'ant',
  GRADLE = 'gradle',
  MAVEN = 'maven',
  NPM = 'npm',
  YARN = 'yarn',
  PIP = 'pip',
  VISUAL_STUDIO_BUILD_TOOLS = 'Visual Studio Build Tools',
  OTHER = 'Other',
  NONE = '',
}