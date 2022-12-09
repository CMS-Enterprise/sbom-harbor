import reduceArrayToMap from '@/selectors/reduceArrayToMap'
import { Project, ProjectModel } from '@/types'

const reduceProjectsArrayToMap = (
  projects: ProjectModel[]
): Record<string, Project> =>
  reduceArrayToMap(
    projects.map(
      ({ codebases, ...rest }: ProjectModel): Project => ({
        ...rest,
        codebases: reduceArrayToMap(codebases),
      })
    )
  )

export default reduceProjectsArrayToMap
