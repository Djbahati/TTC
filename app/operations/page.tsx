"use client"

import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Target, MapPin, Clock, Users, AlertTriangle, CheckCircle, XCircle } from "lucide-react"
import { getStatusBadgeColor, getSeverityColor } from "@/lib/colors"
import { StatCard } from "@/components/stat-card"
import { PageHeader } from "@/components/page-header"
import { DetailModal, ModalPrimaryButton, ModalOutlineButton } from "@/components/detail-modal"
import { ProgressBar } from "@/components/progress-bar"
import { Button } from "@/components/ui/button"

export default function OperationsPage() {
  type Operation = {
    id: string
    name: string
    status: "active" | "planning" | "completed" | "compromised" | string
    priority: "critical" | "high" | "medium" | "low" | string
    location: string
    agents: number
    progress: number
    startDate: string
    estimatedCompletion: string
    description: string
    objectives: string[]
  }

  const [selectedOperation, setSelectedOperation] = useState<Operation | null>(null)

  const operations: Operation[] = [
    {
      id: "OP-OMEGA-001",
      name: "SHADOW PROTOCOL",
      status: "active",
      priority: "critical",
      location: "Eastern Europe",
      agents: 5,
      progress: 75,
      startDate: "2025-06-15",
      estimatedCompletion: "2025-06-30",
      description: "Track high-value target in Eastern Europe",
      objectives: ["Locate target", "Establish surveillance", "Extract intelligence"],
    },
    {
      id: "OP-DELTA-002",
      name: "GHOST FIRE",
      status: "planning",
      priority: "high",
      location: "Seoul",
      agents: 3,
      progress: 25,
      startDate: "2025-06-20",
      estimatedCompletion: "2025-07-05",
      description: "Infiltrate cybercrime network in Seoul",
      objectives: ["Penetrate network", "Gather evidence", "Identify key players"],
    },
    {
      id: "OP-SIERRA-003",
      name: "NIGHT STALKER",
      status: "completed",
      priority: "medium",
      location: "Berlin",
      agents: 2,
      progress: 100,
      startDate: "2025-05-28",
      estimatedCompletion: "2025-06-12",
      description: "Monitor rogue agent communications in Berlin",
      objectives: ["Intercept communications", "Decode messages", "Report findings"],
    },
    {
      id: "OP-ALPHA-004",
      name: "CRIMSON TIDE",
      status: "active",
      priority: "high",
      location: "Cairo",
      agents: 4,
      progress: 60,
      startDate: "2025-06-10",
      estimatedCompletion: "2025-06-25",
      description: "Support covert extraction in South America",
      objectives: ["Secure extraction point", "Neutralize threats", "Extract asset"],
    },
    {
      id: "OP-BRAVO-005",
      name: "SILENT BLADE",
      status: "compromised",
      priority: "critical",
      location: "Moscow",
      agents: 6,
      progress: 40,
      startDate: "2025-06-05",
      estimatedCompletion: "2025-06-20",
      description: "Monitor rogue agent communications in Berlin",
      objectives: ["Assess compromise", "Extract personnel", "Damage control"],
    },
  ]

  const getStatusIcon = (status: Operation["status"]) => {
    switch (status) {
      case "active":
        return <Target className="w-4 h-4" />
      case "planning":
        return <Clock className="w-4 h-4" />
      case "completed":
        return <CheckCircle className="w-4 h-4" />
      case "compromised":
        return <XCircle className="w-4 h-4" />
      default:
        return <AlertTriangle className="w-4 h-4" />
    }
  }

  return (
    <div className="p-6 space-y-6">
      <PageHeader
        title="OPERATIONS CENTER"
        subtitle="Mission planning and execution oversight"
        actions={
          <>
            <Button className="bg-orange-500 hover:bg-orange-600 text-white">New Operation</Button>
            <Button className="bg-orange-500 hover:bg-orange-600 text-white">Mission Brief</Button>
          </>
        }
      />

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard label="ACTIVE OPS" value={23} icon={Target} />
        <StatCard label="COMPLETED" value={156} icon={CheckCircle} />
        <StatCard label="COMPROMISED" value={2} icon={XCircle} valueClassName="text-red-500" iconClassName="text-red-500" />
        <StatCard label="SUCCESS RATE" value="94%" icon={AlertTriangle} />
      </div>

      {/* Operations List */}
      <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
        {operations.map((operation) => (
          <Card
            key={operation.id}
            className="bg-neutral-900 border-neutral-700 hover:border-orange-500/50 transition-colors cursor-pointer"
            onClick={() => setSelectedOperation(operation)}
          >
            <CardHeader className="pb-3">
              <div className="flex items-start justify-between">
                <div>
                  <CardTitle className="text-sm font-bold text-white tracking-wider">{operation.name}</CardTitle>
                  <p className="text-xs text-neutral-400 font-mono">{operation.id}</p>
                </div>
                <div className="flex items-center gap-2">{getStatusIcon(operation.status)}</div>
              </div>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex gap-2">
                <Badge className={getStatusBadgeColor(operation.status)}>{operation.status.toUpperCase()}</Badge>
                <Badge className={getSeverityColor(operation.priority)}>{operation.priority.toUpperCase()}</Badge>
              </div>

              <p className="text-sm text-neutral-300">{operation.description}</p>

              <div className="space-y-2">
                <div className="flex items-center gap-2 text-xs text-neutral-400">
                  <MapPin className="w-3 h-3" />
                  <span>{operation.location}</span>
                </div>
                <div className="flex items-center gap-2 text-xs text-neutral-400">
                  <Users className="w-3 h-3" />
                  <span>{operation.agents} agents assigned</span>
                </div>
                <div className="flex items-center gap-2 text-xs text-neutral-400">
                  <Clock className="w-3 h-3" />
                  <span>Est. completion: {operation.estimatedCompletion}</span>
                </div>
              </div>

              <ProgressBar value={operation.progress} showLabel label="Progress" />
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Operation Detail Modal */}
      {selectedOperation && (
        <DetailModal
          title={selectedOperation.name}
          subtitle={selectedOperation.id}
          onClose={() => setSelectedOperation(null)}
          actions={
            <>
              <ModalPrimaryButton>Update Status</ModalPrimaryButton>
              <ModalOutlineButton>View Reports</ModalOutlineButton>
              <ModalOutlineButton>Assign Agents</ModalOutlineButton>
            </>
          }
        >
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="space-y-4">
              <div>
                <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">OPERATION STATUS</h3>
                <div className="flex gap-2">
                  <Badge className={getStatusBadgeColor(selectedOperation.status)}>
                    {selectedOperation.status.toUpperCase()}
                  </Badge>
                  <Badge className={getSeverityColor(selectedOperation.priority)}>
                    {selectedOperation.priority.toUpperCase()}
                  </Badge>
                </div>
              </div>

              <div>
                <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">MISSION DETAILS</h3>
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-neutral-400">Location:</span>
                    <span className="text-white">{selectedOperation.location}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-neutral-400">Agents:</span>
                    <span className="text-white font-mono">{selectedOperation.agents}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-neutral-400">Start Date:</span>
                    <span className="text-white font-mono">{selectedOperation.startDate}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-neutral-400">Est. Completion:</span>
                    <span className="text-white font-mono">{selectedOperation.estimatedCompletion}</span>
                  </div>
                </div>
              </div>
            </div>

            <div className="space-y-4">
              <div>
                <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">PROGRESS</h3>
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span className="text-neutral-400">Completion</span>
                    <span className="text-white font-mono">{selectedOperation.progress}%</span>
                  </div>
                  <ProgressBar value={selectedOperation.progress} height="h-3" />
                </div>
              </div>

              <div>
                <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">OBJECTIVES</h3>
                <div className="space-y-2">
                  {selectedOperation.objectives.map((objective, index) => (
                    <div key={index} className="flex items-center gap-2 text-sm">
                      <div className="w-2 h-2 bg-orange-500 rounded-full"></div>
                      <span className="text-neutral-300">{objective}</span>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>

          <div>
            <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">DESCRIPTION</h3>
            <p className="text-sm text-neutral-300">{selectedOperation.description}</p>
          </div>
        </DetailModal>
      )}
    </div>
  )
}
