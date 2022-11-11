/**
 * State loader for react-router data routes that require a user's single team.
 * @module @cyclonedx/ui/sbom/loaders/teamLoader
 * @see {@link @cyclonedx/ui/sbom/Routes}
 */
import { json, Params } from 'react-router-dom'
import { ProjectResponse, TeamApiResponse } from '@/types'
import authLoader from '@/router/authLoader'
import harborRequest from '@/utils/harborRequest'

const teamLoader = async ({
  request,
  params: { teamId = '' },
}: {
  request: Request
  params: Params<string>
}) => {
  const data = await harborRequest({
    path: `team/${teamId}`,
    jwtToken: await authLoader(),
    signal: request.signal,
  })
  return json<Promise<TeamApiResponse>>({
    ...data,
    projects: data.projects
      .map((project: ProjectResponse) => {
        return {
          ...project,
          codebases: project.codebases.reduce((acc: any, codebase: any) => {
            return {
              ...acc,
              [codebase.id]: codebase,
            }
          }, {}),
        }
      })
      .reduce((acc: any, project: any) => {
        return {
          ...acc,
          [project.id]: project,
        }
      }, {}),
  })
}

export default teamLoader
