import { CognitoUser } from '@aws-amplify/auth'
import { CognitoUserSession } from 'amazon-cognito-identity-js'

export type AppConfig = {
  AWS_REGION: string | 'us-east-1'
  API_URL: string
  TEAMS_API_URL: string
  CF_DOMAIN: string
  USER_POOL_ID: string
  USER_POOL_CLIENT_ID: string
}

export type AppState = {
  teams: Team[]
  user?: CognitoUserSession | null
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
  codebases: Codebase[]
  tokens: Token[]
}

export type TeamMember = {
  isTeamLead: boolean
  email: string
}

export type Team = {
  Id: string
  members: TeamMember[]
}
