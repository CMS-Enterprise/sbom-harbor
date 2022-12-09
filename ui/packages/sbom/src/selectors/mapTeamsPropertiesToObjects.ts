import { Team, TeamModel } from '@/types'
import reduceProjectsArrayToMap from '@/selectors/reduceProjectsArrayToMap'
import reduceArrayToMap from '@/selectors/reduceArrayToMap'

const mapTeamsPropertiesToObjects = (teams: TeamModel[]): Team[] =>
  teams.map(({ members, projects, tokens, ...rest }: TeamModel) => ({
    ...rest,
    members: reduceArrayToMap(members),
    projects: reduceProjectsArrayToMap(projects),
    tokens: reduceArrayToMap(tokens),
  }))

export default mapTeamsPropertiesToObjects
