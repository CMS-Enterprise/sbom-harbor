/**
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamViewProjectCard
 */
import * as React from 'react'
import Box from '@mui/material/Box'
import Card from '@mui/material/Card'
import Typography from '@mui/material/Typography'
import IconButton from '@mui/material/IconButton'
import CardHeader from '@mui/material/CardHeader'
import CardContent from '@mui/material/CardContent'
import DotsVertical from 'mdi-material-ui/DotsVertical'
import { Project } from '@/types'

type InputProps = { project: Project }

const TeamViewProjectCard = ({ project }: InputProps): JSX.Element => (
  <Card>
    <CardHeader
      title={project.name}
      titleTypographyProps={{
        sx: {
          lineHeight: '2rem !important',
          letterSpacing: '0.15px !important',
        },
      }}
      action={
        <IconButton
          size="small"
          aria-label="settings"
          className="card-more-options"
        >
          <DotsVertical />
        </IconButton>
      }
    >
      <Typography component="h5" variant="caption" sx={{ mb: 5 }}>
        {project.name}
      </Typography>
    </CardHeader>
    <CardContent>
      <Typography component="p" variant="caption" sx={{ mb: 5 }}>
        <>{project?.codebases?.length || 0} Codebases</>
      </Typography>
      {Object.entries(project.codebases).map(([key, item], index, array) => {
        return (
          <Box
            key={key}
            sx={{
              display: 'flex',
              alignItems: 'center',
              mb: index !== (array.length || 0) - 1 ? 5.75 : undefined,
            }}
          >
            <Box
              sx={{
                ml: 3,
                width: '100%',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
              }}
            >
              <Box sx={{ mr: 2, display: 'flex', flexDirection: 'column' }}>
                <Typography
                  variant="body2"
                  sx={{ fontWeight: 600, color: 'text.primary' }}
                >
                  {item.name}
                </Typography>
                <Typography variant="caption">{item.language}</Typography>
              </Box>
              <Box
                sx={{
                  display: 'flex',
                  flexWrap: 'wrap',
                  alignItems: 'center',
                  justifyContent: 'flex-end',
                }}
              >
                <Typography
                  variant="body2"
                  sx={{ fontWeight: 600, color: 'text.primary' }}
                >
                  {item.buildTool}
                </Typography>
              </Box>
            </Box>
          </Box>
        )
      })}
    </CardContent>
  </Card>
)

export default TeamViewProjectCard
