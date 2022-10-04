/**
 * A read only text field that displays an email and a remove button.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/components/TeamMemberReadOnlyField
 */
import * as React from 'react'
import IconButton from '@mui/material/IconButton'
import Input from '@mui/material/Input'
import InputAdornment from '@mui/material/InputAdornment'
import RemoveCircleIcon from '@mui/icons-material/RemoveCircleOutline'

type TeamMemberReadOnlyFieldProps = {
  index: number
  email: string
  handleRemove: (event: React.MouseEvent<HTMLButtonElement>) => void
}

/**
 * A component that renders a read-only field for a team member
 * with button to remove the member from the team on the right.
 * @param {TeamMemberReadOnlyFieldProps} props
 * @param props.index
 * @param props.email
 * @param props.handleRemove
 * @returns {JSX.Element}
 */
const TeamMemberReadOnlyField = ({
  index,
  email,
  handleRemove,
}: TeamMemberReadOnlyFieldProps): JSX.Element => (
  <Input
    id={`member-${index}`}
    name={`member-${index}`}
    disabled
    readOnly
    fullWidth
    value={email}
    sx={(theme) => ({
      '& .Mui-disabled': {
        color:
          theme.palette.mode === 'dark'
            ? 'white !important'
            : 'black !important',
        WebkitTextFillColor:
          theme.palette.mode === 'dark'
            ? 'rgba(255, 255, 255, 1) !important'
            : 'rgba(0, 0, 0, 1) !important',
      },
    })}
    endAdornment={
      <InputAdornment position="end" sx={{ pr: 1 }}>
        <IconButton
          aria-label="remove"
          data-value={email}
          onClick={handleRemove}
          onMouseDown={handleRemove}
          edge="end"
        >
          <RemoveCircleIcon />
        </IconButton>
      </InputAdornment>
    }
  />
)

export default TeamMemberReadOnlyField
