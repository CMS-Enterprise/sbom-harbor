import * as React from 'react'

const useFetch = (
  url: string
): {
  loading: boolean
  error: Error | null
  data: unknown
} => {
  const [data, setData] = React.useState(null)
  const [error, setError] = React.useState<Error | null>(null)
  const [loading, setLoading] = React.useState(false)

  React.useEffect(() => {
    ;(async function () {
      try {
        setLoading(true)
        const response = await fetch(url)
        const data = await response.json()
        setData(data)
      } catch (error) {
        setError(error as Error)
      } finally {
        setLoading(false)
      }
    })()
  }, [url])

  return { data, error, loading }
}

export default useFetch
