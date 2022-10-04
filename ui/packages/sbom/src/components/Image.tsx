/**
 * @module @cyclonedx/ui/sbom/components/Image
 * An image component that defaults to lazy loading.
 */
import * as React from 'react'
import Box from '@mui/material/Box'

type ImageProps = {
  src: HTMLImageElement['src']
  srcSet?: HTMLImageElement['srcset']
  alt?: HTMLImageElement['alt']
  loading?: HTMLImageElement['loading']
  sx?: React.CSSProperties
}

const Image = ({
  alt = '',
  loading = 'lazy',
  src,
  srcSet,
  sx = {},
  sx: { height, width } = {},
}: ImageProps): JSX.Element => (
  <Box sx={sx}>
    <img
      alt={alt}
      loading={loading}
      src={src}
      srcSet={srcSet || `${src} 2x`}
      height={height}
      width={width}
    />
  </Box>
)

export default Image
