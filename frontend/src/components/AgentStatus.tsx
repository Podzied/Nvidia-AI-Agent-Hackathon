'use client'

import { useState, useEffect } from 'react'
import { Cpu, Brain, Zap, CheckCircle, XCircle, Clock, Shield } from 'lucide-react'
import { AgentStatus as AgentStatusType } from '@/lib/types'

export default function AgentStatus() {
  const [agentStatus, setAgentStatus] = useState<AgentStatusType>({
    pii_scanner: 'active',
    compliance_enforcer: 'active',
    coordinator: 'active',
    neural_network: 'trained'
  })

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active':
      case 'trained':
        return <CheckCircle className="w-3 h-3 text-green-500" />
      case 'inactive':
      case 'untrained':
        return <XCircle className="w-3 h-3 text-red-500" />
      case 'training':
        return <Clock className="w-3 h-3 text-yellow-500" />
      case 'error':
        return <XCircle className="w-3 h-3 text-red-500" />
      default:
        return <XCircle className="w-3 h-3 text-gray-500" />
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
      case 'trained':
        return 'text-green-600'
      case 'inactive':
      case 'untrained':
        return 'text-red-600'
      case 'training':
        return 'text-yellow-600'
      case 'error':
        return 'text-red-600'
      default:
        return 'text-gray-600'
    }
  }

  const getStatusText = (status: string) => {
    switch (status) {
      case 'active':
        return 'Active'
      case 'inactive':
        return 'Inactive'
      case 'trained':
        return 'Trained'
      case 'untrained':
        return 'Untrained'
      case 'training':
        return 'Training'
      case 'error':
        return 'Error'
      default:
        return 'Unknown'
    }
  }

  return (
    <div className="glass-effect rounded-xl p-4">
      <div className="flex items-center gap-2 mb-3">
        <Zap className="w-4 h-4 text-blue-600" />
        <h2 className="text-sm font-semibold">AI Agents Status</h2>
      </div>

      <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
        {/* PII Scanner Agent */}
        <div className="flex items-center gap-2 p-2 bg-white/50 rounded-lg">
          <Cpu className="w-4 h-4 text-blue-600" />
          <div className="flex-1 min-w-0">
            <div className="text-xs font-medium truncate">Scanner</div>
            <div className="flex items-center gap-1">
              {getStatusIcon(agentStatus.pii_scanner)}
              <span className={`text-xs ${getStatusColor(agentStatus.pii_scanner)}`}>
                {getStatusText(agentStatus.pii_scanner)}
              </span>
            </div>
          </div>
        </div>

        {/* Compliance Enforcer Agent */}
        <div className="flex items-center gap-2 p-2 bg-white/50 rounded-lg">
          <Shield className="w-4 h-4 text-green-600" />
          <div className="flex-1 min-w-0">
            <div className="text-xs font-medium truncate">Compliance</div>
            <div className="flex items-center gap-1">
              {getStatusIcon(agentStatus.compliance_enforcer)}
              <span className={`text-xs ${getStatusColor(agentStatus.compliance_enforcer)}`}>
                {getStatusText(agentStatus.compliance_enforcer)}
              </span>
            </div>
          </div>
        </div>

        {/* Coordinator Agent */}
        <div className="flex items-center gap-2 p-2 bg-white/50 rounded-lg">
          <Zap className="w-4 h-4 text-purple-600" />
          <div className="flex-1 min-w-0">
            <div className="text-xs font-medium truncate">Coordinator</div>
            <div className="flex items-center gap-1">
              {getStatusIcon(agentStatus.coordinator)}
              <span className={`text-xs ${getStatusColor(agentStatus.coordinator)}`}>
                {getStatusText(agentStatus.coordinator)}
              </span>
            </div>
          </div>
        </div>

        {/* Neural Network */}
        <div className="flex items-center gap-2 p-2 bg-white/50 rounded-lg">
          <Brain className="w-4 h-4 text-indigo-600" />
          <div className="flex-1 min-w-0">
            <div className="text-xs font-medium truncate">Neural Net</div>
            <div className="flex items-center gap-1">
              {getStatusIcon(agentStatus.neural_network)}
              <span className={`text-xs ${getStatusColor(agentStatus.neural_network)}`}>
                {getStatusText(agentStatus.neural_network)}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
} 