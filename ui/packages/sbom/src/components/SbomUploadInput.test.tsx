import * as React from 'react'
import { v4 as uuidv4 } from 'uuid'
import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import SbomUploadInput from '@/components/SbomUploadInput'

const inputProps = {
  teamId: uuidv4(),
  projectId: uuidv4(),
  codebaseId: uuidv4(),
}

jest.mock('react-router-dom', () => {
  const date = new Date()
  const created = date.toLocaleDateString()
  const expires = new Date(
    date.setDate(date.getDate() + 1)
  ).toLocaleDateString()
  return {
    ...jest.requireActual('react-router-dom'),
    useLoaderData: () => [
      { id: 'some-token', token: `sbom-api-abcdefg`, created, expires },
    ],
  }
})

test('should render a label and a file input field', async () => {
  render(<SbomUploadInput {...inputProps} />)
  expect(screen.getByRole('button')).toBeInTheDocument()
  expect(screen.getByRole('heading')).toHaveTextContent('Upload SBOM')
})

test('should upload a file when one is selected', async () => {
  render(<SbomUploadInput {...inputProps} />)

  // create a mock SBOM file
  const fileContent = JSON.stringify({
    bomFormat: 'SPDX',
    bomFormatVersion: '2.2',
    bomSpecVersion: '2.2',
    specVersion: '2.2',
  })
  const blob = new Blob([fileContent])
  const file = new File([blob], 'sbom.json', { type: 'application/JSON' })
  File.prototype.text = jest.fn().mockResolvedValueOnce(fileContent)

  // trigger the file upload event
  const user = userEvent.setup()
  user.upload(screen.getByTestId('file-input'), file)

  // assert that fetch was called
  await waitFor(() => expect(fetch).toHaveBeenCalledTimes(1))
})
