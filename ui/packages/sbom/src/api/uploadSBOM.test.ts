import uploadSBOM from '@/api/uploadSBOM'

test('calls makes a fetch request', async () => {
  // @ts-ignore
  window.fetch.mockResolvedValueOnce({
    ok: true,
    json: async () => ({}),
  })

  await uploadSBOM({
    token: 'some-token',
    teamId: 'some-team',
    codebaseId: 'some-codebase',
    projectId: 'some-project',
    fileContents: '{"bomFormat":"CycloneDX","specVersion": "1.3"}',
  })

  expect(window.fetch).toHaveBeenCalledTimes(1)
})
