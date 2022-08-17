/**
 * Helper function that returns the initials from full name.
 * @module @cyclone-dx/ui/sbom/utils/get-initials
 */

/**
 * @function getInitials
 * @param {string} name - The user's full name to get initials from.
 * @returns {string} A string containing the user's initials.
 */
export const getInitials = (name: string): string =>
  name
    .split(/\s/)
    .reduce((response, word) => (response += word.slice(0, 1)), '')
