/**
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamViewProjectEditCard
 */
import * as React from 'react'
import Button from '@mui/material/Button'
import Card from '@mui/material/Card'
import PlusOutline from 'mdi-material-ui/PlusOutline'
import Avatar from '@/components/mui/Avatar'
import { CenteredCardContent } from '@/components/mui/CardContent'

const TeamViewProjectCreationCard = (): JSX.Element => (
  <Card>
    <CenteredCardContent>
      <Avatar skin="light" sx={{ width: 56, height: 56, mb: 2 }}>
        <PlusOutline sx={{ fontSize: '2rem' }} />
      </Avatar>
      <Button
        variant="outlined"
        sx={{ p: (theme) => theme.spacing(1.75, 5.5) }}
      >
        Add new Project
      </Button>
    </CenteredCardContent>
  </Card>
)

export default TeamViewProjectCreationCard
