/**
 * A component that renders a table of team members with their details.
 * @module @cyclonedx/ui/sbom/views/Dashboard/Team/TeamMembersTable
 */
// ** React Imports
import * as React from 'react'

// ** MUI Imports
import { styled } from '@mui/material/styles'
import Box from '@mui/material/Box'
import Card from '@mui/material/Card'
import Typography from '@mui/material/Typography'
import { DataGrid, GridColDef } from '@mui/x-data-grid'

// ** Icon Imports
import Cog from 'mdi-material-ui/Cog'
import AccountOutline from 'mdi-material-ui/AccountOutline'

// ** App Imports
import { ThemeColor } from '@/layouts/types'
import { getInitials } from '@/utils/get-initials'
import Chip from '@/components/mui/chip'
import Avatar from '@/components/mui/avatar'

export interface TableBodyRowType {
  id?: number
  name?: string
  email: string
  username?: string
  avatarSrc?: string
  status?: 'active' | 'pending' | 'inactive' | 'unknown'
  isTeamLead: boolean
  role?: 'admin' | 'member'
}

export interface CellType {
  row: TableBodyRowType
}

export interface RoleObj {
  [key: string]: {
    icon: React.ReactElement
  }
}

interface StatusObj {
  [key: string]: {
    color: ThemeColor
  }
}

const roleObj: RoleObj = {
  admin: {
    icon: <Cog sx={{ mr: 2, color: 'error.main' }} />,
  },
  member: {
    icon: <AccountOutline sx={{ mr: 2, color: 'primary.main' }} />,
  },
}

const statusObj: StatusObj = {
  active: { color: 'success' },
  pending: { color: 'warning' },
  inactive: { color: 'secondary' },
  unknown: { color: 'error' },
}

const StyledAvatar = styled(Avatar, {
  // configure which props should be forwarded on DOM
  shouldForwardProp: (prop) => prop !== 'sx' && prop !== 'skin',
  name: 'Avatar',
  slot: 'Root',
  // specify how the `styleOverrides` should be applied based on props
  overridesResolver: (props, styles) => [
    styles.root,
    props.skin === 'light' && { sx: { fontSize: '.8rem' } },
  ],
})({
  // default sx styles for the styled component
  mr: 3,
  width: 34,
  height: 34,
})

/**
 * Component that renders a user avatar with the user's avatar image if
 *  it exists, otherwise it renders the user's initials from their name.
 * @param {TableBodyRowType} props - component props representing a user row.
 * @param {string} [props.avatarSrc] - optional source URL for the user's avatar image, if it exists.
 * @param {string} props.name - The user's full name.
 * @returns {JSX.Element} A component that renders a table row.
 */
const UserAvatar = ({ avatarSrc, name }: TableBodyRowType) => {
  if (avatarSrc) {
    return <StyledAvatar src={avatarSrc} />
  }

  return (
    <StyledAvatar skin="light" sx={{ fontSize: '.8rem' }}>
      {getInitials(name ? name : ' ')}
    </StyledAvatar>
  )
}

const columns: GridColDef[] = [
  {
    flex: 0.25,
    field: 'name',
    minWidth: 200,
    headerName: 'User',
    renderCell: ({ row }: CellType) => {
      return (
        <Box sx={{ display: 'flex', alignItems: 'center' }}>
          <UserAvatar {...row} />
          <Box sx={{ display: 'flex', flexDirection: 'column' }}>
            <Typography variant="subtitle2" sx={{ color: 'text.primary' }}>
              {row.name}
            </Typography>
            <Typography variant="caption" sx={{ lineHeight: 1.6667 }}>
              {row.username}
            </Typography>
          </Box>
        </Box>
      )
    },
  },
  {
    flex: 0.3,
    minWidth: 250,
    field: 'email',
    headerName: 'Email',
    renderCell: ({ row: { email } }: CellType) => (
      <Typography variant="body2">{email}</Typography>
    ),
  },
  {
    flex: 0.2,
    minWidth: 130,
    field: 'role',
    headerName: 'Role',
    renderCell: ({ row: { role = 'member' } }: CellType) => (
      <Box sx={{ display: 'flex', alignItems: 'center' }}>
        {roleObj[role].icon}
        <Typography
          sx={{ color: 'text.secondary', textTransform: 'capitalize' }}
        >
          {role}
        </Typography>
      </Box>
    ),
  },
  {
    flex: 0.15,
    minWidth: 110,
    field: 'status',
    headerName: 'Status',
    renderCell: ({ row: { status = 'unknown' } }: CellType) => (
      <Chip
        skin="light"
        size="small"
        label={status}
        color={statusObj[status || 'member'].color}
        sx={{
          textTransform: 'capitalize',
          '& .MuiChip-label': { px: 2.5, lineHeight: 1.385 },
        }}
      />
    ),
  },
]

type InputProps = {
  members: TableBodyRowType[]
}

/**
 * A component that renders a table of team members with their details.
 * @param {InputProps} props
 * @param {TableBodyRowType[]} props.members - The list of team members.
 * @returns {JSX.Element} A component that renders a datagrid table of team members.
 */
const TeamMemberTable = ({ members }: InputProps) => {
  return (
    <Card>
      <DataGrid
        autoHeight
        hideFooter
        rows={members.map((member) => ({ ...member, id: member.email }))}
        columns={columns}
        disableSelectionOnClick
        pagination={undefined}
      />
    </Card>
  )
}

TeamMemberTable.displayName = 'TeamMemberTable'

export default TeamMemberTable
