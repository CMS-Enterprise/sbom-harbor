import { ReactNode } from 'react'

export type ThemeColor =
  | 'primary'
  | 'secondary'
  | 'error'
  | 'warning'
  | 'info'
  | 'success'

export type BlankLayoutProps = {
  children: ReactNode
}
