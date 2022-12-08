import * as React from 'react'
import { useLoaderData } from 'react-router-dom'
import Container from '@mui/material/Container'
import Grid from '@mui/material/Grid'
import useAlert from '@/hooks/useAlert'
import harborRequest from '@/utils/harborRequest'
import { Team, Token } from '@/types'

type Props = {
  teamId: string
  projectId: string
  codebaseId: string
}

const SbomUploadInput = ({ teamId, projectId, codebaseId }: Props) => {
  const { setAlert } = useAlert()
  const { tokens = {} } = useLoaderData() as Team

  const uploadFile = async (fileContents: string) => {
    try {
      const [{ token = '' } = {}] = Object.values(tokens) as Token[]
      await harborRequest({
        path: `${teamId}/${projectId}/${codebaseId}/sbom`,
        method: 'POST',
        body: JSON.parse(fileContents),
        jwtToken: token,
        children: null,
      })
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

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    // get the file input
    const file = e.target.files?.[0]
    // verify that the file exist
    if (!file) {
      setAlert({
        severity: 'warning',
        message: 'No file selected.',
      })
      // return early if the file does not exist
      return
    }
    const fileReader = new FileReader()
    // read the file contents as text
    fileReader.readAsText(file, 'UTF-8')
    // event handler for when the file is loaded
    fileReader.onload = (e) => {
      const contents = e.target?.result
      // validate the file contents
      if (!contents || typeof contents !== 'string') {
        setAlert({
          severity: 'error',
          message: 'Error reading file.',
        })
        return
      }
      // upload the file contents
      uploadFile(contents as string)
    }
  }

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
