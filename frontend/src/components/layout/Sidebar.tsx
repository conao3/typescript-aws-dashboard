import { Link } from '@tanstack/react-router'
import { Home, Settings, Users } from 'lucide-react'

export function Sidebar() {
  return (
    <aside className="w-64 bg-gray-900 text-white">
      <div className="flex h-full flex-col">
        <div className="p-4">
          <h2 className="text-xl font-bold">Navigation</h2>
        </div>
        <nav className="flex-1 space-y-1 px-2 py-4">
          <SidebarLink to="/" icon={<Home size={20} />}>
            Dashboard
          </SidebarLink>
          <SidebarLink to="/users" icon={<Users size={20} />}>
            Users
          </SidebarLink>
          <SidebarLink to="/settings" icon={<Settings size={20} />}>
            Settings
          </SidebarLink>
        </nav>
      </div>
    </aside>
  )
}

function SidebarLink({
  to,
  icon,
  children,
}: {
  to: string
  icon: React.ReactNode
  children: React.ReactNode
}) {
  return (
    <Link
      to={to}
      className="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium hover:bg-gray-800"
      activeProps={{
        className: 'bg-gray-800',
      }}
    >
      {icon}
      {children}
    </Link>
  )
}
