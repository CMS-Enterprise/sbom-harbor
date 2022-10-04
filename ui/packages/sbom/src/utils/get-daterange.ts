/**
 * Helper function that returns a formatted date range string.
 * @module @cyclone-dx/ui/sbom/utils/get-initials
 */
import { format, addDays, differenceInDays } from 'date-fns'

/**
 * @function getDateRange
 * @param {Date} startDate - The start date of the date range.
 * @param {Date} endDate - The end date of the date range.
 * @returns {string} A formatted string containing the date range.
 */
export const getDateRange = (startDate: Date, endDate: Date) => {
  const days = differenceInDays(endDate, startDate)

  return [...Array(days + 1).keys()].map((i) =>
    format(addDays(startDate, i), 'MM/dd/yyyy')
  )
}
