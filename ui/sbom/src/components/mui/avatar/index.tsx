/**
 * @module @cyclonedx/ui/sbom/components/mui/avatar
 */
import * as React from 'react'
import MuiAvatar from '@mui/material/Avatar'
import { lighten, useTheme } from '@mui/material/styles'
import { CustomAvatarProps } from './types'
import { ThemeColor } from '@/layouts/types'
import useBgColor, { UseBgColorType } from '@/hooks/useBgColor'

const Avatar = React.forwardRef(
  (props: CustomAvatarProps, ref: React.Ref<any>) => {
    const { sx, src, skin, color } = props
    const theme = useTheme()
    const bgColors: UseBgColorType = useBgColor()

    const getAvatarStyles = (
      skin: 'filled' | 'light' | 'light-static' | undefined,
      skinColor: ThemeColor
    ) => {
      if (skin === 'light') {
        return { ...bgColors[`${skinColor}Light`] }
      }
      if (skin === 'light-static') {
        return {
          color: bgColors[`${skinColor}Light`].color,
          backgroundColor: lighten(theme.palette[skinColor].main, 0.88),
        }
      }
      return { ...bgColors[`${skinColor}Filled`] }
    }

    const colors: UseBgColorType = {
      primary: getAvatarStyles(skin, 'primary'),
      secondary: getAvatarStyles(skin, 'secondary'),
      success: getAvatarStyles(skin, 'success'),
      error: getAvatarStyles(skin, 'error'),
      warning: getAvatarStyles(skin, 'warning'),
      info: getAvatarStyles(skin, 'info'),
    }

    return (
      <MuiAvatar
        ref={ref}
        {...props}
        sx={!src && skin && color ? Object.assign(colors[color], sx) : sx}
      />
    )
  }
)

Avatar.defaultProps = {
  skin: 'filled',
  color: 'primary',
}

Avatar.displayName = 'Avatar'

export default Avatar
