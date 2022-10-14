/**
 * A component that renders a button to submit a form.
 * @module @cyclonedx/ui/sbom/components/forms/SubmitButton
 */
import * as React from 'react'
import Button, { ButtonProps } from '@mui/material/Button'

type InputProps = {
  disabled?: boolean
  label?: string
} & ButtonProps

const SubmitButton = ({ label, ...props }: InputProps) => (
  <Button sx={{ mt: 3, ml: 1, ...props.sx }} {...props}>
    {label}
  </Button>
)

SubmitButton.defaultProps = {
  color: 'primary',
  label: 'Save',
  type: 'submit',
  variant: 'contained',
}

export default SubmitButton
