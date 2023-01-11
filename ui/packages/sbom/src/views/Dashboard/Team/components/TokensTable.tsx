/**
 * A component that renders a table of a teams tokens and allows for editing.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TokensTable
 */
import * as React from 'react'
import AddIcon from '@mui/icons-material/Add'
import Box from '@mui/material/Box'
import Card from '@mui/material/Card'
import Fab from '@mui/material/Fab'
import Typography from '@mui/material/Typography'
import {
  DataGrid,
  GridActionsCellItem,
  GridColumns,
  GridRowId,
  GridRowParams,
} from '@mui/x-data-grid'
import deleteToken from '@/api/deleteToken'
import updateToken from '@/api/updateToken'
import DateLocaleString from '@/components/DateLocaleString'
import useAlert from '@/hooks/useAlert'
import { useDialog } from '@/hooks/useDialog'
import { useAuthState } from '@/hooks/useAuth'
import { Token } from '@/types'
import TokenCreateDialog from './TokenCreateDialog'

type InputProps = {
  teamId: string
  tokens: Token[]
}

type RenderCellProps = {
  row: Token
}

type TokenUpdatePayload = {
  name?: string
  enabled?: boolean
  expires?: string
}

/**
 * A component that renders a table of team members with their details.
 * @param {InputProps} props Input props for the TeamMembersTable component.
 * @param {UserTableRowType[]} props.members - The list of team members.
 * @returns {JSX.Element} A component that renders a datagrid table of team members.
 */
const TokensTable = ({ teamId, tokens }: InputProps) => {
  const { setAlert } = useAlert()
  const { jwtToken } = useAuthState()
  const [openDialog] = useDialog()

  // set the initial state of the rows to the tokens passed in as props.
  const [rows, setRows] = React.useState<Token[]>(() => tokens)

  // update the rows state if the tokens prop changes.
  React.useEffect(() => setRows(tokens), [tokens])

  /**
   * Callback that makes a request API to delete a token from the team. If the
   * request is successful, the token is removed from the table. Otherwise, if
   * the request fails, or has a non-200 status code, an alert is shown and the
   * row corresponding to this token is not removed from the tokens table. This
   * callback is triggered when "Delete" is clicked from the actions drowdown.
   * @param {GridRowId} params - The params for the row that was selected.
   */
  const handleDeleteToken = React.useCallback(
    (id: GridRowId) => () => {
      const abortController = new AbortController()
      // define async function to delete the token
      const fetchDelete = async () => {
        try {
          const response = await deleteToken({
            tokenId: id as string,
            teamId,
            jwtToken,
            abortController,
          })
          if (!response.ok) {
            throw new Error('Failed to delete token')
          }
          // remove the token row from the table
          setRows((prevRows) => prevRows.filter((row) => row.id !== id))
          setAlert({
            message: 'Token deleted successfully.',
            severity: 'success',
          })
        } catch (error) {
          console.error('Error deleting token:', error)
          setAlert({
            message: 'Failed to delete token',
            severity: 'error',
          })
        }
      }
      // call the async function to make the request
      fetchDelete()
      // return cleanup function to cancel the request
      return () => abortController.abort()
    },
    /* eslint-disable react-hooks/exhaustive-deps */
    [teamId]
    /* eslint-enable react-hooks/exhaustive-deps */
  )

  /**
   * Callback that makes a request API to update a token.
   */
  const handleUpdateToken = (
    id: GridRowId,
    { name, enabled, expires }: TokenUpdatePayload
  ) => {
    const abortController = new AbortController()
    // define async function to update the token
    const fetchUpdate = async () => {
      try {
        const response = await updateToken({
          tokenId: id as string,
          teamId,
          jwtToken,
          abortController,
          token: {
            name,
            enabled,
            expires,
          },
        })
        if (!response.ok) {
          throw new Error('Failed to update token')
        }
        // update the token row in the table
        setRows((prevRows) => {
          const index = prevRows.findIndex((row) => row.id === id)
          const prev = prevRows[index]
          const newToken = {
            name: typeof name !== 'undefined' && name !== '' ? name : prev.name,
            enabled: typeof enabled !== 'undefined' ? enabled : prev.enabled,
            expires: typeof expires !== 'undefined' ? expires : prev.expires,
          }
          const newRows = [...prevRows]
          newRows.splice(index, 1, { ...prevRows[index], ...newToken })
          return newRows
        })
        setAlert({
          message: 'Token updated successfully.',
          severity: 'success',
        })
      } catch (error) {
        console.error('Error updating token:', error)
        setAlert({
          message: 'Failed to update token',
          severity: 'error',
        })
      }
    }
    // call the async function to make the request
    fetchUpdate()
    // return cleanup function to cancel the request
    return () => abortController.abort()
  }

  /**
   * Callback that makes a request API to disable an enabled token.
   */
  const handleDisableToken = React.useCallback(
    (id: GridRowId) => () => handleUpdateToken(id, { enabled: false }),
    /* eslint-disable react-hooks/exhaustive-deps */
    []
    /* eslint-enable react-hooks/exhaustive-deps */
  )

  /**
   * Callback that makes a request API to enable a disabled token.
   */
  const handleEnableToken = React.useCallback(
    (id: GridRowId) => () => handleUpdateToken(id, { enabled: true }),
    /* eslint-disable react-hooks/exhaustive-deps */
    []
    /* eslint-enable react-hooks/exhaustive-deps */
  )

  const handleTokenAdded = React.useCallback(
    (token: Token) => {
      setRows((prevRows) => [...prevRows, token])
    },
    /* eslint-disable react-hooks/exhaustive-deps */
    []
    /* eslint-enable react-hooks/exhaustive-deps */
  )

  /**
   * Callback that displays the pop-up dialog to create a new token.
   */
  const openTokenDialog = React.useCallback(
    () => {
      openDialog({
        children: (
          <TokenCreateDialog teamId={teamId} onTokenAdded={handleTokenAdded} />
        ),
      })
    },
    /* eslint-disable react-hooks/exhaustive-deps */
    []
    /* eslint-enable react-hooks/exhaustive-deps */
  )

  const columns = React.useMemo<GridColumns<Token>>(
    () => [
      {
        flex: 0.35,
        field: 'name',
        headerName: 'Description',
        renderCell: ({ row: { name, id } }: RenderCellProps): JSX.Element => (
          <Typography variant="body2">{name || id}</Typography>
        ),
      },
      {
        flex: 0.125,
        field: 'created',
        headerName: 'Created',
        renderCell: ({ row: { created } }: RenderCellProps): JSX.Element => (
          <DateLocaleString date={new Date(created)} />
        ),
        defaultSort: 'desc',
      },
      {
        flex: 0.125,
        field: 'expires',
        headerName: 'Expires',
        renderCell: ({ row: { expires } }: RenderCellProps): JSX.Element => (
          <DateLocaleString date={new Date(expires)} />
        ),
      },
      {
        flex: 0.125,
        field: 'expired',
        headerName: 'Expired?',
        renderCell: ({ row: { expires } }: RenderCellProps): JSX.Element => {
          const isExpired = new Date() > new Date(expires)
          return (
            <Typography
              variant="caption"
              textAlign="center"
              sx={{ color: isExpired ? 'red' : 'green', width: '100%' }}
            >
              {isExpired ? 'Expired' : 'Active'}
            </Typography>
          )
        },
      },
      {
        flex: 0.125,
        field: 'enabled',
        headerName: 'Enabled?',
        renderCell: ({ row: { enabled } }: RenderCellProps): JSX.Element => (
          <Typography
            variant="caption"
            textAlign="center"
            sx={{ color: !enabled ? 'red' : 'green', width: '100%' }}
          >
            {enabled ? 'Enabled' : 'Disabled'}
          </Typography>
        ),
      },
      {
        field: 'actions',
        type: 'actions',
        width: 80,
        getActions: (params: GridRowParams<Token>): JSX.Element[] =>
          [
            <GridActionsCellItem
              key="delete"
              label="Delete"
              onClick={handleDeleteToken(params.id)}
              showInMenu
            />,
          ].concat([
            params.row.enabled ? (
              <GridActionsCellItem
                key="disable"
                label="Disable Token"
                onClick={handleDisableToken(params.id)}
                showInMenu
              />
            ) : (
              <GridActionsCellItem
                key="enable"
                label="Enable Token"
                onClick={handleEnableToken(params.id)}
                showInMenu
              />
            ),
          ]),
      },
    ],
    [handleDeleteToken, handleDisableToken, handleEnableToken]
  )

  return (
    <>
      <Card>
        <DataGrid
          rows={rows}
          columns={columns}
          pagination={undefined}
          autoHeight
          disableSelectionOnClick
          hideFooter
          sortModel={[
            {
              field: 'created',
              sort: 'desc' as const,
            },
          ]}
        />
      </Card>
      <Box
        sx={{
          display: 'flex',
          justifyContent: 'flex-end',
          width: '100%',
          mt: -2,
          mb: -2,
          '& > :not(style)': { m: 1 },
        }}
      >
        <Fab color="primary" aria-label="add" onClick={openTokenDialog}>
          <AddIcon />
        </Fab>
      </Box>
    </>
  )
}

TokensTable.displayName = 'TokensTable'

export default TokensTable
