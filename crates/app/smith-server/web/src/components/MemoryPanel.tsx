import { useState, useEffect } from 'react'
import * as api from '../api'
import type { MemorySearchResult, MemoryStats } from '../types'

interface Props {
  addToast: (msg: string, type?: 'error' | 'success') => void
}

export function MemoryPanel({ addToast }: Props) {
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<MemorySearchResult[]>([])
  const [searched, setSearched] = useState(false)
  const [searching, setSearching] = useState(false)
  const [stats, setStats] = useState<MemoryStats | null>(null)
  const [loadingStats, setLoadingStats] = useState(true)

  useEffect(() => {
    api.getMemoryStats().then(setStats).catch(() => {}).finally(() => setLoadingStats(false))
  }, [])

  const doSearch = async () => {
    if (!query.trim()) return
    setSearching(true)
    setSearched(true)
    try {
      setResults(await api.searchMemoryEpisodic(query.trim()))
    } catch (e: any) { addToast(e.message) }
    finally { setSearching(false) }
  }

  return (
    <div>
      {/* Memory Stats */}
      <h3 style={{ fontSize: 13, fontWeight: 600, marginBottom: 8, color: 'var(--accent)' }}>
        Episodic Memory
      </h3>
      {!loadingStats && stats && (
        <div className="card" style={{ padding: '8px 10px', marginBottom: 12, cursor: 'default' }}>
          <div className="flex-between">
            <span style={{ fontSize: 13 }}>Status</span>
            <span style={{ fontSize: 12, color: stats.has_episodic_memory ? 'var(--green, #4caf50)' : 'var(--text2)' }}>
              {stats.has_episodic_memory ? 'Active' : 'Disabled'}
            </span>
          </div>
          <div className="flex-between" style={{ marginTop: 4 }}>
            <span style={{ fontSize: 13 }}>Stored Turns</span>
            <span style={{ fontSize: 12 }}>{stats.total_entries}</span>
          </div>
        </div>
      )}

      {!loadingStats && stats && !stats.has_episodic_memory && (
        <div className="empty-state" style={{ marginBottom: 12 }}>
          <p>Episodic memory is disabled. Enable it in Server Settings to record and search past agent turns.</p>
        </div>
      )}

      {/* Memory Search */}
      <h3 style={{ fontSize: 13, fontWeight: 600, marginBottom: 8, color: 'var(--accent)' }}>
        Search
      </h3>

      <div style={{ display: 'flex', gap: 4, marginBottom: 12 }}>
        <input value={query} onChange={e => setQuery(e.target.value)}
          onKeyDown={e => e.key === 'Enter' && doSearch()}
          placeholder="Search episodic memory..."
          style={{ flex: 1, padding: '8px 10px', borderRadius: 'var(--radius)',
            border: '1px solid var(--border)', background: 'var(--bg)', color: 'var(--text)', fontSize: 13 }} />
        <button className="btn btn-primary" onClick={doSearch} disabled={!query.trim() || searching}>
          {searching ? '...' : 'Search'}
        </button>
      </div>

      {searched && results.length === 0 && (
        <div className="empty-state"><p>No memories found.</p></div>
      )}

      {results.map(r => (
        <div key={r.turn_id} className="card">
          <p><strong>Input:</strong> {r.input}</p>
          {r.output && <p style={{ marginTop: 4 }}><strong>Output:</strong> {r.output}</p>}
          <small style={{ display: 'block', marginTop: 4, fontSize: 10, color: 'var(--text2)' }}>
            {r.turn_id} · {r.timestamp}
          </small>
        </div>
      ))}
    </div>
  )
}
