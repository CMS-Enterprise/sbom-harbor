/**
 * Component that renders a text input with autocomplete
 *  for searching for users by their email address.
 * @module @cyclonedx/ui/sbom/components/UserAutocomplete
 */
import * as React from 'react'
import {
  Control,
  Controller,
  ControllerRenderProps,
  FieldValues,
} from 'react-hook-form'
import throttle from 'lodash/throttle'
import Autocomplete from '@mui/material/Autocomplete'
import TextField from '@mui/material/TextField'
import getUsersSearch from '@/api/getUsersSearch'

const THROTTLE_TIMEOUT = 750

const UserSearchInput = ({
  control,
  name,
  ...rest
}: {
  control: Control<FieldValues, object>
  name: string
  [key: string]: unknown
}): JSX.Element => {
  const [inputValue, setInputValue] = React.useState('')
  const [value, setValue] = React.useState<string | null>(null)
  const [options, setOptions] = React.useState<Array<string>>([])
  const [loading, setLoading] = React.useState(false)

  const fetch = React.useMemo(
    () =>
      throttle(
        async (request, active: boolean, abortController: AbortController) => {
          if (!active) return
          setLoading(true)
          const results = await getUsersSearch(request?.input, abortController)
          let newOptions = [] as Array<string>
          if (value) newOptions = [value]
          if (results) newOptions = [...newOptions, ...results]
          setLoading(false)
          setOptions(newOptions)
        },
        THROTTLE_TIMEOUT
      ),
    [value]
  )

  React.useEffect(() => {
    const abortController = new AbortController()
    let active = true
    if (inputValue === '') {
      setLoading(false)
      setOptions(value ? [value] : [])
      return undefined
    }
    fetch({ input: inputValue }, active, abortController)
    return () => {
      active = false
      abortController.abort()
    }
  }, [value, inputValue, fetch, loading])

  return (
    <Controller
      name={name}
      control={control}
      render={({
        field,
      }: {
        field: ControllerRenderProps<FieldValues, string>
      }) => (
        <Autocomplete
          {...field}
          id="user-search"
          data-testid="user-search"
          value={value}
          autoComplete
          clearOnBlur={false}
          clearOnEscape
          filterOptions={(x) => x}
          filterSelectedOptions
          fullWidth
          includeInputInList
          loading={loading}
          loadingText="Loading..."
          options={options}
          getOptionLabel={(option = '') => option}
          isOptionEqualToValue={(option, value) =>
            option === value || option === ''
          }
          onChange={(_, newValue: string | null = '') => {
            setOptions(newValue ? [newValue, ...options] : options)
            setValue(newValue)
            field.onChange(newValue)
          }}
          onInputChange={(_, newValue: string) => {
            setInputValue(newValue)
          }}
          renderInput={(params) => (
            <TextField {...rest} {...params} variant="outlined" fullWidth />
          )}
        />
      )}
    />
  )
}

export default UserSearchInput
