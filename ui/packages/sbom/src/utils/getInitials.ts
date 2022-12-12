/**
 * Helper function that returns the initials from full name.
 * @module @cyclone-dx/ui/sbom/utils/get-initials
 */

type UserPropsForInitials = RequireAtLeastOne<
  {
    name?: string
    email?: string
  },
  'name' | 'email'
>

/**
 * Returns the initials from full name if provided, otherwise
 *  attempts to get initials from the email address and returns that.
 * @param {string} name - User's full name to get initials from (preferred).
 * @param {string} email - User's email to get initials from.
 * @returns {string} A string containing the user's initials.
 */
export const getInitials = ({ name, email }: UserPropsForInitials): string => {
  if ('name' in { name } && name) {
    return name
      .split(/\s/)
      .reduce((response, word) => (response += word.slice(0, 1)), '')
  }

  if ('email' in { email } && email) {
    const id = email.split('@')[0]
    // if there is a dot in the id, return the first letter of each part
    if (id.indexOf('.') !== -1) {
      return id
        .split('.')
        .reduce((response, word) => (response += word.slice(0, 1)), '')
    }
    // otherwise just return the first two letters
    return `${id[0]}${id.length > 1 ? id[1] : ''}`
  }

  // TODO: fix the typing, this shouldn't happen.
  throw new Error('No name or email provided')
}
