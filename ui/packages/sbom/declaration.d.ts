/**
 * @module @cyclonedx/ui/sbom/types
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
