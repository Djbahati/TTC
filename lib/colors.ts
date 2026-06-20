export type StatusType =
  | "active"
  | "planning"
  | "completed"
  | "compromised"
  | "online"
  | "warning"
  | "maintenance"
  | "offline"
  | "verified"
  | "pending"
  | "standby"
  | "training"

export type SeverityLevel = "critical" | "high" | "medium" | "low"

export type ClassificationType = "TOP SECRET" | "SECRET" | "CONFIDENTIAL"

export function getStatusBadgeColor(status: string): string {
  switch (status) {
    case "active":
    case "completed":
    case "online":
    case "verified":
      return "bg-white/20 text-white"
    case "planning":
    case "pending":
    case "warning":
      return "bg-orange-500/20 text-orange-500"
    case "compromised":
    case "offline":
      return "bg-red-500/20 text-red-500"
    case "maintenance":
    case "standby":
    default:
      return "bg-neutral-500/20 text-neutral-300"
  }
}

export function getSeverityColor(level: string): string {
  switch (level) {
    case "critical":
      return "bg-red-500/20 text-red-500"
    case "high":
      return "bg-orange-500/20 text-orange-500"
    case "medium":
      return "bg-neutral-500/20 text-neutral-300"
    case "low":
      return "bg-white/20 text-white"
    default:
      return "bg-neutral-500/20 text-neutral-300"
  }
}

export function getClassificationColor(classification: string): string {
  switch (classification) {
    case "TOP SECRET":
      return "bg-red-500/20 text-red-500"
    case "SECRET":
      return "bg-orange-500/20 text-orange-500"
    case "CONFIDENTIAL":
      return "bg-neutral-500/20 text-neutral-300"
    default:
      return "bg-white/20 text-white"
  }
}

export function getStatusDotColor(status: string): string {
  switch (status) {
    case "active":
      return "bg-white"
    case "standby":
      return "bg-neutral-500"
    case "training":
      return "bg-orange-500"
    case "compromised":
      return "bg-red-500"
    default:
      return "bg-neutral-500"
  }
}

export function getHealthColor(health: number): string {
  if (health >= 70) return "text-white"
  if (health >= 50) return "text-orange-500"
  return "text-red-500"
}
