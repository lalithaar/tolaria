import { memo } from 'react'
import type { VaultEntry } from '../types'
import { cn } from '@/lib/utils'
import { X } from 'lucide-react'
import { Plus, Columns, ArrowsOutSimple } from '@phosphor-icons/react'

interface Tab {
  entry: VaultEntry
  content: string
}

interface TabBarProps {
  tabs: Tab[]
  activeTabPath: string | null
  onSwitchTab: (path: string) => void
  onCloseTab: (path: string) => void
  onCreateNote?: () => void
}

const DISABLED_ICON_STYLE = { opacity: 0.4, cursor: 'not-allowed' } as const

export const TabBar = memo(function TabBar({
  tabs, activeTabPath, onSwitchTab, onCloseTab, onCreateNote,
}: TabBarProps) {
  return (
    <div
      className="flex shrink-0 items-stretch"
      style={{ height: 45, background: 'var(--sidebar)', WebkitAppRegion: 'drag' } as React.CSSProperties}
      data-tauri-drag-region
    >
      {tabs.map((tab) => {
        const isActive = tab.entry.path === activeTabPath
        return (
          <div
            key={tab.entry.path}
            className={cn(
              "group flex shrink-0 cursor-pointer items-center gap-1.5 whitespace-nowrap max-w-[180px] transition-all",
              isActive
                ? "text-foreground"
                : "text-muted-foreground hover:text-secondary-foreground"
            )}
            style={{
              background: isActive ? 'var(--background)' : 'transparent',
              borderRight: `1px solid ${isActive ? 'var(--border)' : 'var(--sidebar-border)'}`,
              borderBottom: isActive ? 'none' : '1px solid var(--sidebar-border)',
              padding: '0 12px',
              fontSize: 12,
              fontWeight: isActive ? 500 : 400,
              WebkitAppRegion: 'no-drag',
            } as React.CSSProperties}
            onClick={() => onSwitchTab(tab.entry.path)}
          >
            <span className="truncate">{tab.entry.title}</span>
            <button
              className={cn(
                "shrink-0 rounded-sm p-0 bg-transparent border-none text-muted-foreground cursor-pointer transition-opacity hover:bg-accent hover:text-foreground",
                isActive ? "opacity-100" : "opacity-0 group-hover:opacity-100"
              )}
              style={{ lineHeight: 0 }}
              onClick={(e) => {
                e.stopPropagation()
                onCloseTab(tab.entry.path)
              }}
            >
              <X size={14} />
            </button>
          </div>
        )
      })}

      <div className="flex-1" style={{ borderBottom: '1px solid var(--border)' }} />

      <div
        className="flex shrink-0 items-center"
        style={{
          borderLeft: '1px solid var(--border)',
          borderBottom: '1px solid var(--border)',
          gap: 12,
          padding: '0 12px',
          WebkitAppRegion: 'no-drag',
        } as React.CSSProperties}
      >
        <button
          className="flex items-center justify-center border-none bg-transparent p-0 text-muted-foreground cursor-pointer hover:text-foreground transition-colors"
          onClick={onCreateNote}
          title="New note"
        >
          <Plus size={16} />
        </button>
        <button
          className="flex items-center justify-center border-none bg-transparent p-0 text-muted-foreground"
          style={DISABLED_ICON_STYLE}
          title="Coming soon"
          tabIndex={-1}
        >
          <Columns size={16} />
        </button>
        <button
          className="flex items-center justify-center border-none bg-transparent p-0 text-muted-foreground"
          style={DISABLED_ICON_STYLE}
          title="Coming soon"
          tabIndex={-1}
        >
          <ArrowsOutSimple size={16} />
        </button>
      </div>
    </div>
  )
})
