'use client'

import { useState, useRef, useEffect } from 'react'
import { Send, Bot, User, Shield, AlertTriangle, X } from 'lucide-react'
import { ScanResponse } from '@/lib/types'

interface Message {
  id: string
  type: 'user' | 'bot'
  content: string
  timestamp: Date
}

interface ChatInterfaceProps {
  onScan: (results: ScanResponse) => void
  isScanning: boolean
  setIsScanning: (scanning: boolean) => void
  scanResults: ScanResponse | null
}

export default function ChatInterface({ onScan, isScanning, setIsScanning, scanResults }: ChatInterfaceProps) {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: '1',
      type: 'bot',
      content: "Hi there! 👋 I'm your AI assistant. How can I help you today?",
      timestamp: new Date()
    }
  ])
  const [inputText, setInputText] = useState('')
  const [showPiiPopup, setShowPiiPopup] = useState(false)
  const [piiAlert, setPiiAlert] = useState<ScanResponse | null>(null)
  const messagesEndRef = useRef<HTMLDivElement>(null)

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }

  useEffect(() => {
    scrollToBottom()
  }, [messages])

  // Check for PII in background when scan results change
  useEffect(() => {
    if (scanResults && scanResults.pii_detected.length > 0) {
      setPiiAlert(scanResults)
      setShowPiiPopup(true)
    }
  }, [scanResults])

  const handleSendMessage = async () => {
    if (!inputText.trim() || isScanning) return

    const userMessage: Message = {
      id: Date.now().toString(),
      type: 'user',
      content: inputText,
      timestamp: new Date()
    }

    setMessages(prev => [...prev, userMessage])
    const currentText = inputText
    setInputText('')
    setIsScanning(true)

    // Add bot thinking message
    const thinkingMessage: Message = {
      id: (Date.now() + 1).toString(),
      type: 'bot',
      content: "Let me think about that... 🤔",
      timestamp: new Date()
    }
    setMessages(prev => [...prev, thinkingMessage])

    // Simulate AI response
    setTimeout(() => {
      const botResponse = generateBotResponse(currentText)
      
      setMessages(prev => {
        const filtered = prev.filter(msg => msg.id !== thinkingMessage.id)
        return [...filtered, {
          id: (Date.now() + 2).toString(),
          type: 'bot',
          content: botResponse,
          timestamp: new Date()
        }]
      })
    }, 1000 + Math.random() * 2000) // Random delay for natural feel

    // Scan for PII in background
    try {
      const response = await fetch('/api/scan', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ text: currentText }),
      })

      if (response.ok) {
        const results = await response.json()
        onScan(results)
      }
    } catch (err) {
      console.error('PII scan error:', err)
    } finally {
      setIsScanning(false)
    }
  }

  const generateBotResponse = (userText: string): string => {
    const responses = [
      "That's interesting! Tell me more about that.",
      "I see what you mean. What are your thoughts on that?",
      "That's a great point! How do you feel about it?",
      "Interesting perspective! Can you elaborate?",
      "I understand. What would you like to explore next?",
      "That's fascinating! What made you think of that?",
      "I see where you're coming from. What's your take on this?",
      "That's a good question! Let me think about that...",
      "Interesting! How did you come to that conclusion?",
      "That's a valid point. What are your next steps?"
    ]
    
    return responses[Math.floor(Math.random() * responses.length)]
  }

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSendMessage()
    }
  }

  const closePiiPopup = () => {
    setShowPiiPopup(false)
    setPiiAlert(null)
  }

  return (
    <div className="relative">
      <div className="glass-effect rounded-xl h-[600px] flex flex-col">
        {/* Chat Header */}
        <div className="flex items-center gap-3 p-4 border-b border-gray-200 dark:border-gray-700">
          <div className="w-8 h-8 bg-gradient-to-r from-blue-500 to-purple-500 rounded-full flex items-center justify-center">
            <Bot className="w-5 h-5 text-white" />
          </div>
          <div>
            <h3 className="font-semibold">AI Assistant</h3>
            <p className="text-xs text-gray-500">Always here to help</p>
          </div>
        </div>

        {/* Messages */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          {messages.map((message) => (
            <div key={message.id} className={`flex ${message.type === 'user' ? 'justify-end' : 'justify-start'}`}>
              <div className={`flex gap-3 max-w-[80%] ${message.type === 'user' ? 'flex-row-reverse' : ''}`}>
                <div className={`w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0 ${
                  message.type === 'user' 
                    ? 'bg-blue-500' 
                    : 'bg-gradient-to-r from-blue-500 to-purple-500'
                }`}>
                  {message.type === 'user' ? (
                    <User className="w-4 h-4 text-white" />
                  ) : (
                    <Bot className="w-4 h-4 text-white" />
                  )}
                </div>
                <div className={`rounded-lg p-3 ${
                  message.type === 'user' 
                    ? 'bg-blue-500 text-white' 
                    : 'bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700'
                }`}>
                  <p className="text-sm">{message.content}</p>
                  <p className="text-xs opacity-70 mt-2">
                    {message.timestamp.toLocaleTimeString()}
                  </p>
                </div>
              </div>
            </div>
          ))}
          <div ref={messagesEndRef} />
        </div>

        {/* Input */}
        <div className="p-4 border-t border-gray-200 dark:border-gray-700">
          <div className="flex gap-2">
            <textarea
              value={inputText}
              onChange={(e) => setInputText(e.target.value)}
              onKeyPress={handleKeyPress}
              placeholder="Type your message here..."
              className="flex-1 p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
              rows={2}
              disabled={isScanning}
            />
            <button
              onClick={handleSendMessage}
              disabled={isScanning || !inputText.trim()}
              className="px-4 py-2 bg-gradient-to-r from-blue-500 to-purple-500 text-white rounded-lg hover:from-blue-600 hover:to-purple-600 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 flex items-center justify-center"
            >
              {isScanning ? (
                <div className="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin" />
              ) : (
                <Send className="w-5 h-5" />
              )}
            </button>
          </div>
        </div>
      </div>

      {/* PII Alert Popup */}
      {showPiiPopup && piiAlert && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-xl p-6 max-w-md mx-4 shadow-2xl">
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center gap-2">
                <Shield className="w-5 h-5 text-red-500" />
                <h3 className="font-semibold text-red-600">⚠️ PII Detected</h3>
              </div>
              <button
                onClick={closePiiPopup}
                className="text-gray-400 hover:text-gray-600"
              >
                <X className="w-5 h-5" />
              </button>
            </div>
            
            <div className="space-y-3">
              <p className="text-sm text-gray-600 dark:text-gray-300">
                I detected {piiAlert.pii_detected.length} PII item{piiAlert.pii_detected.length > 1 ? 's' : ''} in your message:
              </p>
              
              <div className="space-y-2">
                {piiAlert.pii_detected.map((pii, index) => (
                  <div key={index} className="flex items-center gap-2 p-2 bg-red-50 dark:bg-red-900/20 rounded-lg">
                    <AlertTriangle className="w-4 h-4 text-red-500" />
                    <div className="flex-1">
                      <div className="text-sm font-medium text-red-700 dark:text-red-300">
                        {pii.type.toUpperCase()}
                      </div>
                      <div className="text-xs text-red-600 dark:text-red-400 font-mono">
                        {pii.value}
                      </div>
                    </div>
                    <div className="text-xs text-red-500">
                      {Math.round(pii.confidence * 100)}%
                    </div>
                  </div>
                ))}
              </div>
              
              <div className="flex items-center gap-2 p-2 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
                <Shield className="w-4 h-4 text-blue-500" />
                <span className="text-sm text-blue-700 dark:text-blue-300">
                  Compliance Score: {Math.round(piiAlert.compliance_score)}%
                </span>
              </div>
              
              {piiAlert.recommendations.length > 0 && (
                <div className="text-xs text-gray-600 dark:text-gray-400">
                  <p className="font-medium mb-1">Recommendations:</p>
                  <ul className="space-y-1">
                    {piiAlert.recommendations.map((rec, index) => (
                      <li key={index} className="flex items-start gap-2">
                        <span className="text-blue-500">•</span>
                        <span>{rec}</span>
                      </li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
            
            <div className="mt-4 flex gap-2">
              <button
                onClick={closePiiPopup}
                className="flex-1 px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
              >
                Dismiss
              </button>
              <button
                onClick={() => {
                  // Copy redacted text to clipboard
                  navigator.clipboard.writeText(piiAlert.redacted_text)
                  closePiiPopup()
                }}
                className="flex-1 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
              >
                Copy Safe Text
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
} 