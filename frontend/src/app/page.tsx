'use client'

import { useState } from 'react'
import PiiScanner from '@/components/PiiScanner'
import ResultsDisplay from '@/components/ResultsDisplay'
import ComplianceScore from '@/components/ComplianceScore'
import AgentStatus from '@/components/AgentStatus'
import { ScanResponse } from '@/lib/types'

export default function Home() {
  const [scanResults, setScanResults] = useState<ScanResponse | null>(null)
  const [isScanning, setIsScanning] = useState(false)

  return (
    <div className="container mx-auto px-4 py-8">
      {/* Header */}
      <div className="text-center mb-12">
        <h1 className="text-4xl font-bold gradient-text mb-4">
          PII Compliance AI Agent
        </h1>
        <p className="text-lg text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
          Neural network-powered PII detection and compliance enforcement
        </p>
      </div>

      {/* Agent Status */}
      <div className="mb-8">
        <AgentStatus />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Scanner Section */}
        <div className="space-y-6">
          <PiiScanner 
            onScan={setScanResults}
            isScanning={isScanning}
            setIsScanning={setIsScanning}
          />
        </div>

        {/* Results Section */}
        <div className="space-y-6">
          {scanResults && (
            <>
              <ResultsDisplay results={scanResults} />
              <ComplianceScore results={scanResults} />
            </>
          )}
        </div>
      </div>
    </div>
  )
} 