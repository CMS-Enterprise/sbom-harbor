import dateAsISOWithoutZ from '@/utils/dateAsISOWithoutZ'

/**
 * Temporary helper function until timestamps are fixed on the server..
 * @module @cyclonedx/ui/sbom/utils/formatTimestampForServer
 * @param {Date} date The date to format.
 * @param {number} daysToAdd The number of days to add to the date.
 * @returns {TDateISOWithoutZ} The formatted date with the "Z" ending removed.
 */
const formatTimestampForServer = (
  daysToAdd: number,
  date: Date = new Date()
): TDateISOWithoutZ => {
  return dateAsISOWithoutZ(new Date(date.setDate(date.getDate() + daysToAdd)))
}

export default formatTimestampForServer
