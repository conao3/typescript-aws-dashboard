import { createFileRoute } from '@tanstack/react-router'
import { Button } from 'react-aria-components'

export const Route = createFileRoute('/')({
  component: Home,
})

function Home() {
  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center">
      <div className="text-center">
        <h1 className="text-4xl font-bold text-gray-900 mb-4">
          AWS Dashboard
        </h1>
        <p className="text-lg text-gray-600 mb-8">
          Built with TanStack Start, Tailwind CSS, React Aria Components, and Apollo Client
        </p>
        <Button className="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
          Get Started
        </Button>
      </div>
    </div>
  )
}
