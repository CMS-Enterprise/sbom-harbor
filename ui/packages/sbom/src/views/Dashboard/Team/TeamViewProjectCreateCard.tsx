import * as React from 'react'
import Box from '@mui/material/Box'
import Card from '@mui/material/Card'
import Typography from '@mui/material/Typography'
import IconButton from '@mui/material/IconButton'
import CardHeader from '@mui/material/CardHeader'
import CardContent from '@mui/material/CardContent'
import TextField from '@mui/material/TextField'
import Button from '@mui/material/Button'
import Grid from '@mui/material/Grid'
import MenuItem from '@mui/material/MenuItem'
import FormControl from '@mui/material/FormControl'
import Select from '@mui/material/Select'
import InputLabel from '@mui/material/InputLabel'
import DotsVertical from 'mdi-material-ui/DotsVertical'
import { Codebase, CodebaseLanguage, BuildTool, Project } from '@/utils/types'

const LANGUAGES = [...Object.keys(CodebaseLanguage)]
const BUILD_TOOLS = [...Object.keys(BuildTool)]

const defaultProject: Project = {
  projectName: '',
  codebases: [],
  tokens: [],
}

const defaultCodebase: Codebase = {
  codebaseName: '',
  language: '',
  buildTool: '',
}

const TeamViewProjectCreateCard = ({ project }: { project: Project }) => {
  // reducer for the form state
  const [formInput, setFormInput] = React.useReducer(
    (state: Project, newState: Project) => ({ ...state, ...newState }),
    {
      ...defaultProject,
      ...project,
    }
  )

  const [newCodebase, setNewCodebase] = React.useReducer(
    (state: Codebase, newState: Codebase) => ({ ...state, ...newState }),
    defaultCodebase
  )

  // function that handlers change events on form inputs
  const handleInput = (
    evt: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = evt.currentTarget
    setFormInput({
      projectName: project.projectName,
      codebases: [...(project.codebases || []), newCodebase],
      [name]: value,
    })
  }

  // function that handlers change events on form inputs
  const handleInputCodebase = (
    evt: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = evt.currentTarget
    setNewCodebase({ ...newCodebase, [name]: value })
  }

  const handleAddCodebase = () => {
    // @ts-ignore
    const codebases = [...formInput.codebases, { ...defaultCodebase }]
    setFormInput({
      ...formInput,
      codebases,
    })
  }

  return (
    <Card>
      <CardHeader
        title={project.projectName}
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
      ></CardHeader>
      <CardContent>
        <Typography component="h5" variant="caption" sx={{ mb: 5 }}>
          <TextField
            fullWidth
            id="projectName"
            label="Project Name"
            name="projectName"
            onChange={handleInput}
            required
            value={formInput?.projectName}
            variant="standard"
            InputProps={{
              sx: {
                '& .Mui-disabled': {
                  color: 'text.primary',
                },
              },
            }}
          />
        </Typography>
        <Grid
          container
          spacing={2}
          sx={{
            mb: 3,
            width: '100%',
            flexFlow: 'row',
            alignItems: 'center',
            justifyContent: 'space-between',
          }}
        >
          <Grid item sx={{ justifyContent: 'space-between' }}>
            <Typography component="p" variant="caption" sx={{ mb: 0 }}>
              {project?.codebases?.length || 0} Codebases
            </Typography>
          </Grid>
          <Grid item>
            <Button
              sx={{ mt: 0, mb: 0, ml: 1 }}
              variant="contained"
              size="small"
              color="secondary"
              onClick={handleAddCodebase}
            >
              Add Codebase
            </Button>
          </Grid>
        </Grid>

        {formInput?.codebases?.map((codebase: Codebase, index: number) => {
          return (
            <Box
              key={`${codebase.codebaseName}-${index}`}
              sx={{
                display: 'flex',
                alignItems: 'center',
                mb:
                  index !== (formInput?.codebases?.length || 0) - 1
                    ? 5.75
                    : undefined,
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
                    variant="h6"
                    sx={{ fontWeight: 600, color: 'text.primary' }}
                  >
                    <TextField
                      fullWidth
                      id="codebaseName"
                      label="Codebase Name"
                      name="codebaseName"
                      onChange={handleInputCodebase}
                      required
                      value={codebase.codebaseName}
                      variant="standard"
                      InputProps={{
                        sx: {
                          '& .Mui-disabled': {
                            color: 'text.primary',
                          },
                        },
                      }}
                    />
                  </Typography>
                </Box>
                <Box
                  sx={{
                    display: 'flex',
                    flexWrap: 'wrap',
                    alignItems: 'center',
                    justifyContent: 'flex-end',
                  }}
                >
                  <Grid container spacing={1}>
                    <Grid item>
                      <FormControl>
                        <InputLabel>Language</InputLabel>
                        <Select
                          size="small"
                          value={codebase.language}
                          label="Age"
                          defaultValue=""
                          id="demo-simple-select-outlined"
                          labelId="demo-simple-select-outlined-label"
                        >
                          <MenuItem value="">
                            <em>None</em>
                          </MenuItem>
                          {LANGUAGES.map((language: string) => (
                            <MenuItem key={language} value={language}>
                              {language}
                            </MenuItem>
                          ))}
                        </Select>
                      </FormControl>
                    </Grid>
                    <Grid item>
                      <FormControl>
                        <InputLabel>Build Tool</InputLabel>
                        <Select
                          size="small"
                          value={codebase.buildTool}
                          label="Age"
                          defaultValue=""
                          id="demo-simple-select-outlined"
                          labelId="demo-simple-select-outlined-label"
                        >
                          <MenuItem value="">
                            <em>None</em>
                          </MenuItem>
                          {BUILD_TOOLS.map((buildTool: string) => (
                            <MenuItem key={buildTool} value={buildTool}>
                              {buildTool}
                            </MenuItem>
                          ))}
                        </Select>
                      </FormControl>
                    </Grid>
                  </Grid>
                </Box>
              </Box>
            </Box>
          )
        })}
      </CardContent>
    </Card>
  )
}

export default TeamViewProjectCreateCard
