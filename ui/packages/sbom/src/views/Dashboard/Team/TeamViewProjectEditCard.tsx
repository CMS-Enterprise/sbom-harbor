import * as React from 'react'
import Button from '@mui/material/Button'
import Card from '@mui/material/Card'
import CardContent from '@mui/material/CardContent'
import PlusOutline from 'mdi-material-ui/PlusOutline'
import CustomAvatar from '@/components/mui/Avatar'

const TeamViewProjectCreationCard = () => {
  return (
    <Card>
      <CardContent
        sx={{
          display: 'flex',
          textAlign: 'center',
          alignItems: 'center',
          flexDirection: 'column',
        }}
      >
        <CustomAvatar skin="light" sx={{ width: 56, height: 56, mb: 2 }}>
          <PlusOutline sx={{ fontSize: '2rem' }} />
        </CustomAvatar>
        <Button
          variant="outlined"
          sx={{ p: (theme) => theme.spacing(1.75, 5.5) }}
        >
          Add new Project
        </Button>
      </CardContent>
    </Card>
  )
}

export default TeamViewProjectCreationCard
