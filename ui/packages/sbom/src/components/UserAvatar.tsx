/**
 * A component that renders a user's avatar image or initials.
 * @module @cyclonedx/ui/sbom/components/UserAvatar
 */
import * as React from 'react'
import { styled } from '@mui/material/styles'
import Avatar from '@/components/mui/Avatar'
import { getInitials } from '@/utils/get-initials'
import { TeamMemberTableRow } from '@/types'

const StyledAvatar = styled(Avatar, {
  name: 'Avatar',
  slot: 'Root',
  shouldForwardProp: (prop) => prop !== 'sx' && prop !== 'skin',
  overridesResolver: (props, styles) => [
    styles.root,
    props.skin === 'light' && { sx: { fontSize: '.8rem' } },
  ],
})({
  mr: 3,
  height: 34,
  width: 34,
})

/**
 * Component that renders a user avatar with the user's avatar image if
 *  it exists, otherwise it renders the user's initials from their name.
 * @param {InputProps} props Input props for the UserAvatar component.
 * @param {string} [props.avatarSrc] An optional image source URL for the avatar.
 * @param {string} props.email The user's email address.
 * @param {string} props.name The user's full name.
 * @returns {JSX.Element} A component that renders a table row.
 */
const UserAvatar = ({ avatarSrc, email, name }: TeamMemberTableRow) => {
  // if there is a source url for the avatar image, render it.
  if (avatarSrc) {
    return <StyledAvatar src={avatarSrc} />
  }

  // otherwise, render the user's initials in the avatar.
  return (
    <StyledAvatar
      sx={{
        fontSize: '.9rem',
        textTransform: 'uppercase',
      }}
    >
      {getInitials({ name, email })}
    </StyledAvatar>
  )
}

export default UserAvatar
