import { useState } from 'react'
import * as api from '../api'
import type { AgentSummary, AgentDefinition, ProviderConfig, ToolBinding, ScrollConfig } from '../types'

interface Props {
  agents: AgentSummary[]
  providers: ProviderConfig[]
  selectedAgent: AgentSummary | null
  onSelect: (agent: AgentSummary) => void
  onRefresh: () => void
  addToast: (msg: string, type?: 'error' | 'success') => void
}

const AVAILABLE_TOOLS = [
  { name: 'calculator', desc: 'Arithmetic' },
  { name: 'time', desc: 'Current time' },
  { name: 'shell', desc: 'Shell commands' },
]

const SCROLL_OPTIONS: { value: string; label: string }[] = [
  { value: JSON.stringify({ type: 'truncate', max_messages: 50 }), label: 'Truncate (50)' },
  { value: JSON.stringify({ type: 'truncate', max_messages: 100 }), label: 'Truncate (100)' },
  { value: JSON.stringify({ type: 'sliding_window', window_size: 20 }), label: 'Sliding Window (20)' },
  { value: JSON.stringify({ type: 'no_op' }), label: 'Keep All' },
]

// Track enabled state (client-side only for now)
const enabledAgents = new Set<string>()

export function AgentsPanel({ agents, providers, selectedAgent, onSelect, onRefresh, addToast }: Props) {
  const [showForm, setShowForm] = useState(false)
  const [editId, setEditId] = useState<string | null>(null)
  const [agentEnabled, setAgentEnabled] = useState<Record<string, boolean>>({})

  // Form fields
  const [name, setName] = useState('')
  const [description, setDescription] = useState('')
  const [providerId, setProviderId] = useState('')
  const [systemPrompt, setSystemPrompt] = useState('')
  const [temperature, setTemperature] = useState('')
  const [maxTokens, setMaxTokens] = useState('')
  const [modelOverride, setModelOverride] = useState('')
  const [selectedTools, setSelectedTools] = useState<Set<string>>(new Set())
  const [scrollStr, setScrollStr] = useState(SCROLL_OPTIONS[0].value)
  const [protectActiveTurn, setProtectActiveTurn] = useState(false)
  const [toolResultCap, setToolResultCap] = useState('')

  const openCreate = () => {
    setEditId(null)
    setName('')
    setDescription('')
    setProviderId(providers[0]?.id || '')
    setSystemPrompt('You are a helpful assistant.')
    setTemperature('')
    setMaxTokens('')
    setModelOverride('')
    setSelectedTools(new Set(['calculator', 'time']))
    setScrollStr(SCROLL_OPTIONS[0].value)
    setProtectActiveTurn(false)
    setToolResultCap('')
    setShowForm(true)
  }

  const openEdit = async (agentId: string) => {
    try {
      const a: AgentDefinition = await api.getAgent(agentId)
      setEditId(a.id)
      setName(a.name)
      setDescription(a.description || '')
      setProviderId(a.provider_id)
      setSystemPrompt(a.system_prompt)
      setTemperature(a.temperature != null ? String(a.temperature) : '')
      setMaxTokens(a.max_tokens != null ? String(a.max_tokens) : '')
      setModelOverride('')
      setSelectedTools(new Set(
        a.tools.filter((t): t is ToolBinding & { type: 'builtin' } => t.type === 'builtin').map(t => t.name)
      ))
      setScrollStr(JSON.stringify(a.scroll_strategy))
      setProtectActiveTurn(a.protect_active_turn ?? false)
      setToolResultCap(a.tool_result_cap != null ? String(a.tool_result_cap) : '')
      setShowForm(true)
    } catch (e: any) {
      addToast(e.message)
    }
  }

  const toggleEnabled = (agentId: string) => {
    setAgentEnabled(prev => {
      const next = { ...prev }
      next[agentId] = !prev[agentId]
      if (!next[agentId]) delete next[agentId]
      return next
    })
    addToast('Agent state toggled', 'success')
  }

  const toggleTool = (tool: string) => {
    setSelectedTools(prev => {
      const next = new Set(prev)
      if (next.has(tool)) next.delete(tool)
      else next.add(tool)
      return next
    })
  }

  const save = async () => {
    const tools: ToolBinding[] = Array.from(selectedTools).map(name => ({
      type: 'builtin' as const,
      name,
      enabled: true,
    }))
    let scrollStrategy: ScrollConfig
    try { scrollStrategy = JSON.parse(scrollStr) }
    catch { scrollStrategy = { type: 'truncate', max_messages: 50 } }

    const body: Partial<AgentDefinition> = {
      name,
      description: description || null,
      provider_id: providerId,
      system_prompt: systemPrompt,
      temperature: temperature ? parseFloat(temperature) : null,
      max_tokens: maxTokens ? parseInt(maxTokens) : null,
      tools,
      scroll_strategy: scrollStrategy,
      protect_active_turn: protectActiveTurn,
      tool_result_cap: toolResultCap ? parseInt(toolResultCap) : null,
    }

    try {
      if (editId) {
        await api.updateAgent(editId, body)
      } else {
        await api.createAgent(body)
      }
      setShowForm(false)
      onRefresh()
      addToast('Agent saved', 'success')
    } catch (e: any) {
      addToast(e.message)
    }
  }

  const remove = async (id: string) => {
    if (!confirm('Delete this agent?')) return
    try {
      await api.deleteAgent(id)
      onRefresh()
      addToast('Agent deleted', 'success')
    } catch (e: any) {
      addToast(e.message)
    }
  }

  return (
    <>
      <button className="bg-sage-teal text-white rounded-lg px-4 py-2 font-inter text-body-sm font-medium cursor-pointer transition hover:opacity-90 w-full mb-2 border-none" onClick={openCreate}>
        + New Agent
      </button>

      {agents.length === 0 ? (
        <div className="text-center px-5 py-10 text-body-sm text-slate"><p>No agents yet.<br />Create one to get started.</p></div>
      ) : (
        agents.map(a => (
          <div key={a.id}
            className={`bg-paper rounded-lg shadow-sm border px-3 py-2.5 mb-1.5 cursor-pointer transition ${
              selectedAgent?.id === a.id ? 'border-sage-teal bg-[#f0faf8]' : 'border-cloud hover:border-sage-teal'
            }`}
            onClick={() => onSelect(a)}
            onDoubleClick={() => openEdit(a.id)}
          >
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-1.5">
                <label className="cursor-pointer flex items-center"
                  onClick={e => { e.stopPropagation(); toggleEnabled(a.id) }}>
                  <input type="checkbox" checked={agentEnabled[a.id] !== false} readOnly
                    className="cursor-pointer accent-sage-teal" />
                </label>
                <h3 className="text-body-sm font-semibold text-graphite">{a.name}</h3>
              </div>
              <div className="flex gap-1">
                <button className="text-slate hover:text-graphite cursor-pointer bg-transparent border-none p-1 text-caption rounded hover:bg-veil transition"
                  onClick={e => { e.stopPropagation(); openEdit(a.id) }}
                  title="Edit">✎</button>
                <button className="text-red hover:opacity-80 cursor-pointer bg-transparent border-none p-1 text-caption rounded hover:bg-veil transition"
                  onClick={e => { e.stopPropagation(); remove(a.id) }}>✕</button>
              </div>
            </div>
            <p className="text-caption text-slate mt-1 truncate">{a.system_prompt}</p>
            <div className="text-caption text-fog mt-1">{providers.find(p => p.id === a.provider_id)?.label || a.provider_id} · {a.tool_count} tools</div>
            {!agentEnabled[a.id] && agentEnabled[a.id] !== undefined && (
              <span className="text-caption text-red">Disabled</span>
            )}
          </div>
        ))
      )}

      {/* Agent form modal */}
      <div className={`fixed inset-0 bg-black/65 z-50 flex items-center justify-center ${showForm ? '' : 'hidden'}`}>
        <div className="bg-paper rounded-lg shadow-md px-6 py-6 w-[90%] max-w-[560px] max-h-[85vh] overflow-y-auto">
          <h2 className="text-subheading font-semibold text-graphite mb-4">{editId ? 'Edit Agent' : 'New Agent'}</h2>

          <div className="mb-3">
            <label className="block text-caption text-slate mb-1 font-medium">Name</label>
            <input value={name} onChange={e => setName(e.target.value)} placeholder="My Assistant"
              className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal" />
          </div>

          <div className="mb-3">
            <label className="block text-caption text-slate mb-1 font-medium">Description</label>
            <input value={description} onChange={e => setDescription(e.target.value)} placeholder="Optional description"
              className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal" />
          </div>

          <div className="mb-3">
            <label className="block text-caption text-slate mb-1 font-medium">Provider</label>
            {providers.length === 0 ? (
              <div className="text-caption text-red py-2">
                No providers configured. Create one in the Providers tab first.
              </div>
            ) : (
              <select value={providerId} onChange={e => setProviderId(e.target.value)}
                className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal">
                {providers.map(p => (
                  <option key={p.id} value={p.id}>{p.label} ({p.kind})</option>
                ))}
              </select>
            )}
          </div>

          <div className="mb-3">
            <label className="block text-caption text-slate mb-1 font-medium">System Prompt</label>
            <textarea value={systemPrompt} onChange={e => setSystemPrompt(e.target.value)}
              placeholder="You are a helpful assistant."
              className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal min-h-[80px] resize-y"
            />
          </div>

          <div className="flex gap-2 mb-3">
            <div className="flex-1">
              <label className="block text-caption text-slate mb-1 font-medium">Temperature</label>
              <input value={temperature} onChange={e => setTemperature(e.target.value)}
                type="number" step="0.1" placeholder="Default"
                className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal" />
            </div>
            <div className="flex-1">
              <label className="block text-caption text-slate mb-1 font-medium">Max Tokens</label>
              <input value={maxTokens} onChange={e => setMaxTokens(e.target.value)}
                type="number" placeholder="Default"
                className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal" />
            </div>
          </div>

          <div className="mb-3">
            <label className="block text-caption text-slate mb-1 font-medium">Model Override <span className="font-normal text-fog">(optional)</span></label>
            <input value={modelOverride} onChange={e => setModelOverride(e.target.value)}
              placeholder="Leave empty to use provider default model"
              className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal" />
          </div>

          <div className="mb-3">
            <label className="flex items-center gap-1.5 cursor-pointer text-caption text-slate font-medium mb-1">
              <input type="checkbox" checked={protectActiveTurn}
                onChange={e => setProtectActiveTurn(e.target.checked)}
                className="accent-sage-teal" />
              Protect Active Turn
            </label>
            <div className="text-caption text-fog mt-0.5">
              Pins the most recent user message and prevents it from being evicted during scroll.
            </div>
          </div>

          <div className="mb-3">
            <label className="block text-caption text-slate mb-1 font-medium">Tool Result Cap (bytes) <span className="font-normal text-fog">(empty = no cap)</span></label>
            <input value={toolResultCap} onChange={e => setToolResultCap(e.target.value)}
              type="number" min="0" step="1024"
              placeholder="e.g. 4096 for 4 KiB cap"
              className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal" />
            <div className="text-caption text-fog mt-0.5">
              Large tool results exceeding this limit are stored in episodic memory and replaced with a recall stub.
            </div>
          </div>

          <div className="mb-3">
            <label className="block text-caption text-slate mb-1 font-medium">Tools</label>
            <div className="flex flex-wrap gap-1 mt-1">
              {AVAILABLE_TOOLS.map(t => (
                <span key={t.name}
                  className={`inline-flex items-center gap-1 px-2 py-0.5 rounded-lg text-caption cursor-pointer transition border ${
                    selectedTools.has(t.name) ? 'bg-sage-teal text-white border-sage-teal' : 'bg-paper text-slate border-mist hover:border-sage-teal'
                  }`}
                  onClick={() => toggleTool(t.name)}
                  title={t.desc}
                >{t.name}</span>
              ))}
            </div>
          </div>

          <div className="mb-3">
            <label className="block text-caption text-slate mb-1 font-medium">Scroll Strategy</label>
            <select value={scrollStr} onChange={e => setScrollStr(e.target.value)}
              className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal">
              {SCROLL_OPTIONS.map((o, i) => (
                <option key={i} value={o.value}>{o.label}</option>
              ))}
            </select>
          </div>

          <div className="flex justify-end gap-2 mt-4">
            <button className="bg-paper border border-mist text-graphite rounded-lg px-4 py-2 font-inter text-body-sm font-medium cursor-pointer transition hover:border-sage-teal" onClick={() => setShowForm(false)}>Cancel</button>
            <button className="bg-sage-teal text-white rounded-lg px-4 py-2 font-inter text-body-sm font-medium cursor-pointer transition hover:opacity-90 disabled:opacity-50 disabled:cursor-not-allowed border-none" onClick={save} disabled={providers.length === 0}>Save</button>
          </div>
        </div>
      </div>
    </>
  )
}
