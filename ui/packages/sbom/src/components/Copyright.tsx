/**
 * Copyright component used in the footer.
 * @module @cyclonedx/ui/sbom/components/Copyright
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
      <Link color="inherit" href="https://cms.gov/">
        Centers for Medicare & Medicaid Services
      </Link>
      {` ${new Date().getFullYear()}.`}
    </Typography>
  </Container>
)

export default Copyright
