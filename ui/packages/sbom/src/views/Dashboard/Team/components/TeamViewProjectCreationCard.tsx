/**
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamViewProjectCreationCard
 */
import * as React from 'react'
import Card from '@mui/material/Card'
import Button from '@mui/material/Button'
import PlusOutline from 'mdi-material-ui/PlusOutline'
import Avatar from '@/components/mui/Avatar'
import { CenteredCardContent } from '@/components/mui/CardContent'

type InputProps = {
  onClick: React.MouseEventHandler<HTMLButtonElement>
}

/**
 * A component for rendering a card with a button to create a new project.
 * @param {InputProps} props The input props for the component.
 * @returns {JSX.Element} The component to render.
 */
const TeamViewProjectCreationCard = (props: InputProps): JSX.Element => (
  <Card>
    <CenteredCardContent>
      <Avatar skin="light" sx={{ width: 56, height: 56, mb: 2 }}>
        <PlusOutline sx={{ fontSize: '2rem' }} />
      </Avatar>
      <Button
        variant="outlined"
        sx={{ p: (theme) => theme.spacing(1.75, 5.5) }}
        {...props}
      >
        Add new Project
      </Button>
    </CenteredCardContent>
  </Card>
)

export default TeamViewProjectCreationCard
