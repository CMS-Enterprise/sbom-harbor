/**
 * A component for rendering a card with a button to create a new team.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/DashboardTeamCreateCard
 */
import * as React from 'react'
import Button from '@mui/material/Button'
import Card from '@mui/material/Card'
import CardContent from '@mui/material/CardContent'

const DashboardTeamCreationCard = ({ onClick }: { onClick: () => void }) => {
  return (
    <Card sx={{ height: '168px' }}>
      <CardContent
        sx={{
          display: 'flex',
          textAlign: 'center',
          alignItems: 'center',
          justifyContent: 'center',
          flexDirection: 'column',
          height: '100%',
        }}
      >
        <Button
          variant="outlined"
          sx={{ p: (theme) => theme.spacing(1.75, 5.5) }}
          onClick={onClick}
        >
          Create new Team
        </Button>
      </CardContent>
    </Card>
  )
}

export default DashboardTeamCreationCard
