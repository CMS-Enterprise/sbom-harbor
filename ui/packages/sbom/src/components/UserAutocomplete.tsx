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

  const fetch = React.useMemo(
    () =>
      throttle(
        async (request, active: boolean, abortController: AbortController) => {
          if (!active) return
          const results = await getUsersSearch(request?.input, abortController)
          let newOptions = [] as Array<string>
          if (value) newOptions = [value]
          if (results) newOptions = [...newOptions, ...results]
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
      setOptions(value ? [value] : [])
      return undefined
    }
    fetch({ input: inputValue }, active, abortController)
    return () => {
      active = false
      abortController.abort()
    }
  }, [value, inputValue, fetch])

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
          autoComplete
          fullWidth
          clearOnBlur={false}
          clearOnEscape
          filterSelectedOptions
          includeInputInList
          value={value}
          options={options}
          filterOptions={(x) => x}
          getOptionLabel={(option = '') => option}
          isOptionEqualToValue={(option, value) =>
            option === value || option === ''
          }
          onChange={(_, newValue: string | null = '') => {
            console.log('onChange', newValue)
            setOptions(newValue ? [newValue, ...options] : options)
            setValue(newValue)
            field.onChange(newValue)
          }}
          onInputChange={(_, newValue: string) => {
            console.log('onInputChange', newValue)
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
