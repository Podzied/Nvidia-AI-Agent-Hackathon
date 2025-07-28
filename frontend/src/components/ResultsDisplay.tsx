'use client'

import { ScanResponse } from '@/lib/types'
import { Eye, EyeOff, Copy, Check } from 'lucide-react'
import { useState } from 'react'

interface ResultsDisplayProps {
  results: ScanResponse
}

export default function ResultsDisplay({ results }: ResultsDisplayProps) {
  const [showRedacted, setShowRedacted] = useState(false)
  const [copied, setCopied] = useState(false)

  const copyToClipboard = async (text: string) => {
    await navigator.clipboard.writeText(text)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const getPiiTypeColor = (type: string) => {
    const colors: { [key: string]: string } = {
      email: 'bg-red-100 text-red-800',
      phone: 'bg-blue-100 text-blue-800',
      ssn: 'bg-purple-100 text-purple-800',
      credit_card: 'bg-green-100 text-green-800',
      ip_address: 'bg-yellow-100 text-yellow-800',
      name: 'bg-indigo-100 text-indigo-800',
    }
    return colors[type] || 'bg-gray-100 text-gray-800'
  }

  return (
    <div className="glass-effect rounded-xl p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-xl font-semibold">Scan Results</h2>
        <div className="flex gap-2">
          <button
            onClick={() => setShowRedacted(!showRedacted)}
            className="flex items-center gap-2 px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors"
          >
            {showRedacted ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
            {showRedacted ? 'Show Original' : 'Show Redacted'}
          </button>
          <button
            onClick={() => copyToClipboard(showRedacted ? results.redacted_text : results.text)}
            className="flex items-center gap-2 px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors"
          >
            {copied ? <Check className="w-4 h-4" /> : <Copy className="w-4 h-4" />}
            {copied ? 'Copied!' : 'Copy'}
          </button>
        </div>
      </div>

      <div className="space-y-4">
        {/* Text Display */}
        <div>
          <label className="block text-sm font-medium mb-2">
            {showRedacted ? 'Redacted Text' : 'Original Text'}
          </label>
          <div className="p-3 bg-gray-50 rounded-lg border text-sm">
            {showRedacted ? results.redacted_text : results.text}
          </div>
        </div>

        {/* PII Detected */}
        {results.pii_detected.length > 0 && (
          <div>
            <label className="block text-sm font-medium mb-2">
              PII Detected ({results.pii_detected.length})
            </label>
            <div className="space-y-2">
              {results.pii_detected.map((pii, index) => (
                <div key={index} className="flex items-center justify-between p-3 bg-white rounded-lg border">
                  <div className="flex items-center gap-3">
                    <span className={`px-2 py-1 rounded-full text-xs font-medium ${getPiiTypeColor(pii.type)}`}>
                      {pii.type}
                    </span>
                    <span className="text-sm font-mono">{pii.value}</span>
                  </div>
                  <div className="text-sm text-gray-500">
                    {Math.round(pii.confidence * 100)}% confidence
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Processing Info */}
        <div className="text-xs text-gray-500">
          Processed in {results.processing_time}ms
        </div>
      </div>
    </div>
  )
} 