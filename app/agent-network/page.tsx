"use client"

import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Filter, MoreHorizontal, MapPin, Clock, Shield } from "lucide-react"
import { getStatusDotColor, getSeverityColor } from "@/lib/colors"
import { StatCard } from "@/components/stat-card"
import { PageHeader } from "@/components/page-header"
import { SearchInput } from "@/components/search-input"
import { DetailModal, ModalPrimaryButton, ModalOutlineButton } from "@/components/detail-modal"

export default function AgentNetworkPage() {
  type Agent = {
    id: string
    name: string
    status: "active" | "standby" | "training" | "compromised" | string
    location: string
    lastSeen: string
    missions: number
    risk: "critical" | "high" | "medium" | "low" | string
  }

  const [searchTerm, setSearchTerm] = useState("")
  const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null)

  const agents: Agent[] = [
    {
      id: "G-078W",
      name: "VENGEFUL SPIRIT",
      status: "active",
      location: "Berlin",
      lastSeen: "2 min ago",
      missions: 47,
      risk: "high",
    },
    {
      id: "G-079X",
      name: "OBSIDIAN SENTINEL",
      status: "standby",
      location: "Tokyo",
      lastSeen: "15 min ago",
      missions: 32,
      risk: "medium",
    },
    {
      id: "G-080Y",
      name: "GHOSTLY FURY",
      status: "active",
      location: "Cairo",
      lastSeen: "1 min ago",
      missions: 63,
      risk: "high",
    },
    {
      id: "G-081Z",
      name: "CURSED REVENANT",
      status: "compromised",
      location: "Moscow",
      lastSeen: "3 hours ago",
      missions: 28,
      risk: "critical",
    },
    {
      id: "G-082A",
      name: "VENOMOUS SHADE",
      status: "active",
      location: "London",
      lastSeen: "5 min ago",
      missions: 41,
      risk: "medium",
    },
    {
      id: "G-083B",
      name: "MYSTIC ENIGMA",
      status: "training",
      location: "Base Alpha",
      lastSeen: "1 day ago",
      missions: 12,
      risk: "low",
    },
    {
      id: "G-084C",
      name: "WRAITH AVENGER",
      status: "active",
      location: "Paris",
      lastSeen: "8 min ago",
      missions: 55,
      risk: "high",
    },
    {
      id: "G-085D",
      name: "SPECTRAL FURY",
      status: "standby",
      location: "Sydney",
      lastSeen: "22 min ago",
      missions: 38,
      risk: "medium",
    },
  ]

  const filteredAgents = agents.filter(
    (agent) =>
      agent.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      agent.id.toLowerCase().includes(searchTerm.toLowerCase()),
  )

  return (
    <div className="p-6 space-y-6">
      <PageHeader
        title="AGENT NETWORK"
        subtitle="Manage and monitor field operatives"
        actions={
          <>
            <Button className="bg-orange-500 hover:bg-orange-600 text-white">Deploy Agent</Button>
            <Button className="bg-orange-500 hover:bg-orange-600 text-white">
              <Filter className="w-4 h-4 mr-2" />
              Filter
            </Button>
          </>
        }
      />

      {/* Search and Stats */}
      <div className="grid grid-cols-1 lg:grid-cols-4 gap-4">
        <SearchInput
          placeholder="Search agents..."
          value={searchTerm}
          onChange={setSearchTerm}
          className="lg:col-span-1"
        />
        <StatCard label="ACTIVE AGENTS" value={847} icon={Shield} />
        <StatCard label="COMPROMISED" value={3} icon={Shield} valueClassName="text-red-500" iconClassName="text-red-500" />
        <StatCard label="IN TRAINING" value={23} icon={Shield} valueClassName="text-orange-500" iconClassName="text-orange-500" />
      </div>

      {/* Agent List */}
      <Card className="bg-neutral-900 border-neutral-700">
        <CardHeader>
          <CardTitle className="text-sm font-medium text-neutral-300 tracking-wider">AGENT ROSTER</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-neutral-700">
                  <th className="text-left py-3 px-4 text-xs font-medium text-neutral-400 tracking-wider">AGENT ID</th>
                  <th className="text-left py-3 px-4 text-xs font-medium text-neutral-400 tracking-wider">CODENAME</th>
                  <th className="text-left py-3 px-4 text-xs font-medium text-neutral-400 tracking-wider">STATUS</th>
                  <th className="text-left py-3 px-4 text-xs font-medium text-neutral-400 tracking-wider">LOCATION</th>
                  <th className="text-left py-3 px-4 text-xs font-medium text-neutral-400 tracking-wider">LAST SEEN</th>
                  <th className="text-left py-3 px-4 text-xs font-medium text-neutral-400 tracking-wider">MISSIONS</th>
                  <th className="text-left py-3 px-4 text-xs font-medium text-neutral-400 tracking-wider">RISK</th>
                  <th className="text-left py-3 px-4 text-xs font-medium text-neutral-400 tracking-wider">ACTIONS</th>
                </tr>
              </thead>
              <tbody>
                {filteredAgents.map((agent, index) => (
                  <tr
                    key={agent.id}
                    className={`border-b border-neutral-800 hover:bg-neutral-800 transition-colors cursor-pointer ${
                      index % 2 === 0 ? "bg-neutral-900" : "bg-neutral-850"
                    }`}
                    onClick={() => setSelectedAgent(agent)}
                  >
                    <td className="py-3 px-4 text-sm text-white font-mono">{agent.id}</td>
                    <td className="py-3 px-4 text-sm text-white">{agent.name}</td>
                    <td className="py-3 px-4">
                      <div className="flex items-center gap-2">
                        <div className={`w-2 h-2 rounded-full ${getStatusDotColor(agent.status)}`}></div>
                        <span className="text-xs text-neutral-300 uppercase tracking-wider">{agent.status}</span>
                      </div>
                    </td>
                    <td className="py-3 px-4">
                      <div className="flex items-center gap-2">
                        <MapPin className="w-3 h-3 text-neutral-400" />
                        <span className="text-sm text-neutral-300">{agent.location}</span>
                      </div>
                    </td>
                    <td className="py-3 px-4">
                      <div className="flex items-center gap-2">
                        <Clock className="w-3 h-3 text-neutral-400" />
                        <span className="text-sm text-neutral-300 font-mono">{agent.lastSeen}</span>
                      </div>
                    </td>
                    <td className="py-3 px-4 text-sm text-white font-mono">{agent.missions}</td>
                    <td className="py-3 px-4">
                      <span className={`text-xs px-2 py-1 rounded uppercase tracking-wider ${getSeverityColor(agent.risk)}`}>
                        {agent.risk}
                      </span>
                    </td>
                    <td className="py-3 px-4">
                      <Button variant="ghost" size="icon" className="text-neutral-400 hover:text-orange-500">
                        <MoreHorizontal className="w-4 h-4" />
                      </Button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>

      {/* Agent Detail Modal */}
      {selectedAgent && (
        <DetailModal
          title={selectedAgent.name}
          subtitle={selectedAgent.id}
          onClose={() => setSelectedAgent(null)}
          maxWidth="max-w-2xl"
          actions={
            <>
              <ModalPrimaryButton>Assign Mission</ModalPrimaryButton>
              <ModalOutlineButton>View History</ModalOutlineButton>
              <ModalOutlineButton>Send Message</ModalOutlineButton>
            </>
          }
        >
          <div className="grid grid-cols-2 gap-4">
            <div>
              <p className="text-xs text-neutral-400 tracking-wider mb-1">STATUS</p>
              <div className="flex items-center gap-2">
                <div className={`w-2 h-2 rounded-full ${getStatusDotColor(selectedAgent.status)}`}></div>
                <span className="text-sm text-white uppercase tracking-wider">{selectedAgent.status}</span>
              </div>
            </div>
            <div>
              <p className="text-xs text-neutral-400 tracking-wider mb-1">LOCATION</p>
              <p className="text-sm text-white">{selectedAgent.location}</p>
            </div>
            <div>
              <p className="text-xs text-neutral-400 tracking-wider mb-1">MISSIONS COMPLETED</p>
              <p className="text-sm text-white font-mono">{selectedAgent.missions}</p>
            </div>
            <div>
              <p className="text-xs text-neutral-400 tracking-wider mb-1">RISK LEVEL</p>
              <span className={`text-xs px-2 py-1 rounded uppercase tracking-wider ${getSeverityColor(selectedAgent.risk)}`}>
                {selectedAgent.risk}
              </span>
            </div>
          </div>
        </DetailModal>
      )}
    </div>
  )
}
