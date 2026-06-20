"use client"

import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Progress } from "@/components/ui/progress"
import {
  Server,
  Database,
  Shield,
  Wifi,
  HardDrive,
  Cpu,
  Activity,
  AlertTriangle,
  CheckCircle,
  Settings,
} from "lucide-react"
import { getStatusBadgeColor, getHealthColor } from "@/lib/colors"
import { StatCard } from "@/components/stat-card"
import { PageHeader } from "@/components/page-header"
import { DetailModal, ModalPrimaryButton, ModalOutlineButton } from "@/components/detail-modal"
import { ProgressBar } from "@/components/progress-bar"
import { Button } from "@/components/ui/button"

export default function SystemsPage() {
  type System = {
    id: string
    name: string
    type: string
    status: "online" | "warning" | "maintenance" | "offline" | string
    health: number
    cpu: number
    memory: number
    storage: number
    uptime: string
    location: string
    lastMaintenance: string
  }

  const [selectedSystem, setSelectedSystem] = useState<System | null>(null)

  const systems: System[] = [
    {
      id: "SYS-001",
      name: "COMMAND SERVER ALPHA",
      type: "Primary Server",
      status: "online",
      health: 98,
      cpu: 45,
      memory: 67,
      storage: 34,
      uptime: "247 days",
      location: "Data Center 1",
      lastMaintenance: "2025-05-15",
    },
    {
      id: "SYS-002",
      name: "DATABASE CLUSTER BETA",
      type: "Database",
      status: "online",
      health: 95,
      cpu: 72,
      memory: 84,
      storage: 78,
      uptime: "189 days",
      location: "Data Center 2",
      lastMaintenance: "2025-06-01",
    },
    {
      id: "SYS-003",
      name: "SECURITY GATEWAY",
      type: "Firewall",
      status: "warning",
      health: 87,
      cpu: 23,
      memory: 45,
      storage: 12,
      uptime: "156 days",
      location: "DMZ",
      lastMaintenance: "2025-04-20",
    },
    {
      id: "SYS-004",
      name: "COMMUNICATION HUB",
      type: "Network",
      status: "online",
      health: 92,
      cpu: 38,
      memory: 52,
      storage: 23,
      uptime: "203 days",
      location: "Network Core",
      lastMaintenance: "2025-05-28",
    },
    {
      id: "SYS-005",
      name: "BACKUP STORAGE ARRAY",
      type: "Storage",
      status: "maintenance",
      health: 76,
      cpu: 15,
      memory: 28,
      storage: 89,
      uptime: "0 days",
      location: "Backup Facility",
      lastMaintenance: "2025-06-17",
    },
    {
      id: "SYS-006",
      name: "ANALYTICS ENGINE",
      type: "Processing",
      status: "online",
      health: 94,
      cpu: 89,
      memory: 76,
      storage: 45,
      uptime: "134 days",
      location: "Data Center 1",
      lastMaintenance: "2025-05-10",
    },
  ]

  const getStatusIcon = (status: System["status"]) => {
    switch (status) {
      case "online":
        return <CheckCircle className="w-4 h-4" />
      case "warning":
        return <AlertTriangle className="w-4 h-4" />
      case "maintenance":
        return <Settings className="w-4 h-4" />
      case "offline":
        return <AlertTriangle className="w-4 h-4" />
      default:
        return <Activity className="w-4 h-4" />
    }
  }

  const getSystemIcon = (type: System["type"]) => {
    switch (type) {
      case "Primary Server":
        return <Server className="w-6 h-6" />
      case "Database":
        return <Database className="w-6 h-6" />
      case "Firewall":
        return <Shield className="w-6 h-6" />
      case "Network":
        return <Wifi className="w-6 h-6" />
      case "Storage":
        return <HardDrive className="w-6 h-6" />
      case "Processing":
        return <Cpu className="w-6 h-6" />
      default:
        return <Server className="w-6 h-6" />
    }
  }

  return (
    <div className="p-6 space-y-6">
      <PageHeader
        title="SYSTEMS MONITOR"
        subtitle="Infrastructure health and performance monitoring"
        actions={
          <>
            <Button className="bg-orange-500 hover:bg-orange-600 text-white">System Scan</Button>
            <Button className="bg-orange-500 hover:bg-orange-600 text-white">Maintenance Mode</Button>
          </>
        }
      />

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard label="SYSTEMS ONLINE" value="24/26" icon={CheckCircle} />
        <StatCard label="WARNINGS" value={3} icon={AlertTriangle} valueClassName="text-orange-500" iconClassName="text-orange-500" />
        <StatCard label="AVG UPTIME" value="99.7%" icon={Activity} />
        <StatCard label="MAINTENANCE" value={1} icon={Settings} valueClassName="text-neutral-300" iconClassName="text-neutral-300" />
      </div>

      {/* Systems Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
        {systems.map((system) => (
          <Card
            key={system.id}
            className="bg-neutral-900 border-neutral-700 hover:border-orange-500/50 transition-colors cursor-pointer"
            onClick={() => setSelectedSystem(system)}
          >
            <CardHeader className="pb-3">
              <div className="flex items-start justify-between">
                <div className="flex items-center gap-3">
                  {getSystemIcon(system.type)}
                  <div>
                    <CardTitle className="text-sm font-bold text-white tracking-wider">{system.name}</CardTitle>
                    <p className="text-xs text-neutral-400">{system.type}</p>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  {getStatusIcon(system.status)}
                  <Badge className={getStatusBadgeColor(system.status)}>{system.status.toUpperCase()}</Badge>
                </div>
              </div>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-xs text-neutral-400">SYSTEM HEALTH</span>
                <span className={`text-sm font-bold font-mono ${getHealthColor(system.health)}`}>{system.health}%</span>
              </div>
              <Progress value={system.health} className="h-2" />

              <div className="grid grid-cols-3 gap-4 text-xs">
                {([["CPU", system.cpu], ["MEMORY", system.memory], ["STORAGE", system.storage]] as const).map(
                  ([label, value]) => (
                    <div key={label}>
                      <div className="text-neutral-400 mb-1">{label}</div>
                      <div className="text-white font-mono">{value}%</div>
                      <ProgressBar value={value} height="h-1" />
                    </div>
                  ),
                )}
              </div>

              <div className="space-y-1 text-xs text-neutral-400">
                <div className="flex justify-between">
                  <span>Uptime:</span>
                  <span className="text-white font-mono">{system.uptime}</span>
                </div>
                <div className="flex justify-between">
                  <span>Location:</span>
                  <span className="text-white">{system.location}</span>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {/* System Detail Modal */}
      {selectedSystem && (
        <DetailModal
          title={selectedSystem.name}
          subtitle={`${selectedSystem.id} • ${selectedSystem.type}`}
          titlePrefix={getSystemIcon(selectedSystem.type)}
          onClose={() => setSelectedSystem(null)}
          actions={
            <>
              <ModalPrimaryButton>Restart System</ModalPrimaryButton>
              <ModalOutlineButton>View Logs</ModalOutlineButton>
              <ModalOutlineButton>Schedule Maintenance</ModalOutlineButton>
            </>
          }
        >
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="space-y-4">
              <div>
                <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">SYSTEM STATUS</h3>
                <div className="flex items-center gap-2">
                  {getStatusIcon(selectedSystem.status)}
                  <Badge className={getStatusBadgeColor(selectedSystem.status)}>
                    {selectedSystem.status.toUpperCase()}
                  </Badge>
                </div>
              </div>

              <div>
                <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">SYSTEM INFORMATION</h3>
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-neutral-400">Location:</span>
                    <span className="text-white">{selectedSystem.location}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-neutral-400">Uptime:</span>
                    <span className="text-white font-mono">{selectedSystem.uptime}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-neutral-400">Last Maintenance:</span>
                    <span className="text-white font-mono">{selectedSystem.lastMaintenance}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-neutral-400">Health Score:</span>
                    <span className={`font-mono ${getHealthColor(selectedSystem.health)}`}>
                      {selectedSystem.health}%
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <div className="space-y-4">
              <div>
                <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">RESOURCE USAGE</h3>
                <div className="space-y-3">
                  {([["CPU Usage", selectedSystem.cpu], ["Memory Usage", selectedSystem.memory], ["Storage Usage", selectedSystem.storage]] as const).map(
                    ([label, value]) => (
                      <div key={label}>
                        <div className="flex justify-between text-sm mb-1">
                          <span className="text-neutral-400">{label}</span>
                          <span className="text-white font-mono">{value}%</span>
                        </div>
                        <ProgressBar value={value} />
                      </div>
                    ),
                  )}
                </div>
              </div>
            </div>
          </div>
        </DetailModal>
      )}
    </div>
  )
}
