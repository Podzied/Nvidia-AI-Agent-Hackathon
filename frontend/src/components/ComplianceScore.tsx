'use client'

import { ScanResponse } from '@/lib/types'
import { Shield, AlertTriangle, CheckCircle, TrendingUp } from 'lucide-react'

interface ComplianceScoreProps {
  results: ScanResponse
}

export default function ComplianceScore({ results }: ComplianceScoreProps) {
  const getScoreColor = (score: number) => {
    if (score >= 80) return 'text-green-600'
    if (score >= 60) return 'text-yellow-600'
    return 'text-red-600'
  }

  const getScoreIcon = (score: number) => {
    if (score >= 80) return <CheckCircle className="w-5 h-5" />
    if (score >= 60) return <AlertTriangle className="w-5 h-5" />
    return <AlertTriangle className="w-5 h-5" />
  }

  const getScoreText = (score: number) => {
    if (score >= 80) return 'Excellent'
    if (score >= 60) return 'Good'
    return 'Needs Improvement'
  }

  return (
    <div className="glass-effect rounded-xl p-6">
      <div className="flex items-center gap-2 mb-4">
        <Shield className="w-5 h-5 text-green-600" />
        <h2 className="text-xl font-semibold">Compliance Score</h2>
      </div>

      <div className="space-y-4">
        {/* Score Display */}
        <div className="text-center">
          <div className={`text-4xl font-bold ${getScoreColor(results.compliance_score)} mb-2`}>
            {Math.round(results.compliance_score)}%
          </div>
          <div className="flex items-center justify-center gap-2 text-sm text-gray-600">
            {getScoreIcon(results.compliance_score)}
            {getScoreText(results.compliance_score)}
          </div>
        </div>

        {/* Progress Bar */}
        <div className="w-full bg-gray-200 rounded-full h-2">
          <div 
            className={`h-2 rounded-full transition-all duration-300 ${
              results.compliance_score >= 80 ? 'bg-green-500' : 
              results.compliance_score >= 60 ? 'bg-yellow-500' : 'bg-red-500'
            }`}
            style={{ width: `${results.compliance_score}%` }}
          />
        </div>

        {/* Recommendations */}
        {results.recommendations.length > 0 && (
          <div>
            <h3 className="text-sm font-medium mb-2">Recommendations</h3>
            <div className="space-y-2">
              {results.recommendations.map((rec, index) => (
                <div key={index} className="flex items-start gap-2 text-sm">
                  <TrendingUp className="w-4 h-4 text-blue-600 mt-0.5 flex-shrink-0" />
                  <span className="text-gray-700">{rec}</span>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* PII Count Summary */}
        <div className="text-xs text-gray-500 text-center">
          {results.pii_detected.length} PII items detected
        </div>
      </div>
    </div>
  )
} 