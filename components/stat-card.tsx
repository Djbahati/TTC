import type { LucideIcon } from "lucide-react"
import { Card, CardContent } from "@/components/ui/card"

interface StatCardProps {
  label: string
  value: string | number
  icon: LucideIcon
  valueClassName?: string
  iconClassName?: string
}

export function StatCard({
  label,
  value,
  icon: Icon,
  valueClassName = "text-white",
  iconClassName = "text-white",
}: StatCardProps) {
  return (
    <Card className="bg-neutral-900 border-neutral-700">
      <CardContent className="p-4">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-xs text-neutral-400 tracking-wider">{label}</p>
            <p className={`text-2xl font-bold font-mono ${valueClassName}`}>{value}</p>
          </div>
          <Icon className={`w-8 h-8 ${iconClassName}`} />
        </div>
      </CardContent>
    </Card>
  )
}
