/**
 * A component for rendering a card with a button to create a new team.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/DashboardTeamCreateCard
 */
import * as React from 'react'
import Button from '@mui/material/Button'
import Card from '@mui/material/Card'
import { CenteredCardContent } from '@/components/mui/CardContent'

type InputProps = {
  onClick: React.MouseEventHandler<HTMLButtonElement>
}

const DashboardTeamCreationCard = ({ onClick }: InputProps) => {
  return (
    <Card sx={{ height: '168px' }}>
      <CenteredCardContent>
        <Button
          variant="outlined"
          sx={{ p: (theme) => theme.spacing(1.75, 5.5) }}
          onClick={onClick}
        >
          Create new Team
        </Button>
      </CenteredCardContent>
    </Card>
  )
}

export default DashboardTeamCreationCard
