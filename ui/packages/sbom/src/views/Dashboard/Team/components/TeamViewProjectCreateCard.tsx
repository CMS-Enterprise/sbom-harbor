/**
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamViewProjectCreateCard
 */
import * as React from 'react'
import { v4 as uuidv4 } from 'uuid'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Card from '@mui/material/Card'
import CardActions from '@mui/material/CardActions'
import CardContent from '@mui/material/CardContent'
import CardHeader from '@mui/material/CardHeader'
import FormControl from '@mui/material/FormControl'
import Grid from '@mui/material/Grid'
import IconButton from '@mui/material/IconButton'
import InputLabel from '@mui/material/InputLabel'
import MenuItem from '@mui/material/MenuItem'
import Select from '@mui/material/Select'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import DotsVertical from 'mdi-material-ui/DotsVertical'
import { BuildTool, Codebase, CodebaseLanguage, Project } from '@/types'
import {
  defaultProject,
  defaultCodebase,
} from '@/views/Dashboard/Team/constants'

const LANGUAGES = [...Object.keys(CodebaseLanguage)]
const BUILD_TOOLS = [...Object.keys(BuildTool)]

type InputProps = {
  project: Project
  onUpdate: (payload: Project) => void
}

type FormState = {
  id: string
  name: string
  fisma: string
  codebases: Record<string, Codebase>
}

const TeamViewProjectCreateCard = ({
  project,
  onUpdate,
}: InputProps): JSX.Element => {
  // reducer for the form state
  const [formInput, setFormInput] = React.useReducer(
    (state: FormState, newState: FormState) => ({ ...state, ...newState }),
    {
      ...defaultProject,
      id: project.id,
      name: project.name,
      codebases: project.codebases,
    }
  )

  /* eslint-disable react-hooks/exhaustive-deps */
  // the dependency array for this useEffect does not need formInput.
  React.useEffect(() => {
    onUpdate(formInput)
  }, [formInput])
  /* eslint-enable react-hooks/exhaustive-deps */

  // function that handles change events on form inputs.
  const handleInput = (
    evt: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = evt.currentTarget
    const payload = { ...formInput, [name]: value }
    setFormInput(payload)
  }

  // function that handles change events on form inputs for codebases.
  const handleInputCodebase = ({
    name,
    value,
  }: {
    name: string
    value: Codebase
  }) => {
    setFormInput({
      ...formInput,
      codebases: {
        ...formInput.codebases,
        [name]: value,
      },
    })
  }

  // function that handles adding a new codebase to the project with a new id.
  const handleAddCodebase = () => {
    setFormInput({
      ...formInput,
      codebases: {
        ...formInput.codebases,
        // the temporary id is only used read the codebase data from the app
        // state, and is not included in the data of the request to the server.
        [uuidv4()]: { ...defaultCodebase },
      },
    })
  }

  return (
    <Card
      sx={{
        position: 'relative',
        pt: 2,
      }}
    >
      <CardHeader
        sx={{
          position: 'absolute',
          top: 0,
          right: 0,
        }}
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
        <TextField
          fullWidth
          id="projectName"
          label="Project Name"
          name="name"
          onChange={handleInput}
          required
          value={formInput?.name}
          variant="standard"
          InputProps={{
            sx: {
              '& .Mui-disabled': {
                color: 'text.primary',
              },
            },
          }}
          sx={{ mb: 3 }}
        />
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
              {Object.keys(project.codebases).length || 0} Codebases
            </Typography>
          </Grid>
        </Grid>

        {formInput?.codebases &&
          Object.entries(formInput.codebases).map(
            ([key, codebase], index, array) => {
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
                      ml: 2,
                      width: '100%',
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      spacing: 2,
                    }}
                  >
                    <Typography
                      variant="h6"
                      flexGrow={1}
                      sx={{ mr: 4, fontWeight: 600, color: 'text.primary' }}
                    >
                      <TextField
                        fullWidth
                        id="codebaseName"
                        label="Codebase Name"
                        name={`codebases[${key}].name`}
                        onChange={(event) =>
                          handleInputCodebase({
                            name: key,
                            value: { ...codebase, name: event.target.value },
                          })
                        }
                        required
                        value={codebase.name}
                        variant="standard"
                        InputProps={{
                          sx: {
                            '& .Mui-disabled': {
                              color: 'text.primary',
                            },
                          },
                        }}
                        sx={{
                          width: '100%',
                        }}
                      />
                    </Typography>
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
                              name="language"
                              onChange={(event) =>
                                handleInputCodebase({
                                  name: key,
                                  value: {
                                    ...codebase,
                                    [event.target.name]: event.target
                                      .value as CodebaseLanguage,
                                  },
                                })
                              }
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
                              name="buildTool"
                              onChange={(event) =>
                                handleInputCodebase({
                                  name: key,
                                  value: {
                                    ...codebase,
                                    [event.target.name]: event.target
                                      .value as CodebaseLanguage,
                                  },
                                })
                              }
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
            }
          )}
        <CardActions
          sx={{
            width: '100%',
            display: 'flex',
            alignItems: 'flex-end',
            flexFlow: 'row',
            justifyContent: 'flex-end',
          }}
        >
          <Button
            sx={{ mt: 0, mb: 0, ml: 1 }}
            variant="contained"
            size="small"
            color="secondary"
            onClick={handleAddCodebase}
          >
            Add Codebase
          </Button>
        </CardActions>
      </CardContent>
    </Card>
  )
}

export default TeamViewProjectCreateCard
