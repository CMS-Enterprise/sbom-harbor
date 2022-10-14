/**
 * An image component that defaults to lazy loading.
 * @module @cyclonedx/ui/sbom/components/Image
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

/**
 * An image component that defaults to lazy loading.
 * @param {ImageProps} props Input props for the Image component.
 * @param {string} props.alt Image alt text.
 * @param {string} [props.loading='lazy'] Optional image loading attribute.
 * @param {string} props.src The image source.
 * @param {string} props.srcSet The image source set.
 * @param {React.CSSProperties} props.sx The image style.
 * @param {number} props.sx.height The image height in pixels.
 * @param {number} props.sx.width The image width in pixels.
 * @returns {JSX.Element} The Image component.
 */
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
