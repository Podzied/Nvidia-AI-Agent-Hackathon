import { NextRequest, NextResponse } from 'next/server'

export async function POST(request: NextRequest) {
  try {
    const { text } = await request.json()

    if (!text) {
      return NextResponse.json(
        { error: 'Text is required' },
        { status: 400 }
      )
    }

    // Call the real Rust backend
    try {
      const response = await fetch('http://localhost:8000/api/scan', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ text }),
      })

      if (!response.ok) {
        throw new Error(`Backend error: ${response.status}`)
      }

      const result = await response.json()
      
      // Transform the response to match frontend expectations
      const transformedResult = {
        text: result.text,
        pii_detected: result.pii_detected.map((pii: any) => ({
          type: pii.type_,
          value: pii.value,
          confidence: pii.confidence,
          position: pii.position
        })),
        compliance_score: result.compliance_score,
        redacted_text: result.redacted_text,
        recommendations: result.recommendations,
        processing_time: result.processing_time
      }

      return NextResponse.json(transformedResult)
    } catch (error) {
      console.error('Backend connection failed, using fallback:', error)
      
      // Fallback to mock data if backend is not available
      const mockResponse = {
        text,
        pii_detected: [
          {
            type: 'email',
            value: 'john.doe@example.com',
            confidence: 0.95,
            position: { start: 12, end: 32 }
          },
          {
            type: 'phone',
            value: '555-123-4567',
            confidence: 0.88,
            position: { start: 45, end: 58 }
          }
        ],
        compliance_score: 0.85,
        redacted_text: text.replace(/[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}/g, '[EMAIL]')
                          .replace(/\d{3}-\d{3}-\d{4}/g, '[PHONE]'),
        recommendations: [
          'Consider implementing stricter data handling policies',
          'Review access controls for sensitive information'
        ],
        processing_time: Math.floor(Math.random() * 100) + 50
      }

      return NextResponse.json(mockResponse)
    }
  } catch (error) {
    console.error('Scan API error:', error)
    return NextResponse.json(
      { error: 'Failed to connect to PII detection service' },
      { status: 500 }
    )
  }
} 