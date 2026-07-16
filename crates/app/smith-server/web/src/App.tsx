import { useState, useEffect, useCallback, useRef } from 'react'
import './app.css'
import { ProvidersPanel } from './components/ProvidersPanel'
import { AgentsPanel } from './components/AgentsPanel'
import { ChatArea } from './components/ChatArea'
import { SessionsPanel } from './components/SessionsPanel'
import { SettingsPanel } from './components/SettingsPanel'
import * as api from './api'
import type { AgentSummary, ProviderConfig, SessionSummary, ChatMessage, Config } from './types'

type Tab = 'agents' | 'providers'

type ViewMode = 'normal' | 'wide' | 'simple'

interface Toast { id: number; msg: string; type: 'error' | 'success' | 'info' }

let toastId = 0;

function loadViewMode(): ViewMode {
  try {
    const saved = localStorage.getItem('smith_view_mode')
    if (saved === 'wide' || saved === 'simple') return saved
  } catch { /* ignore */ }
  return 'normal'
}

function saveViewMode(mode: ViewMode) {
  try { localStorage.setItem('smith_view_mode', mode) }
  catch { /* ignore */ }
}

export default function App() {
  const [tab, setTab] = useState<Tab>('agents')
  const [agents, setAgents] = useState<AgentSummary[]>([])
  const [providers, setProviders] = useState<ProviderConfig[]>([])
  const [selectedAgent, setSelectedAgent] = useState<AgentSummary | null>(null)
  const [sessions, setSessions] = useState<SessionSummary[]>([])
  const [currentSessionId, setCurrentSessionId] = useState<string | null>(null)
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const [toasts, setToasts] = useState<Toast[]>([])
  const [config, setConfig] = useState<Config | null>(null)
  const [showSettings, setShowSettings] = useState(false)
  const [viewMode, setViewMode] = useState<ViewMode>(loadViewMode)
  const notifIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null)

  const addToast = useCallback((msg: string, type: 'error' | 'success' | 'info' = 'error') => {
    const id = ++toastId
    setToasts(prev => [...prev, { id, msg, type }])
    setTimeout(() => setToasts(prev => prev.filter(t => t.id !== id)), 4000)
  }, [])

  const loadAgents = useCallback(async () => {
    try { setAgents(await api.listAgents()) }
    catch (e: any) { addToast(e.message) }
  }, [addToast])

  const loadProviders = useCallback(async () => {
    try { setProviders(await api.listProviders()) }
    catch (e: any) { addToast(e.message) }
  }, [addToast])

  const loadConfig = useCallback(async () => {
    try { setConfig(await api.getConfig()) }
    catch { /* config not critical */ }
  }, [])

  const loadSessions = useCallback(async (agentId: string) => {
    try { setSessions(await api.listSessions(agentId)) }
    catch { /* ignore */ }
  }, [])

  const selectAgent = useCallback((agent: AgentSummary) => {
    setSelectedAgent(agent)
    setCurrentSessionId(null)
    setMessages([])
    loadSessions(agent.id)
  }, [loadSessions])

  const handleViewModeChange = useCallback((mode: ViewMode) => {
    setViewMode(mode)
    saveViewMode(mode)
  }, [])

  useEffect(() => { loadProviders(); loadAgents(); loadConfig() }, [loadProviders, loadAgents, loadConfig])

  // Poll notifications for background task completion
  useEffect(() => {
    if (notifIntervalRef.current) clearInterval(notifIntervalRef.current)
    notifIntervalRef.current = setInterval(async () => {
      try {
        const notes = await api.getNotifications()
        for (const n of notes) {
          if (n.kind === 'task_completed') {
            addToast(`Task completed: ${n.message}`, 'success')
          } else if (n.kind === 'task_failed') {
            addToast(`Task failed: ${n.message}`, 'error')
          } else if (n.kind === 'approval_created') {
            addToast(`🔴 ${n.message}`, 'info')
          } else {
            addToast(n.message, 'info')
          }
        }
      } catch { /* ignore polling errors */ }
    }, 5000)
    return () => {
      if (notifIntervalRef.current) clearInterval(notifIntervalRef.current)
    }
  }, [addToast])

  const refreshAll = useCallback(() => {
    loadProviders(); loadAgents(); loadConfig()
    if (selectedAgent) loadSessions(selectedAgent.id)
  }, [loadProviders, loadAgents, loadConfig, loadSessions, selectedAgent])

  const sidebarVisible = viewMode === 'normal'

  return (
    <div className={`flex h-screen font-inter ${viewMode === 'wide' ? 'app-wide' : ''} ${viewMode === 'simple' ? 'app-simple' : ''}`}>
      {/* View-only banner */}
      {config?.owner_id && (
        <div className="viewonly-banner">
          View-Only Mode — You are viewing {config.owner_id}'s console
        </div>
      )}

      {/* Sidebar */}
      {sidebarVisible && (
        <div className="w-72 min-w-72 bg-paper border-r border-mist flex flex-col">
          <div className="px-5 py-4 border-b border-mist flex items-center gap-2">
            <div className="w-7 h-7 bg-sage-teal rounded-lg flex items-center justify-center text-white text-body-sm font-semibold">S</div>
            <div>
              <h1 className="text-heading-sm font-semibold text-graphite tracking-tight">Smith</h1>
              <span className="text-caption text-slate -mt-0.5 block">Console</span>
            </div>
          </div>
          <div className="flex border-b border-mist">
            <div
              className={`flex-1 py-2.5 text-center cursor-pointer text-body-sm font-medium transition border-b-2 select-none ${
                tab === 'agents' ? 'text-sage-teal border-sage-teal' : 'text-slate hover:text-graphite border-transparent'
              }`}
              onClick={() => setTab('agents')}>Agents</div>
            <div
              className={`flex-1 py-2.5 text-center cursor-pointer text-body-sm font-medium transition border-b-2 select-none ${
                tab === 'providers' ? 'text-sage-teal border-sage-teal' : 'text-slate hover:text-graphite border-transparent'
              }`}
              onClick={() => setTab('providers')}>Providers</div>
          </div>
          <div className={`flex-1 overflow-y-auto px-3 py-3 ${tab === 'agents' ? 'block' : 'hidden'}`}>
            <AgentsPanel
              agents={agents}
              providers={providers}
              selectedAgent={selectedAgent}
              onSelect={selectAgent}
              onRefresh={loadAgents}
              addToast={addToast}
            />
          </div>
          <div className={`flex-1 overflow-y-auto px-3 py-3 ${tab === 'providers' ? 'block' : 'hidden'}`}>
            <ProvidersPanel
              providers={providers}
              onRefresh={loadProviders}
              addToast={addToast}
            />
          </div>
        </div>
      )}

      {/* Main area */}
      <div className="flex-1 flex flex-col min-w-0 bg-paper">
        <div className="px-5 py-3 border-b border-mist flex items-center justify-between bg-paper">
          <div className="flex items-center gap-3">
            {!sidebarVisible && (
              <button className="text-slate hover:text-graphite transition cursor-pointer bg-transparent border-none p-1 text-body-sm" onClick={() => handleViewModeChange('normal')} title="Show sidebar">
                ☰
              </button>
            )}
            <span id="active-agent-name" className="text-body font-medium text-graphite">
              {selectedAgent ? selectedAgent.name : 'Select an agent'}
            </span>
            {selectedAgent && (
              <span className="text-caption text-slate">
                {providers.find(p => p.id === selectedAgent.provider_id)?.label || selectedAgent.provider_id}
                {' · '}{selectedAgent.tool_count} tools
              </span>
            )}
          </div>
          <div className="flex items-center gap-1.5">
            {selectedAgent && (
              <SessionsPanel
                sessions={sessions}
                currentSessionId={currentSessionId}
                onSelectSession={(id) => {
                  setCurrentSessionId(id)
                  setMessages([])
                }}
                onNewSession={() => {
                  setCurrentSessionId(null)
                  setMessages([])
                }}
                agentId={selectedAgent.id}
                onSessionsChange={() => loadSessions(selectedAgent.id)}
              />
            )}
            <button className="text-slate hover:text-graphite transition cursor-pointer bg-transparent border-none p-1.5 text-body-sm rounded-lg hover:bg-veil" onClick={() => setShowSettings(true)} title="Settings">
              ⚙
            </button>
          </div>
        </div>

        {selectedAgent ? (
          <ChatArea
            key={selectedAgent.id}
            agentId={selectedAgent.id}
            sessionId={currentSessionId}
            messages={messages}
            onMessagesChange={setMessages}
            onSessionChange={(sid) => {
              setCurrentSessionId(sid)
              if (selectedAgent) loadSessions(selectedAgent.id)
            }}
            addToast={addToast}
          />
        ) : (
          <div className="flex-1 flex flex-col items-center justify-center text-center px-5">
            <div className="w-12 h-12 bg-veil rounded-lg flex items-center justify-center text-heading-sm text-sage-teal mb-4">S</div>
            <h3 className="text-body font-semibold text-graphite mb-2">Smith Console</h3>
            <p className="text-body-sm text-slate">Select an agent from the sidebar to start chatting.</p>
          </div>
        )}

        {/* Toasts */}
        <div className="toast-container">
          {toasts.map(t => (
            <div key={t.id} className={`toast toast-${t.type}`}>{t.msg}</div>
          ))}
        </div>

        {/* Settings modal */}
        {showSettings && (
          <SettingsPanel
            config={config}
            viewMode={viewMode}
            onViewModeChange={handleViewModeChange}
            onClose={() => setShowSettings(false)}
            addToast={addToast}
          />
        )}
      </div>
    </div>
  )
}
