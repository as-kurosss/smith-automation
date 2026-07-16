import { useState } from 'react'
import * as api from '../api'
import type { ProviderConfig, ProviderKind } from '../types'

interface Props {
  providers: ProviderConfig[]
  onRefresh: () => void
  addToast: (msg: string, type?: 'error' | 'success') => void
}

const PROVIDER_KINDS: { value: ProviderKind; label: string; defaultUrl: string }[] = [
  { value: 'openai', label: 'OpenAI', defaultUrl: 'https://api.openai.com/v1' },
  { value: 'anthropic', label: 'Anthropic', defaultUrl: 'https://api.anthropic.com/v1' },
  { value: 'gemini', label: 'Gemini', defaultUrl: 'https://generativelanguage.googleapis.com/v1beta' },
  { value: 'ollama', label: 'Ollama', defaultUrl: 'http://localhost:11434/v1' },
  { value: 'custom', label: 'Custom (OpenAI-compatible)', defaultUrl: 'https://' },
  { value: 'lm_studio', label: 'LM Studio', defaultUrl: 'http://localhost:1234/v1' },
]

const DEFAULT_MODELS: Record<ProviderKind, string> = {
  openai: 'gpt-4o',
  anthropic: 'claude-3-5-sonnet-20241022',
  gemini: 'gemini-2.0-flash',
  ollama: 'llama3.2',
  custom: 'gpt-4o-mini',
  lm_studio: 'local-model',
}

export function ProvidersPanel({ providers, onRefresh, addToast }: Props) {
  const [editing, setEditing] = useState<ProviderConfig | null>(null)
  const [showForm, setShowForm] = useState(false)

  const [kind, setKind] = useState<ProviderKind>('openai')
  const [label, setLabel] = useState('')
  const [model, setModel] = useState('')
  const [apiKey, setApiKey] = useState('')
  const [apiUrl, setApiUrl] = useState('')

  const openCreate = () => {
    setEditing(null)
    setKind('openai')
    setLabel('')
    setModel('gpt-4o')
    setApiKey('')
    setApiUrl('https://api.openai.com/v1')
    setShowForm(true)
  }

  const openEdit = (p: ProviderConfig) => {
    setEditing(p)
    setKind(p.kind)
    setLabel(p.label)
    setModel(p.model)
    setApiKey(p.api_key)
    setApiUrl(p.api_url || '')
    setShowForm(true)
  }

  const handleKindChange = (k: ProviderKind) => {
    setKind(k)
    setModel(DEFAULT_MODELS[k])
    setApiUrl(PROVIDER_KINDS.find(x => x.value === k)?.defaultUrl || '')
  }

  const save = async () => {
    const body = { kind, label, model, api_key: apiKey, api_url: apiUrl || null }
    try {
      if (editing) {
        await api.updateProvider(editing.id, body)
      } else {
        await api.createProvider(body)
      }
      setShowForm(false)
      onRefresh()
      addToast('Provider saved', 'success')
    } catch (e: any) {
      addToast(e.message)
    }
  }

  const remove = async (id: string) => {
    if (!confirm('Delete this provider?')) return
    try {
      await api.deleteProvider(id)
      onRefresh()
      addToast('Provider deleted', 'success')
    } catch (e: any) {
      addToast(e.message)
    }
  }

  return (
    <>
      <button className="bg-sage-teal text-white rounded-lg px-4 py-2 font-inter text-body-sm font-medium cursor-pointer transition hover:opacity-90 w-full mb-2 border-none" onClick={openCreate}>
        + New Provider
      </button>
      {providers.length === 0 ? (
        <div className="text-center px-5 py-10 text-body-sm text-slate"><p>No providers configured yet.</p></div>
      ) : (
        providers.map(p => (
          <div className="bg-paper rounded-lg shadow-sm border border-cloud px-3 py-2.5 mb-1.5 cursor-pointer transition hover:border-sage-teal" key={p.id} onClick={() => openEdit(p)}>
            <div className="flex items-center justify-between">
              <div>
                <h3 className="text-body-sm font-semibold text-graphite inline">{p.label}</h3>
                <span className={`inline-block px-1.5 py-0.5 rounded text-caption font-semibold ml-1 text-white ${
                  p.kind === 'openai' ? 'bg-[#10a37f]' : p.kind === 'anthropic' ? 'bg-[#d97706]' : p.kind === 'gemini' ? 'bg-[#4285f4]' : 'bg-[#7a5ac2]'
                }`}>{p.kind}</span>
              </div>
              <button className="text-red hover:opacity-80 cursor-pointer bg-transparent border-none p-1 text-caption rounded hover:bg-veil transition" onClick={e => { e.stopPropagation(); remove(p.id) }}>✕</button>
            </div>
            <p className="text-caption text-slate mt-1">{p.model}</p>
            <div className="text-caption text-fog mt-0.5">{p.id}</div>
          </div>
        ))
      )}

      {/* Provider form modal */}
      <div className={`fixed inset-0 bg-black/65 z-50 flex items-center justify-center ${showForm ? '' : 'hidden'}`}>
        <div className="bg-paper rounded-lg shadow-md px-6 py-6 w-[90%] max-w-[560px] max-h-[85vh] overflow-y-auto">
          <h2 className="text-subheading font-semibold text-graphite mb-4">{editing ? 'Edit Provider' : 'New Provider'}</h2>
          <div className="mb-3">
            <label className="block text-caption text-slate mb-1 font-medium">Label</label>
            <input value={label} onChange={e => setLabel(e.target.value)} placeholder="My OpenAI Key"
              className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal" />
          </div>
          <div className="flex gap-2 mb-3">
            <div className="flex-1">
              <label className="block text-caption text-slate mb-1 font-medium">Provider</label>
              <select value={kind} onChange={e => handleKindChange(e.target.value as ProviderKind)}
                className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal">
                {PROVIDER_KINDS.map(k => <option key={k.value} value={k.value}>{k.label}</option>)}
              </select>
            </div>
            <div className="flex-1">
              <label className="block text-caption text-slate mb-1 font-medium">Model</label>
              <input value={model} onChange={e => setModel(e.target.value)} placeholder="gpt-4o"
                className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal" />
            </div>
          </div>
          <div className="mb-3">
            <label className="block text-caption text-slate mb-1 font-medium">API Key {kind === 'ollama' ? <span className="font-normal text-fog">(optional for local)</span> : ''}</label>
            <input value={apiKey} onChange={e => setApiKey(e.target.value)} type="password" placeholder={kind === 'ollama' ? 'Leave empty for local' : 'sk-...'}
              className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal" />
          </div>
          <div className="mb-3">
            <label className="block text-caption text-slate mb-1 font-medium">API URL</label>
            <input value={apiUrl} onChange={e => setApiUrl(e.target.value)} placeholder={PROVIDER_KINDS.find(k => k.value === kind)?.defaultUrl || 'https://...'}
              className="w-full px-2.5 py-2 rounded-lg border border-mist bg-paper text-body-sm text-graphite outline-none focus:border-sage-teal" />
          </div>
          <div className="flex justify-end gap-2 mt-4">
            <button className="bg-paper border border-mist text-graphite rounded-lg px-4 py-2 font-inter text-body-sm font-medium cursor-pointer transition hover:border-sage-teal" onClick={() => setShowForm(false)}>Cancel</button>
            <button className="bg-sage-teal text-white rounded-lg px-4 py-2 font-inter text-body-sm font-medium cursor-pointer transition hover:opacity-90 border-none" onClick={save}>Save</button>
          </div>
        </div>
      </div>
    </>
  )
}
