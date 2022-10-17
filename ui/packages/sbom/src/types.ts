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
      })
  | null

export type UserTableRowType = {
  email: string
  isTeamLead: boolean
  avatarSrc?: string
  id?: string
  name?: string
  role?: 'admin' | 'member'
  username?: string
}

// ** Teams

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
  NONE = '',
}

export enum BuildTool {
  ANT = 'ANT',
  GRADLE = 'GRADLE',
  MAVEN = 'MAVEN',
  NPM = 'NPM',
  PIP = 'PIP',
  VISUAL_STUDIO_BUILD_TOOLS = 'VISUAL_STUDIO_BUILD_TOOLS',
  OTHER = 'OTHER',
  NONE = '',
}
