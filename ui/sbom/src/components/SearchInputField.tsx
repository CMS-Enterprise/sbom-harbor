import * as React from 'react'
import FormControl from '@mui/material/FormControl'
import FormHelperText from '@mui/material/FormHelperText'
import IconButton from '@mui/material/IconButton'
import Input from '@mui/material/Input'
import InputAdornment from '@mui/material/InputAdornment'
import InputLabel from '@mui/material/InputLabel'
import SearchIcon from '@mui/icons-material/Search'

type SearchInputFieldProps = {
  label: string
  onChange: (event: React.ChangeEvent<HTMLInputElement>) => void
  value: string
  placeholder?: string
  disabled?: boolean
  error?: string
  helperText?: string
  margin?: 'none' | 'dense' | 'normal'
  fullWidth?: boolean
  name?: string
  readonly?: boolean
  required?: boolean
  type?: string
  variant?: 'standard' | 'outlined' | 'filled'
}

const SearchInputField = ({
  onChange,
  value,
  name,
  label,
  placeholder,
  required,
  error,
  helperText,
  ...props
}: SearchInputFieldProps) => (
  <FormControl
    variant="outlined"
    margin="normal"
    fullWidth
    required={required}
    error={!!error}
    {...props}
  >
    <InputLabel htmlFor={name}>{label}</InputLabel>
    <Input
      autoComplete="off"
      margin="none"
      id={name}
      name={name}
      required={required}
      onChange={onChange}
      value={value}
      placeholder={placeholder}
      endAdornment={
        <InputAdornment position="end">
          <IconButton
            aria-label="search"
            onClick={() => ({})}
            onMouseDown={() => ({})}
            edge="end"
          >
            <SearchIcon />
          </IconButton>
        </InputAdornment>
      }
    />
    {error && <FormHelperText error>{error}</FormHelperText>}
    {helperText && <FormHelperText>{helperText}</FormHelperText>}
  </FormControl>
)

export default SearchInputField
