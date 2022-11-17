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

      if (!token) {
        throw new Error('No token available for this team.')
      }

      const res = await harborRequest({
        path: `${teamId}/${projectId}/${codebaseId}/sbom`,
        method: 'POST',
        body: JSON.parse(fileContents),
        jwtToken: token,
        children: null,
      })

      if (res.status === 200) {
        setAlert({
          severity: 'success',
          message: 'File uploaded successfully',
        })
      } else {
        throw new Error('Error uploading file')
      }
    } catch (err) {
      console.error(err)
      setAlert({
        severity: 'error',
        message: (err as Error).message,
      })
    }
  }

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!e?.target?.files) {
      setAlert({
        severity: 'warning',
        message: 'No file selected.',
      })
      return
    }
    // get the first file
    const file = e.target.files?.[0]
    if (!file) {
      setAlert({
        severity: 'warning',
        message: 'No file selected.',
      })
      return
    }
    // read the file contents as text
    const fileReader = new FileReader()
    fileReader.readAsText(file, 'UTF-8')
    fileReader.onload = (e) => {
      const contents = e.target?.result
      if (!contents || typeof contents !== 'string') {
        setAlert({
          severity: 'error',
          message: 'Error reading file.',
        })
        return
      }
      // upload the file
      uploadFile(contents as string)
    }
  }

  const fileInputId = `${teamId}-${projectId}-${codebaseId}-file-input`

  return (
    <Container className="file-select">
      <Grid container spacing={1} sx={{ flexFlow: 'column' }}>
        <Grid item>
          <label htmlFor={fileInputId}>Upload SBOM</label>
        </Grid>
        <Grid item>
          <input
            id={fileInputId}
            name={fileInputId}
            type="file"
            onChange={handleChange}
            accept=".json"
          />
        </Grid>
      </Grid>
    </Container>
  )
}

export default SbomUploadInput
