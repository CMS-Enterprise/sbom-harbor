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
import CircularProgress from '@mui/material/CircularProgress'
import getUsersSearch from '@/api/getUsersSearch'
import TextField from '@mui/material/TextField'

const THROTTLE_TIMEOUT = 750

type State = {
  inputValue?: string
  loading?: boolean
  options?: Array<string>
  value?: string | null
}

const defaultFormState = {
  inputValue: '',
  loading: false,
  options: [],
  value: null,
}

const UserSearchInput = ({
  control,
  name,
  ...rest
}: {
  control: Control<FieldValues, object>
  name: string
  [key: string]: unknown
}): JSX.Element => {
  const [state, dispatchSetState] = React.useReducer(
    (state: State, newState: State) => ({
      ...state,
      ...newState,
    }),
    { ...defaultFormState }
  )

  const fetch = React.useMemo(
    () =>
      throttle(
        async (
          input: string,
          active: boolean,
          abortController: AbortController
        ) => {
          try {
            if (!active) {
              return
            }
            dispatchSetState({ loading: true })
            const results = await getUsersSearch(input, abortController)
            const optionsSet = new Set<string>()
            if (results) {
              results.forEach((r) => optionsSet.add(r))
            }
            const newOptions = [...optionsSet]
            dispatchSetState({ options: newOptions, loading: false })
          } catch (error: unknown) {
            if (error instanceof Error) console.warn(error)
          }
        },
        THROTTLE_TIMEOUT
      ),
    []
  )

  React.useEffect(() => {
    let active = true
    if (state.inputValue === '' || state.value === state.inputValue) {
      active = false
      dispatchSetState({ options: state.value ? [state.value] : [] })
    }
    const abortController = new AbortController()
    if (active && state.inputValue) {
      fetch(state.inputValue, active, abortController)
    } else {
      dispatchSetState({ loading: false })
    }
    return () => {
      active = false
      abortController.abort()
    }
  }, [state.value, state.inputValue])

  // TODO: filter out illegal characters
  const handleInputChange = React.useCallback(
    (event: React.SyntheticEvent<Element, Event>, newValue: string) => {
      dispatchSetState({ inputValue: newValue })
    },
    []
  )

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
          value={state.value}
          autoComplete
          clearOnBlur={false}
          clearOnEscape
          freeSolo
          filterOptions={(options) => options}
          fullWidth
          includeInputInList
          loading={state.loading}
          loadingText="Loading..."
          options={state.options || []}
          getOptionLabel={(option = '') => option}
          isOptionEqualToValue={(option, value) =>
            option === value || option === ''
          }
          onChange={(_, newValue: string | null = '') => {
            const newOptions: string[] = state.options || []
            dispatchSetState({
              options: [
                ...new Set(newValue ? [newValue, ...newOptions] : newOptions),
              ],
              value: newValue,
            })
            field.onChange(newValue)
          }}
          onInputChange={handleInputChange}
          renderInput={(params) => (
            <TextField
              {...rest}
              {...params}
              variant="outlined"
              fullWidth
              InputProps={{
                ...params.InputProps,
                endAdornment: (
                  <React.Fragment>
                    {state.loading ? (
                      <CircularProgress color="inherit" size={20} />
                    ) : null}
                    {params.InputProps.endAdornment}
                  </React.Fragment>
                ),
              }}
            />
          )}
        />
      )}
    />
  )
}

export default UserSearchInput
