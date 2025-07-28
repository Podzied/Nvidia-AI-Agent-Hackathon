'use client'

import { useState } from 'react'
import { Scan, Shield, AlertTriangle } from 'lucide-react'
import { ScanResponse } from '@/lib/types'

interface PiiScannerProps {
  onScan: (results: ScanResponse) => void
  isScanning: boolean
  setIsScanning: (scanning: boolean) => void
}

export default function PiiScanner({ onScan, isScanning, setIsScanning }: PiiScannerProps) {
  const [text, setText] = useState('')
  const [error, setError] = useState('')

  const handleScan = async () => {
    if (!text.trim()) {
      setError('Please enter some text to scan')
      return
    }

    setIsScanning(true)
    setError('')

    try {
      const response = await fetch('/api/scan', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ text }),
      })

      if (!response.ok) {
        throw new Error('Failed to scan text')
      }

      const results = await response.json()
      onScan(results)
    } catch (err) {
      setError('Failed to scan text. Please try again.')
      console.error('Scan error:', err)
    } finally {
      setIsScanning(false)
    }
  }

  return (
    <div className="glass-effect rounded-xl p-6">
      <div className="flex items-center gap-2 mb-4">
        <Shield className="w-5 h-5 text-blue-600" />
        <h2 className="text-xl font-semibold">PII Scanner</h2>
      </div>

      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium mb-2">
            Enter text to scan for PII
          </label>
          <textarea
            value={text}
            onChange={(e) => setText(e.target.value)}
            placeholder="Enter text containing potential PII (emails, phone numbers, SSNs, etc.)..."
            className="w-full h-32 p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
            disabled={isScanning}
          />
        </div>

        {error && (
          <div className="flex items-center gap-2 text-red-600 text-sm">
            <AlertTriangle className="w-4 h-4" />
            {error}
          </div>
        )}

        <button
          onClick={handleScan}
          disabled={isScanning || !text.trim()}
          className="w-full bg-gradient-to-r from-blue-600 to-purple-600 text-white py-3 px-6 rounded-lg font-medium hover:from-blue-700 hover:to-purple-700 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 flex items-center justify-center gap-2"
        >
          {isScanning ? (
            <>
              <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
              Scanning...
            </>
          ) : (
            <>
              <Scan className="w-4 h-4" />
              Scan for PII
            </>
          )}
        </button>
      </div>
    </div>
  )
} 