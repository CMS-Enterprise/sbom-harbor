import reduceArrayToMap from '@/selectors/reduceArrayToMap'
import { Project, ProjectResponse } from '@/types'

const projectsArrayToEntries = (
  projects: ProjectResponse[]
): Record<string, Project> =>
  reduceArrayToMap(
    projects.map(
      ({ codebases, ...rest }: ProjectResponse): Project => ({
        ...rest,
        codebases: reduceArrayToMap(codebases),
      })
    )
  )

export default projectsArrayToEntries
