import { BuildTool, CodebaseLanguage } from '@/types'
import mapTeamsPropertiesToObjects from '../mapTeamsPropertiesToObjects'

const rtf = new Intl.RelativeTimeFormat('en', {
  localeMatcher: 'best fit',
  numeric: 'always',
  style: 'long',
})

describe('mapTeamsPropertiesToObjects', () => {
  const codebase0 = {
    id: 'codebase-0',
    name: 'codebase-0',
    buildTool: BuildTool.NPM,
    language: CodebaseLanguage.JAVASCRIPT,
  }

  const codebases = [{ ...codebase0 }]

  const project = {
    id: 'project-0',
    name: 'project-0',
    fisma: 'project-0',
  }

  const project0 = { ...project, codebases }

  const projects = [{ ...project0 }]

  const member0 = {
    id: 'member-0',
    email: 'member-0@gmail.com',
    isTeamLead: true,
  }

  const members = [{ ...member0 }]

  const token0 = {
    id: 'token-0',
    name: 'token-0',
    token: 'token-0',
    enabled: true,
    created: rtf.format(0, 'day'),
    expires: rtf.format(1, 'week'),
  }

  const tokens = [{ ...token0 }]

  const team = {
    id: 'team-0',
    name: 'team-0',
  }

  const teams = [
    {
      ...team,
      members: [...members],
      projects: [...projects],
      tokens: [...tokens],
    },
  ]

  const correctResult = [
    {
      ...team,
      members: {
        'member-0': {
          ...member0,
        },
      },
      projects: {
        'project-0': {
          ...project,
          codebases: {
            'codebase-0': {
              ...codebase0,
            },
          },
        },
      },
      tokens: {
        'token-0': {
          ...token0,
        },
      },
    },
  ]

  it('should map a response to a teams object', () => {
    const result = mapTeamsPropertiesToObjects(teams)
    expect(result).toEqual(correctResult)
  })

  it('should return an empty array if no teams are passed', () => {
    const result = mapTeamsPropertiesToObjects([])
    expect(result).toEqual([])
  })
})
