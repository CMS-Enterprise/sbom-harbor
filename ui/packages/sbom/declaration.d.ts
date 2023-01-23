/**
 * Global type declarations
 */

// Image module declarations
declare module '*.png'
declare module '*.svg'

// Style module declaration
declare module '*.css' {
  const classes: { [key: string]: string }
  export default classes
}

/**
 * Generalizable way to require at least one of a set of properties is provided.
 * @see https://stackoverflow.com/a/49725198/1526037
 * @author KPD (https://stackoverflow.com/users/2077574/kpd)
 */
type RequireAtLeastOne<T, Keys extends keyof T = keyof T> = Pick<
  T,
  Exclude<keyof T, Keys>
> &
  {
    [K in Keys]-?: Required<Pick<T, K>> & Partial<Pick<T, Exclude<Keys, K>>>
  }[Keys]

/**
 * Partial but not absolute way to require that one and only one is provided.
 * @see https://stackoverflow.com/a/49725198/1526037
 * @author KPD (https://stackoverflow.com/users/2077574/kpd)
 */
type RequireOnlyOne<T, Keys extends keyof T = keyof T> = Pick<
  T,
  Exclude<keyof T, Keys>
> &
  {
    [K in Keys]-?: Required<Pick<T, K>> &
      Partial<Record<Exclude<Keys, K>, undefined>>
  }[Keys]

/**
 * Generic Error Callback type
 */
type ErrorCallbackType = (err: Error) => void

/**
 * Date Interface
 */
interface Date {
  /**
   * Give a more precise return type to the method `toISOString()`:
   * https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString
   */
  toISOString(): TDateISO
}

type TYear = `${number}${number}${number}${number}`
type TMonth = `${number}${number}`
type TDay = `${number}${number}`
type THours = `${number}${number}`
type TMinutes = `${number}${number}`
type TSeconds = `${number}${number}`
type TMilliseconds = `${number}${number}${number}`

/**
 * Represent a string like `2021-01-08`
 */
type TDateISODate = `${TYear}-${TMonth}-${TDay}`

/**
 * Represent a string like `14:42:34.678`
 */
type TDateISOTime = `${THours}:${TMinutes}:${TSeconds}.${TMilliseconds}`

/**
 * Represent a string like `2021-01-08T14:42:34.678Z` (format: ISO 8601).
 *
 * It is not possible to type more precisely (list every possible values for months, hours etc) as
 * it would result in a warning from TypeScript:
 *   "Expression produces a union type that is too complex to represent. ts(2590)
 */
type TDateISO = `${TDateISODate}T${TDateISOTime}Z`

/**
 * Represent a string like `2021-01-08T14:42:34.678` (format: ISO 8601).
 * The server needs the string to be formatted this way.
 */
type TDateISOWithoutZ = `${TDateISODate}T${TDateISOTime}`
