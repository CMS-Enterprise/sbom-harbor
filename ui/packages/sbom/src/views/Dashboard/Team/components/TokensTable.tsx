/**
 * A component that renders a table of a teams tokens and allows for editing.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TokensTable
 */
import * as React from 'react'
import AddIcon from '@mui/icons-material/Add'
import MoreVertIcon from '@mui/icons-material/MoreVert'
import Box from '@mui/material/Box'
import Card from '@mui/material/Card'
import Fab from '@mui/material/Fab'
import IconButton from '@mui/material/IconButton'
import Typography from '@mui/material/Typography'
import styled from '@mui/system/styled'
import {
  DataGrid,
  GridActionsCell,
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

/**
 * Hack component used to render a custom, disabled actions menu in the
 * data grid that prevents the forwarding of the "focusElementRef" prop
 * to a raw DOM element. See the last item in the memoized "columns"
 * array in the TokensTable state.
 */
const IconButtonWithExcludedProps = styled(IconButton, {
  // use the default MUI name of IconButton
  name: 'IconButton',
  // don't forward the focusElementRef prop passed from GridActionsCell
  shouldForwardProp: (prop) => prop !== 'focusElementRef',
})({})

type InputProps = {
  teamId: string
  tokens: Token[]
}

type TokenRow = {
  loading: boolean
} & Token

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
 * @param {UserTableRowType[]} props.members The list of team members.
 * @returns {JSX.Element} A component that renders a datagrid table of team members.
 */
const TokensTable = ({ teamId, tokens }: InputProps) => {
  const { setAlert } = useAlert()
  const { jwtToken } = useAuthState()
  const [openDialog] = useDialog()

  // set the initial state of the rows to the tokens passed in as props.
  const [rows, setRows] = React.useState<TokenRow[]>(() =>
    tokens.map((t) => ({ ...t, loading: false }))
  )

  /**
   * Temporarily disables the actions button of a row while it being edited.
   * @param {GridRowId} id The id of the row to set the loading property of.
   * @param {boolean} loading The value to set the loading property to.
   */
  const setRowLoading = (id: GridRowId, loading: boolean) => {
    setRows((prevRows) => {
      const index = prevRows.findIndex((row) => row.id === id)
      const newRows = [...prevRows]
      newRows.splice(index, 1, { ...prevRows[index], loading })
      return newRows
    })
  }

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
          setRowLoading(id, true)
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
          setRowLoading(id, true)
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
   * @param {GridRowId} id The id of the row matching the token to update.
   * @param {TokenUpdatePayload} updateParams Updated properties of the token.
   * @param {TokenUpdatePayload} updateParams.name The name of the token.
   * @param {TokenUpdatePayload} updateParams.enabled If the token is enabled.
   * @param {TokenUpdatePayload} updateParams.expires Tokens expiration date.
   */
  const handleUpdateToken = (
    id: GridRowId,
    { name, enabled, expires }: TokenUpdatePayload
  ) => {
    const abortController = new AbortController()
    // mark the row as loading to disable the actions menu
    setRowLoading(id, true)
    // define async function to update the token
    const fetchUpdate = async () => {
      try {
        // make the request to the API to update the token
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
        // verify that the request response was OK
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
          newRows.splice(index, 1, {
            ...prevRows[index],
            ...newToken,
            loading: false,
          })
          return newRows
        })
        // show a success alert
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
        setRowLoading(id, false)
      }
    }
    // call the async function to make the request
    fetchUpdate()
    // return cleanup function to cancel the request
    return () => abortController.abort()
  }

  /**
   * Callback that makes a request API to disable an enabled token.
   * @param {GridRowId} id The id of the row matching the token to update.
   */
  /* eslint-disable react-hooks/exhaustive-deps */
  const handleDisableToken = React.useCallback(
    (id: GridRowId) => () => handleUpdateToken(id, { enabled: false }),
    []
  )
  /* eslint-enable react-hooks/exhaustive-deps */

  /**
   * Callback that makes a request API to enable a disabled token.
   * @param {GridRowId} id The id of the row matching the token to update.
   */
  /* eslint-disable react-hooks/exhaustive-deps */
  const handleEnableToken = React.useCallback(
    (id: GridRowId) => () => handleUpdateToken(id, { enabled: true }),
    []
  )
  /* eslint-enable react-hooks/exhaustive-deps */

  /**
   * Callback that updates the rows in the table after a token is created.
   * @param {GridRowId} id The id of the row matching the token to update.
   */
  /* eslint-disable react-hooks/exhaustive-deps */
  const handleTokenAdded = React.useCallback((token: Token) => {
    setRows((prevRows) => [...prevRows, { ...token, loading: false }])
  }, [])
  /* eslint-enable react-hooks/exhaustive-deps */

  /**
   * Callback that displays the pop-up dialog to create a new token.
   * @param {GridRowId} id The id of the row matching the token to update.
   */
  /* eslint-disable react-hooks/exhaustive-deps */
  const openTokenDialog = React.useCallback(() => {
    openDialog({
      children: (
        <TokenCreateDialog teamId={teamId} onTokenAdded={handleTokenAdded} />
      ),
    })
  }, [])
  /* eslint-enable react-hooks/exhaustive-deps */

  /**
   * The column definitions for the TokensTable DataGrid.
   */
  const columns = React.useMemo<GridColumns<TokenRow>>(
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
        cellClassName: 'MuiDataGrid-cell--full-width ',
        renderCell: (params): JSX.Element => {
          if (params.row.loading) {
            params.focusElementRef = null
            return (
              <IconButtonWithExcludedProps disabled>
                <MoreVertIcon />
              </IconButtonWithExcludedProps>
            )
          }
          return <GridActionsCell {...params} sx={{ m: 0 }} />
        },
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
    <Box>
      <Card>
        <DataGrid
          columns={columns}
          rows={rows}
          autoHeight
          disableSelectionOnClick
          getRowClassName={({ row: { loading = false } = {} }) =>
            `row-loading-${loading}`
          }
          hideFooter
          sortModel={[
            {
              field: 'created',
              sort: 'desc' as const,
            },
          ]}
          sx={{
            '& .MuiDataGrid-row': {
              transition: 'all 0.5s ease',

              '&.row-loading-true': {
                filter: 'opacity(0.5)',
                backgroundColor: 'rgba(0,0,0, .25)',
              },

              '& .MuiDataGrid-cell--full-width': {
                width: '100% !important',

                '& > div': {
                  alignItems: 'center',
                  display: 'inline-flex',
                  position: 'relative',
                  textAlign: 'center',
                  minHeight: '100%',
                  minWidth: '100%',
                  width: '100% !important',

                  '& > button': {
                    borderRadius: 0,
                    display: 'block',
                    position: 'absolute',
                    width: '100%',
                    height: '100%',
                    padding: '0',
                    top: '0',
                    bottom: '0',
                  },
                },
              },
            },
          }}
        />
      </Card>
      <Box
        sx={{
          display: 'flex',
          justifyContent: 'flex-end',
          mb: -1.5,
          mt: -1.5,
          ml: 3,
          width: '100%',
        }}
      >
        <Fab
          color="primary"
          aria-label="add"
          onClick={openTokenDialog}
          size="medium"
        >
          <AddIcon />
        </Fab>
      </Box>
    </Box>
  )
}

TokensTable.displayName = 'TokensTable'

export default TokensTable
