import reduceArrayToMap from '@/selectors/reduceArrayToMap'
import { Project, ProjectEntity } from '@/types'

const reduceProjectsArrayToMap = (
  projects: ProjectEntity[]
): Record<string, Project> =>
  reduceArrayToMap(
    projects.map(
      ({ codebases, ...rest }: ProjectEntity): Project => ({
        ...rest,
        codebases: reduceArrayToMap(codebases),
      })
    )
  )

export default reduceProjectsArrayToMap
