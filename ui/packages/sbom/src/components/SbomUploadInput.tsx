import * as React from 'react'
import { useLoaderData } from 'react-router-dom'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import uploadSBOM from '@/api/uploadSBOM'
import useAlert from '@/hooks/useAlert'
import { Team } from '@/types'

type Props = {
  teamId: string
  projectId: string
  codebaseId: string
}

const SbomUploadInput = ({ teamId, projectId, codebaseId }: Props) => {
  const { setAlert } = useAlert()
  const { tokens = {} } = useLoaderData() as Team

  const uploadFile = async (fileContents: string) => {
    const abortController = new AbortController()
    try {
      // filter the active tokens, and get the first one to use for the upload
      const [{ token = '' } = {}] = Object.values(tokens).filter(
        ({ enabled, expires }) => enabled && new Date(expires) > new Date()
      )
      // if there are no enabled and unexpired tokens, throw an error
      if (!token) {
        throw new Error('No active tokens found.')
      }
      // make the request to upload the SBOM file
      await uploadSBOM({
        abortController,
        jwtToken: token,
        fileContents,
        codebaseId,
        projectId,
        teamId,
      })
      // set the alert to show the success message
      setAlert({
        severity: 'success',
        message: 'File uploaded successfully',
      })
    } catch (err) {
      console.error(err)
      setAlert({
        severity: 'error',
        message: (err as Error).message,
      })
    }
  }

  const handleChange = React.useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      try {
        // get the file input
        const file = e.target.files?.[0]
        // verify that the file exist
        if (!file) {
          throw new Error('No file selected.')
        }
        const fileReader = new FileReader()
        // read the file contents as text
        fileReader.readAsText(file, 'UTF-8')
        // handle event when the file is loaded
        fileReader.onload = (event: ProgressEvent<FileReader>) => {
          // get the file contents
          const contents = event.target?.result
          // validate the file contents, or return early if contents are invalid
          if (!contents || typeof contents !== 'string') {
            throw new Error('Error reading file.')
          }
          // upload the file contents
          uploadFile(contents)
        }
      } catch (err) {
        console.error(err)
        setAlert({
          severity: 'error',
          message: (err as Error).message,
        })
      }
    },
    /* eslint-disable react-hooks/exhaustive-deps */
    []
    /* eslint-enable react-hooks/exhaustive-deps */
  )

  const fileInputId = `${teamId}-${projectId}-${codebaseId}-file-input`

  return (
    <Container className="file-select" role="section">
      <Grid container spacing={1} sx={{ flexFlow: 'column' }}>
        <Grid item>
          <label htmlFor={fileInputId} role="heading">
            Upload SBOM
          </label>
        </Grid>
        <Grid item>
          <input
            id={fileInputId}
            name={fileInputId}
            type="file"
            onChange={handleChange}
            accept=".json"
            role="button"
            data-testid="file-input"
          />
        </Grid>
      </Grid>
    </Container>
  )
}

export default SbomUploadInput
