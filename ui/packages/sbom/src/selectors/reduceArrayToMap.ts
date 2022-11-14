/**
 * Reduces an array of items to a map of items by id.
 * @param {Array<T>} array the array of items with ids to be reduced to a map
 * @param {[Object={}]} map accumulator for the reduce function
 * @returns {Record<string, T>} a map of the array items by id
 */
const reduceArrayToMap = <T extends { id: string }>(
  array: T[],
  map: Record<string, T> = {}
): Record<string, T> =>
  array.reduce(
    (acc, item) => ({
      ...acc,
      [item.id]: item,
    }),
    map
  )

export default reduceArrayToMap
