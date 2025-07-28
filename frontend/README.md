# PII Compliance AI Agent - Frontend

A sleek, modern Next.js frontend for the PII Compliance AI Agent system.

## Features

- **Real-time PII Scanning**: Interactive text input with instant PII detection
- **Results Visualization**: Beautiful display of detected PII with confidence scores
- **Compliance Scoring**: Visual compliance metrics and recommendations
- **Agent Status Monitoring**: Real-time status of all AI agents
- **Responsive Design**: Works seamlessly on desktop and mobile devices
- **Modern UI**: Glass morphism effects and gradient designs

## Tech Stack

- **Next.js 14**: React framework with App Router
- **TypeScript**: Full type safety
- **Tailwind CSS**: Utility-first styling
- **Lucide React**: Beautiful icons
- **Glass Morphism**: Modern UI effects

## Getting Started

### Prerequisites

- Node.js 18+ 
- npm or yarn

### Installation

1. Navigate to the frontend directory:
```bash
cd frontend
```

2. Install dependencies:
```bash
npm install
```

3. Start the development server:
```bash
npm run dev
```

4. Open [http://localhost:3000](http://localhost:3000) in your browser.

## Usage

### Scanning Text for PII

1. Enter text in the PII Scanner textarea
2. Click "Scan for PII" button
3. View results in the Results Display panel
4. Toggle between original and redacted text
5. Copy text to clipboard

### Viewing Compliance Score

- Compliance score is automatically calculated
- Color-coded progress bar shows compliance level
- Recommendations are provided for improvement

### Monitoring Agent Status

- Real-time status of all AI agents
- Visual indicators for active/inactive states
- Neural network training status

## API Integration

The frontend is designed to integrate with the Rust backend:

- API routes are configured in `/src/app/api/`
- Backend calls are proxied through Next.js
- Mock responses are provided for development

## Development

### Project Structure

```
frontend/
├── src/
│   ├── app/                 # Next.js App Router
│   │   ├── api/            # API routes
│   │   ├── globals.css     # Global styles
│   │   ├── layout.tsx      # Root layout
│   │   └── page.tsx        # Main page
│   ├── components/         # React components
│   │   ├── PiiScanner.tsx
│   │   ├── ResultsDisplay.tsx
│   │   ├── ComplianceScore.tsx
│   │   └── AgentStatus.tsx
│   └── lib/               # Utilities and types
│       └── types.ts
├── package.json
├── tailwind.config.js
└── next.config.js
```

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run start` - Start production server
- `npm run lint` - Run ESLint

## Customization

### Styling

- Modify `tailwind.config.js` for theme customization
- Update `globals.css` for custom CSS variables
- Component-specific styles in individual component files

### Components

- All components are modular and reusable
- TypeScript interfaces ensure type safety
- Props are well-documented

## Deployment

### Vercel (Recommended)

1. Push to GitHub
2. Connect repository to Vercel
3. Deploy automatically

### Other Platforms

- Build with `npm run build`
- Deploy the `out` directory
- Configure environment variables as needed

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - see LICENSE file for details. 