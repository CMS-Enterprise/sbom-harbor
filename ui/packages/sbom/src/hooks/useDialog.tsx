import * as React from 'react'
import Dialog, { DialogProps } from '@mui/material/Dialog'

type DialogState = {
  children: JSX.Element
  props?: DialogProps | null
}

const DialogContext = React.createContext([
  (dialog: DialogState) => {
    return
  },
])

const DialogProvider = ({ children }: { children: React.ReactNode }) => {
  const [
    { children: dialogChildren, props: dialogProps, ...params },
    setDialog,
  ] = React.useState<DialogState>({ children: <></>, props: null })

  const [open, setOpen] = React.useState(false)

  const openDialog = React.useCallback((dialog: DialogState) => {
    setDialog(dialog)
    setOpen(true)
  }, [])

  const closeDialog = () => {
    setOpen(false)
  }

  const contextValue = React.useRef([openDialog, closeDialog])

  return (
    <DialogContext.Provider value={contextValue.current}>
      {children}
      <Dialog {...dialogProps} onClose={closeDialog} open={open}>
        {React.Children.map(dialogChildren, (child) => {
          // Checking isValidElement is the safe way and avoids a
          // typescript error too.
          const combinedProps = {
            ...params,
            setOpen,
          }
          if (React.isValidElement(child)) {
            return React.cloneElement(child, combinedProps)
          }
          return child
        })}
      </Dialog>
    </DialogContext.Provider>
  )
}

export const useDialog = () => {
  const result = React.useContext(DialogContext)
  if (!result) {
    throw new Error('Dialog context is only available inside its provider')
  }
  return result
}

export default DialogProvider
