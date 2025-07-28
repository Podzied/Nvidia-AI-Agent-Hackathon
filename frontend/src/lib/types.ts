export interface PiiResult {
  type: string
  value: string
  confidence: number
  position: {
    start: number
    end: number
  }
}

export interface ScanResponse {
  text: string
  pii_detected: PiiResult[]
  compliance_score: number
  redacted_text: string
  recommendations: string[]
  processing_time: number
}

export interface AgentStatus {
  pii_scanner: 'active' | 'inactive' | 'error'
  compliance_enforcer: 'active' | 'inactive' | 'error'
  coordinator: 'active' | 'inactive' | 'error'
  neural_network: 'trained' | 'untrained' | 'training'
} 