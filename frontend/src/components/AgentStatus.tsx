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
        return <CheckCircle className="w-4 h-4 text-green-500" />
      case 'inactive':
      case 'untrained':
        return <XCircle className="w-4 h-4 text-red-500" />
      case 'training':
        return <Clock className="w-4 h-4 text-yellow-500" />
      case 'error':
        return <XCircle className="w-4 h-4 text-red-500" />
      default:
        return <XCircle className="w-4 h-4 text-gray-500" />
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
    <div className="glass-effect rounded-xl p-6">
      <div className="flex items-center gap-2 mb-4">
        <Zap className="w-5 h-5 text-blue-600" />
        <h2 className="text-xl font-semibold">Agent Status</h2>
      </div>

      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {/* PII Scanner Agent */}
        <div className="flex items-center gap-3 p-3 bg-white/50 rounded-lg">
          <Cpu className="w-5 h-5 text-blue-600" />
          <div className="flex-1">
            <div className="text-sm font-medium">PII Scanner</div>
            <div className="flex items-center gap-1">
              {getStatusIcon(agentStatus.pii_scanner)}
              <span className={`text-xs ${getStatusColor(agentStatus.pii_scanner)}`}>
                {getStatusText(agentStatus.pii_scanner)}
              </span>
            </div>
          </div>
        </div>

        {/* Compliance Enforcer Agent */}
        <div className="flex items-center gap-3 p-3 bg-white/50 rounded-lg">
          <Shield className="w-5 h-5 text-green-600" />
          <div className="flex-1">
            <div className="text-sm font-medium">Compliance</div>
            <div className="flex items-center gap-1">
              {getStatusIcon(agentStatus.compliance_enforcer)}
              <span className={`text-xs ${getStatusColor(agentStatus.compliance_enforcer)}`}>
                {getStatusText(agentStatus.compliance_enforcer)}
              </span>
            </div>
          </div>
        </div>

        {/* Coordinator Agent */}
        <div className="flex items-center gap-3 p-3 bg-white/50 rounded-lg">
          <Zap className="w-5 h-5 text-purple-600" />
          <div className="flex-1">
            <div className="text-sm font-medium">Coordinator</div>
            <div className="flex items-center gap-1">
              {getStatusIcon(agentStatus.coordinator)}
              <span className={`text-xs ${getStatusColor(agentStatus.coordinator)}`}>
                {getStatusText(agentStatus.coordinator)}
              </span>
            </div>
          </div>
        </div>

        {/* Neural Network */}
        <div className="flex items-center gap-3 p-3 bg-white/50 rounded-lg">
          <Brain className="w-5 h-5 text-indigo-600" />
          <div className="flex-1">
            <div className="text-sm font-medium">Neural Net</div>
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