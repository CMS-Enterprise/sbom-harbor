import * as React from 'react'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Container from '@mui/material/Container'
import Paper from '@mui/material/Paper'
import TeamForm from './TeamForm'

export default function Checkout() {
  const handleSave = () => {
    return
  }

  return (
    <Container component="main" maxWidth="sm" data-testid="team">
      <Paper
        variant="outlined"
        sx={{ mt: { xs: 3, md: 6 }, p: { xs: 2, md: 3 } }}
      >
        <React.Fragment>
          <React.Fragment>
            <TeamForm />
            <Box sx={{ display: 'flex', justifyContent: 'flex-end' }}>
              <Button
                variant="contained"
                onClick={handleSave}
                sx={{ mt: 3, ml: 1 }}
              >
                Save
              </Button>
            </Box>
          </React.Fragment>
        </React.Fragment>
      </Paper>
    </Container>
  )
}
