/**
 * Calculates a date in the future and returns it as ISO
 * @module @cyclonedx/ui/sbom/utils/getFutureDate
 * @param {Date} date The date to format.
 * @param {number} daysToAdd The number of days to add to the date.
 * @returns {TDateISO} The resultant date as an ISO string.
 */
const getFutureDate = (
  daysToAdd: number,
  date: Date = new Date()
): TDateISO => {
  const result = new Date(date.setDate(date.getDate() + daysToAdd))
  return result.toISOString()
}

export default getFutureDate
