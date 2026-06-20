interface ProgressBarProps {
  value: number
  height?: string
  barColor?: string
  trackColor?: string
  showLabel?: boolean
  label?: string
}

export function ProgressBar({
  value,
  height = "h-2",
  barColor = "bg-orange-500",
  trackColor = "bg-neutral-800",
  showLabel = false,
  label,
}: ProgressBarProps) {
  const clampedValue = Math.max(0, Math.min(100, value))

  return (
    <div className="space-y-2">
      {showLabel && (
        <div className="flex justify-between text-xs">
          <span className="text-neutral-400">{label ?? "Progress"}</span>
          <span className="text-white font-mono">{clampedValue}%</span>
        </div>
      )}
      <div className={`w-full ${trackColor} rounded-full ${height}`}>
        <div
          className={`${barColor} ${height} rounded-full transition-all duration-300`}
          style={{ width: `${clampedValue}%` }}
        />
      </div>
    </div>
  )
}
