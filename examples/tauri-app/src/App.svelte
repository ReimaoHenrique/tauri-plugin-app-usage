<script>
  import {
    checkPermission,
    requestPermission,
    getAllAppUsage,
  } from '@reimaohenrique/plugin-app-usage-api'

  let granted = $state(false)
  let report = $state({ apps: [], totalTodayMs: 0 })
  let error = $state('')

  function formatTime(ms) {
    const minutes = Math.round(ms / 60000)
    const h = Math.floor(minutes / 60)
    const m = minutes % 60
    if (h > 0 && m > 0) return `${h}h ${m}min`
    if (h > 0) return `${h}h`
    return `${m}min`
  }

  async function refresh() {
    try {
      error = ''
      granted = await checkPermission()
      report = granted ? await getAllAppUsage(7) : { apps: [], totalTodayMs: 0 }
    } catch (e) {
      error = String(e)
    }
  }

  refresh()
</script>

<main class="container">
  <h1>Uso de aplicativos</h1>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  {#if !granted && !error}
    <p>Permissão pendente: ative o acesso aos dados de uso no Android.</p>
    <button onclick={() => requestPermission()}>Abrir configurações</button>
    <button onclick={refresh}>Já concedi, verificar</button>
  {:else if granted}
    <h2>Total hoje: {formatTime(report.totalTodayMs)}</h2>
    <ol>
      {#each report.apps as app (app.packageName)}
        <li>
          {#if app.iconBase64}
            <img src={app.iconBase64} alt="" width="32" height="32" />
          {/if}
          <div>
            <strong>{app.appName}</strong>
            <small>{app.packageName}</small>
          </div>
          <span>{formatTime(app.totalTimeInForeground)}</span>
        </li>
      {/each}
    </ol>
  {/if}
</main>

<style>
  .container { max-width: 640px; margin: 2rem auto; font-family: sans-serif; }
  .error { color: #ff6b6b; }
  ol { list-style: none; padding: 0; display: grid; gap: 8px; }
  li { display: flex; align-items: center; gap: 12px; background: #f4f4f4; padding: 8px; border-radius: 8px; }
  li div { flex: 1; display: grid; }
  small { opacity: 0.6; }
</style>
