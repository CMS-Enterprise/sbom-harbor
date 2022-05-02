/**
 * @module @cyclonedx/ui/sbom/components/Copyright
 * Copyright component used in the footer.
 */
import * as React from 'react'
import Link from '@mui/material/Link'
import Typography, { TypographyProps } from '@mui/material/Typography'
import Container from '@mui/material/Container'
import { SxProps } from '@mui/material/styles'

const Copyright = ({
  typograpyProps,
  sx,
}: {
  typograpyProps?: TypographyProps
  sx?: SxProps
}): JSX.Element => (
  <Container sx={{ m: 1, p: 2, ...sx }}>
    <Typography
      variant="body2"
      color="text.secondary"
      align="center"
      {...typograpyProps}
    >
      {'Copyright © '}
      <Link color="inherit" href="https://aquia.us/">
        Aquia, Inc.
      </Link>
      {` ${new Date().getFullYear()}.`}
    </Typography>
  </Container>
)

export default Copyright
