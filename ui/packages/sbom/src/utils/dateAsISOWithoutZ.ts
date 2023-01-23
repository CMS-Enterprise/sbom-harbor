/**
 * Converts a date to ISO without the Z
 * @param {Date} date the date to convert to ISO without the Z
 * @returns {TDateISOWithoutZ} the date as ISO without the Z
 */
const dateAsISOWithoutZ = (date: Date): TDateISOWithoutZ =>
  date.toISOString().replace(/Z$/, '') as TDateISOWithoutZ

export default dateAsISOWithoutZ
