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

    // TODO: Replace with actual Rust backend call
    // For now, simulate the API response
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
  } catch (error) {
    console.error('Scan API error:', error)
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    )
  }
} 