import type React from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"

interface DetailModalProps {
  title: string
  subtitle?: string
  titlePrefix?: React.ReactNode
  onClose: () => void
  actions?: React.ReactNode
  maxWidth?: string
  children: React.ReactNode
}

export function DetailModal({
  title,
  subtitle,
  titlePrefix,
  onClose,
  actions,
  maxWidth = "max-w-4xl",
  children,
}: DetailModalProps) {
  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
      <Card className={`bg-neutral-900 border-neutral-700 w-full ${maxWidth} max-h-[90vh] overflow-y-auto`}>
        <CardHeader className="flex flex-row items-center justify-between">
          <div className="flex items-center gap-3">
            {titlePrefix}
            <div>
              <CardTitle className="text-xl font-bold text-white tracking-wider">{title}</CardTitle>
              {subtitle && <p className="text-sm text-neutral-400 font-mono">{subtitle}</p>}
            </div>
          </div>
          <Button
            variant="ghost"
            onClick={onClose}
            className="text-neutral-400 hover:text-white"
          >
            ✕
          </Button>
        </CardHeader>
        <CardContent className="space-y-6">
          {children}
          {actions && (
            <div className="flex gap-2 pt-4 border-t border-neutral-700">
              {actions}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}

export function ModalOutlineButton({ children }: { children: React.ReactNode }) {
  return (
    <Button
      variant="outline"
      className="border-neutral-700 text-neutral-400 hover:bg-neutral-800 hover:text-neutral-300 bg-transparent"
    >
      {children}
    </Button>
  )
}

export function ModalPrimaryButton({ children }: { children: React.ReactNode }) {
  return (
    <Button className="bg-orange-500 hover:bg-orange-600 text-white">
      {children}
    </Button>
  )
}
