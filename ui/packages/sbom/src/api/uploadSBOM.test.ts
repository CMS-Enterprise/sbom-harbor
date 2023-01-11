import uploadSBOM from '@/api/uploadSBOM'

test('calls makes a single fetch request', async () => {
  await uploadSBOM({
    token: 'some-token',
    teamId: 'some-team',
    codebaseId: 'some-codebase',
    projectId: 'some-project',
    fileContents: '{"bomFormat":"CycloneDX","specVersion": "1.3"}',
  })

  expect(global.fetch).toHaveBeenCalledTimes(1)
})
