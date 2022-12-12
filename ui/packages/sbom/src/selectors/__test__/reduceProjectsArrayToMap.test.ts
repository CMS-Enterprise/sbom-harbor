import reduceProjectsArrayToMap from '../reduceProjectsArrayToMap'

const projects = [
  { id: 'project-0', name: 'project-0', fisma: 'project-0', codebases: [] },
  { id: 'project-1', name: 'project-1', fisma: 'project-1', codebases: [] },
]

const correctOutput = {
  [projects[0].id]: { ...projects[0], codebases: {} },
  [projects[1].id]: { ...projects[1], codebases: {} },
}

test('returns the correct result', () => {
  const result = reduceProjectsArrayToMap(projects)
  expect(result).toEqual(correctOutput)
})
