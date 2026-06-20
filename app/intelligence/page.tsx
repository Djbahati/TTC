"use client"

import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { FileText, Eye, Download, Filter, Globe, Shield, AlertTriangle } from "lucide-react"
import { getStatusBadgeColor, getSeverityColor, getClassificationColor } from "@/lib/colors"
import { StatCard } from "@/components/stat-card"
import { PageHeader } from "@/components/page-header"
import { SearchInput } from "@/components/search-input"
import { DetailModal, ModalPrimaryButton, ModalOutlineButton } from "@/components/detail-modal"
import { ProgressBar } from "@/components/progress-bar"

export default function IntelligencePage() {
  type IntelligenceReport = {
    id: string
    title: string
    classification: "TOP SECRET" | "SECRET" | "CONFIDENTIAL" | string
    source: string
    location: string
    date: string
    status: "verified" | "pending" | "active" | string
    threat: "critical" | "high" | "medium" | "low" | string
    summary: string
    tags: string[]
  }

  const [searchTerm, setSearchTerm] = useState("")
  const [selectedReport, setSelectedReport] = useState<IntelligenceReport | null>(null)

  const reports: IntelligenceReport[] = [
    {
      id: "INT-2025-001",
      title: "CYBERCRIME NETWORK ANALYSIS",
      classification: "TOP SECRET",
      source: "SIGINT",
      location: "Eastern Europe",
      date: "2025-06-17",
      status: "verified",
      threat: "high",
      summary: "Detailed analysis of emerging cybercrime syndicate operating across multiple jurisdictions",
      tags: ["cybercrime", "international", "financial"],
    },
    {
      id: "INT-2025-002",
      title: "ROGUE AGENT COMMUNICATIONS",
      classification: "SECRET",
      source: "HUMINT",
      location: "Berlin",
      date: "2025-06-16",
      status: "pending",
      threat: "critical",
      summary: "Intercepted communications suggesting potential security breach in European operations",
      tags: ["internal", "security", "communications"],
    },
    {
      id: "INT-2025-003",
      title: "ARMS TRAFFICKING ROUTES",
      classification: "CONFIDENTIAL",
      source: "OSINT",
      location: "Middle East",
      date: "2025-06-15",
      status: "verified",
      threat: "medium",
      summary: "Updated intelligence on weapons smuggling corridors through Mediterranean region",
      tags: ["trafficking", "weapons", "maritime"],
    },
    {
      id: "INT-2025-004",
      title: "TERRORIST CELL SURVEILLANCE",
      classification: "TOP SECRET",
      source: "HUMINT",
      location: "North Africa",
      date: "2025-06-14",
      status: "active",
      threat: "critical",
      summary: "Ongoing surveillance of suspected terrorist cell planning coordinated attacks",
      tags: ["terrorism", "surveillance", "coordinated"],
    },
    {
      id: "INT-2025-005",
      title: "DIPLOMATIC INTELLIGENCE BRIEF",
      classification: "SECRET",
      source: "DIPLOMATIC",
      location: "Asia Pacific",
      date: "2025-06-13",
      status: "verified",
      threat: "low",
      summary: "Political developments affecting regional security and operational considerations",
      tags: ["diplomatic", "political", "regional"],
    },
  ]

  const threatToProgress: Record<string, number> = {
    critical: 100,
    high: 75,
    medium: 50,
    low: 25,
  }

  const threatToBarColor: Record<string, string> = {
    critical: "bg-red-500",
    high: "bg-orange-500",
    medium: "bg-neutral-400",
    low: "bg-white",
  }

  const filteredReports = reports.filter(
    (report) =>
      report.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
      report.id.toLowerCase().includes(searchTerm.toLowerCase()) ||
      report.tags.some((tag) => tag.toLowerCase().includes(searchTerm.toLowerCase())),
  )

  return (
    <div className="p-6 space-y-6">
      <PageHeader
        title="INTELLIGENCE CENTER"
        subtitle="Classified reports and threat analysis"
        actions={
          <>
            <Button className="bg-orange-500 hover:bg-orange-600 text-white">New Report</Button>
            <Button className="bg-orange-500 hover:bg-orange-600 text-white">
              <Filter className="w-4 h-4 mr-2" />
              Filter
            </Button>
          </>
        }
      />

      {/* Stats and Search */}
      <div className="grid grid-cols-1 lg:grid-cols-5 gap-4">
        <SearchInput
          placeholder="Search intelligence reports..."
          value={searchTerm}
          onChange={setSearchTerm}
          className="lg:col-span-2"
        />
        <StatCard label="TOTAL REPORTS" value="1,247" icon={FileText} />
        <StatCard label="CRITICAL THREATS" value={12} icon={AlertTriangle} valueClassName="text-red-500" iconClassName="text-red-500" />
        <StatCard label="ACTIVE SOURCES" value={89} icon={Globe} />
      </div>

      {/* Intelligence Reports */}
      <Card className="bg-neutral-900 border-neutral-700">
        <CardHeader>
          <CardTitle className="text-sm font-medium text-neutral-300 tracking-wider">INTELLIGENCE REPORTS</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {filteredReports.map((report) => (
              <div
                key={report.id}
                className="border border-neutral-700 rounded p-4 hover:border-orange-500/50 transition-colors cursor-pointer"
                onClick={() => setSelectedReport(report)}
              >
                <div className="flex flex-col sm:flex-row sm:items-start justify-between gap-4">
                  <div className="flex-1 space-y-2">
                    <div className="flex items-start gap-3">
                      <FileText className="w-5 h-5 text-neutral-400 mt-0.5" />
                      <div className="flex-1">
                        <h3 className="text-sm font-bold text-white tracking-wider">{report.title}</h3>
                        <p className="text-xs text-neutral-400 font-mono">{report.id}</p>
                      </div>
                    </div>

                    <p className="text-sm text-neutral-300 ml-8">{report.summary}</p>

                    <div className="flex flex-wrap gap-2 ml-8">
                      {report.tags.map((tag) => (
                        <Badge key={tag} className="bg-neutral-800 text-neutral-300 text-xs">
                          {tag}
                        </Badge>
                      ))}
                    </div>
                  </div>

                  <div className="flex flex-col sm:items-end gap-2">
                    <div className="flex flex-wrap gap-2">
                      <Badge className={getClassificationColor(report.classification)}>{report.classification}</Badge>
                      <Badge className={getSeverityColor(report.threat)}>{report.threat.toUpperCase()}</Badge>
                      <Badge className={getStatusBadgeColor(report.status)}>{report.status.toUpperCase()}</Badge>
                    </div>

                    <div className="text-xs text-neutral-400 space-y-1">
                      <div className="flex items-center gap-2">
                        <Globe className="w-3 h-3" />
                        <span>{report.location}</span>
                      </div>
                      <div className="flex items-center gap-2">
                        <Shield className="w-3 h-3" />
                        <span>{report.source}</span>
                      </div>
                      <div className="font-mono">{report.date}</div>
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* Report Detail Modal */}
      {selectedReport && (
        <DetailModal
          title={selectedReport.title}
          subtitle={selectedReport.id}
          onClose={() => setSelectedReport(null)}
          actions={
            <>
              <ModalPrimaryButton>
                <Eye className="w-4 h-4 mr-2" />
                View Full Report
              </ModalPrimaryButton>
              <ModalOutlineButton>
                <Download className="w-4 h-4 mr-2" />
                Download
              </ModalOutlineButton>
              <ModalOutlineButton>Share Intel</ModalOutlineButton>
            </>
          }
        >
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="space-y-4">
              <div>
                <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">CLASSIFICATION</h3>
                <div className="flex gap-2">
                  <Badge className={getClassificationColor(selectedReport.classification)}>
                    {selectedReport.classification}
                  </Badge>
                  <Badge className={getSeverityColor(selectedReport.threat)}>
                    THREAT: {selectedReport.threat.toUpperCase()}
                  </Badge>
                </div>
              </div>

              <div>
                <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">SOURCE DETAILS</h3>
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-neutral-400">Source Type:</span>
                    <span className="text-white font-mono">{selectedReport.source}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-neutral-400">Location:</span>
                    <span className="text-white">{selectedReport.location}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-neutral-400">Date:</span>
                    <span className="text-white font-mono">{selectedReport.date}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-neutral-400">Status:</span>
                    <Badge className={getStatusBadgeColor(selectedReport.status)}>
                      {selectedReport.status.toUpperCase()}
                    </Badge>
                  </div>
                </div>
              </div>
            </div>

            <div className="space-y-4">
              <div>
                <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">TAGS</h3>
                <div className="flex flex-wrap gap-2">
                  {selectedReport.tags.map((tag) => (
                    <Badge key={tag} className="bg-neutral-800 text-neutral-300">
                      {tag}
                    </Badge>
                  ))}
                </div>
              </div>

              <div>
                <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">THREAT ASSESSMENT</h3>
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span className="text-neutral-400">Threat Level</span>
                    <Badge className={getSeverityColor(selectedReport.threat)}>
                      {selectedReport.threat.toUpperCase()}
                    </Badge>
                  </div>
                  <ProgressBar
                    value={threatToProgress[selectedReport.threat] ?? 50}
                    barColor={threatToBarColor[selectedReport.threat] ?? "bg-neutral-400"}
                  />
                </div>
              </div>
            </div>
          </div>

          <div>
            <h3 className="text-sm font-medium text-neutral-300 tracking-wider mb-2">EXECUTIVE SUMMARY</h3>
            <p className="text-sm text-neutral-300 leading-relaxed">{selectedReport.summary}</p>
          </div>
        </DetailModal>
      )}
    </div>
  )
}
