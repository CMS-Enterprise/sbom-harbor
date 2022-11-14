import { Team, TeamApiResponse } from '@/types'
import projectsArrayToEntries from '@/selectors/projectsArrayToEntries'
import reduceArrayToMap from '@/selectors/reduceArrayToMap'

const teamsResponseToMappedProps = (teams: TeamApiResponse[]): Team[] =>
  teams.map(({ members, projects, tokens, ...rest }: TeamApiResponse) => ({
    ...rest,
    members, // TODO: also map members
    projects: projectsArrayToEntries(projects),
    tokens: reduceArrayToMap(tokens),
  }))

export default teamsResponseToMappedProps
