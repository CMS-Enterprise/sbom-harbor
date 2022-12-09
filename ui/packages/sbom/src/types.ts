/**
 * @module @cyclone-dx/ui/sbom/utils/types
 */
import {
  CognitoIdToken,
  CognitoUser,
  CognitoUserSession,
} from 'amazon-cognito-identity-js'

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
  teams: Array<TeamModel>
}

export type AppStateTeam = {
  name: string
  members: User[]
  projects: Project[]
  tokens: Token[]
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

export type User = {
  id: string
  email: string
  isTeamLead: boolean
}

export type UserTableRowType = User & {
  avatarSrc?: string
  name?: string
  role?: UserRole
  username?: string
}

export type TokenRowType = {
  id: string
  name: string
  created: string
  expires: string
  enabled: boolean
  token: string
}

/**
 * List of user roles to display in a table of members
 * @see {@link @cyclone-dx/ui/sbom/components/TeamMembersTable}
 */
export enum UserRole {
  ADMIN = 'admin',
  MEMBER = 'member',
}

// ** Teams

// a team as it is set in the app state
export type Team = {
  id: string
  name: string
  members: Record<string, User>
  projects: Record<string, Project>
  tokens: Record<string, Token>
}

// a team as it is returned from the API
export type TeamModel = {
  id: string
  name: string
  members: Array<User>
  projects: Array<ProjectModel>
  tokens: Array<Token>
}

// a project as it is set in the app state
export type Project = {
  id: string
  name: string
  fisma: string
  codebases: Record<string, Codebase>
}

// a project as it is returned from the API
export type ProjectModel = {
  id: string
  name: string
  fisma: string
  codebases: Codebase[]
}

// a codebase as it is returned from the API and set in the app state
export type Codebase = {
  id: string
  name: string
  language: CodebaseLanguage | ''
  buildTool: BuildTool | ''
}

// a token as it is returned from the API and set in the app state
export type Token = {
  id: string
  name: string
  created: string | number
  expires: string | number
  enabled: boolean
  token: string
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
}
