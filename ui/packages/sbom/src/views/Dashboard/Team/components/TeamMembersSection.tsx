/**
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/components/TeamMemberSection
 */
import * as React from 'react'
import FormControl from '@mui/material/FormControl'
import Grid from '@mui/material/Grid'
import IconButton from '@mui/material/IconButton'
import Input from '@mui/material/Input'
import InputAdornment from '@mui/material/InputAdornment'
import InputLabel from '@mui/material/InputLabel'
import Typography from '@mui/material/Typography'
import AddCircleIcon from '@mui/icons-material/AddCircleOutline'
import TeamMemberReadOnlyField from '@/views/Dashboard/Team/components/TeamMemberReadOnlyField'
import { TeamMember } from '@/types'

type TeamMembersSectionProps = {
  members?: [string, TeamMember][]
  // TODO: add validation of email
  newEmail?: string
  title: string
  name: string
  handleAdd: () => void
  handleRemove: (event: React.MouseEvent<HTMLButtonElement>) => void
  handleChange: (event: React.ChangeEvent<HTMLInputElement>) => void
}

/**
 * A component that renders a read-only field for a team member
 * with button to remove the member from the team on the right.
 * @param {TeamMembersSectionProps} props
 * @param {TeamMember[]} props.members a list of objects representing team
 *  members to be included in the team once the form is submitted to update it.
 * @param {string} props.newEmail the email of a new member to be added
 * @param {string} props.title the title of the section (e.g. 'Admins' or 'Members')
 * @param {string} props.name the name of the team
 * @param {Function} props.handleAdd function to adds the user to the team
 * @param {Function} props.handleRemove function to remove the user from the team
 * @param {Function} props.handleChange function to handle changes to the form
 * @returns {JSX.Element}
 */
const TeamMembersSection = ({
  members = [],
  newEmail = '',
  title,
  name,
  handleAdd,
  handleRemove,
  handleChange,
}: TeamMembersSectionProps): JSX.Element => {
  /**
   * Wrapper for the handleAdd function that accepts a keyboard event argument
   *  to prevent the event from bubbling up to the form and causing the form to
   *  submit if the enter key is pressed. Instead, when the enter key is pressed,
   *  this adds the new email to the list of member emails in the team edit form.
   * @param {KeyboardEvent} event keyboard event to check if enter key was pressed.
   */
  const handleAddWrapper = (
    event: React.KeyboardEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    if (event.key === 'Enter') {
      event?.preventDefault()
      handleAdd()
    }
  }

  return (
    <>
      <Typography
        sx={{ textTransform: 'capitalize' }}
        gutterBottom
        variant="h5"
      >
        {title}:
      </Typography>
      <Grid container spacing={1} sx={{ mb: 3 }}>
        {members.map(([id, { email }], index) => (
          <Grid item xs={12} key={id}>
            <FormControl fullWidth variant="standard" disabled margin="none">
              <TeamMemberReadOnlyField
                index={index}
                email={email}
                handleRemove={handleRemove}
              />
            </FormControl>
          </Grid>
        ))}
        <Grid item xs={12}>
          <FormControl fullWidth variant="standard" size="small">
            <InputLabel htmlFor={`${name}`}>
              <Typography sx={{ textTransform: 'capitalize' }}>
                Add email
              </Typography>
            </InputLabel>
            <Input
              autoComplete="off"
              margin="none"
              id={`${name}`}
              name={`${name}`}
              required
              fullWidth
              onChange={handleChange}
              value={newEmail}
              onKeyDown={handleAddWrapper}
              endAdornment={
                <InputAdornment position="end" sx={{ pr: 1 }}>
                  <IconButton
                    aria-label="add"
                    data-value={newEmail}
                    onClick={handleAdd}
                    onMouseDown={handleAdd}
                    edge="end"
                  >
                    <AddCircleIcon />
                  </IconButton>
                </InputAdornment>
              }
            />
          </FormControl>
        </Grid>
      </Grid>
    </>
  )
}

export default TeamMembersSection
