/**
 * A component that renders a table of team members with their details.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamMembersTable
 */
import * as React from 'react'
import Card from '@mui/material/Card'
import Typography from '@mui/material/Typography'
import { DataGrid, GridColDef } from '@mui/x-data-grid'
import { Token, TokenRowType } from '@/types'

type RenderCellProps = {
  row: TokenRowType
}

const DateRow = ({ date }: { date: Date }) => (
  <Typography variant="caption">{date.toLocaleDateString('en-US')}</Typography>
)

/**
 * The configuration object for the columns of the tokens table,
 *  where each row represents a user that is a token of the team.
 * @constant {GridColDef[]} columns The columns for the tokens table
 */
const columns: GridColDef[] = [
  {
    flex: 0.2,
    field: 'name',
    headerName: 'Description',
    renderCell: ({ row: { name, id } }: RenderCellProps) => {
      return <Typography variant="body2">{name || id}</Typography>
    },
  },
  {
    flex: 0.125,
    field: 'created',
    headerName: 'Created',
    renderCell: ({ row: { created } }: RenderCellProps) => (
      <DateRow date={new Date(created)} />
    ),
  },
  {
    flex: 0.125,
    field: 'expires',
    headerName: 'Expires',
    renderCell: ({ row: { expires } }: RenderCellProps) => (
      <DateRow date={new Date(expires)} />
    ),
  },
  {
    flex: 0.125,
    field: 'expired',
    headerName: 'Expired?',
    renderCell: ({ row: { expires } }: RenderCellProps) => {
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
    renderCell: ({ row: { enabled } }: RenderCellProps) => {
      return (
        <Typography
          variant="caption"
          textAlign="center"
          sx={{ color: !enabled ? 'red' : 'green', width: '100%' }}
        >
          {enabled ? 'Enabled' : 'Disabled'}
        </Typography>
      )
    },
  },
  {
    flex: 0.5,
    minWidth: 130,
    field: 'token',
    headerName: 'Secret',
    renderCell: ({ row: { token = '********' } }: RenderCellProps) => (
      <Typography variant="caption">{token}</Typography>
    ),
  },
]

type InputProps = {
  tokens: Token[]
}

/**
 * A component that renders a table of team members with their details.
 * @param {InputProps} props Input props for the TeamMembersTable component.
 * @param {UserTableRowType[]} props.members - The list of team members.
 * @returns {JSX.Element} A component that renders a datagrid table of team members.
 */
const TokensTable = ({ tokens }: InputProps) => (
  <Card>
    <DataGrid
      autoHeight
      hideFooter
      rows={tokens}
      columns={columns}
      disableSelectionOnClick
      pagination={undefined}
    />
  </Card>
)

TokensTable.displayName = 'TokensTable'

export default TokensTable
