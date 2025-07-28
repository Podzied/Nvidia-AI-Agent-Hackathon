'use client'

import { useState } from 'react'
import ChatInterface from '@/components/ChatInterface'
import AgentStatus from '@/components/AgentStatus'
import { ScanResponse } from '@/lib/types'

export default function Home() {
  const [scanResults, setScanResults] = useState<ScanResponse | null>(null)
  const [isScanning, setIsScanning] = useState(false)

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-800">
      {/* Header */}
      <div className="text-center py-8">
        <h1 className="text-4xl font-bold gradient-text mb-4">
          AI Assistant
        </h1>
        <p className="text-lg text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
          Your intelligent conversation partner with built-in privacy protection
        </p>
      </div>

      {/* Agent Status */}
      <div className="max-w-4xl mx-auto px-4 mb-6">
        <AgentStatus />
      </div>

      {/* Chat Interface */}
      <div className="max-w-4xl mx-auto px-4">
        <ChatInterface 
          onScan={setScanResults}
          isScanning={isScanning}
          setIsScanning={setIsScanning}
          scanResults={scanResults}
        />
      </div>
    </div>
  )
} 