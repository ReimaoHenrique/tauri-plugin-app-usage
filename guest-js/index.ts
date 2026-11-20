import { invoke } from '@tauri-apps/api/core'

/** Uso de um aplicativo em primeiro plano. Tempos em milissegundos. */
export interface AppUsage {
  packageName: string
  appName: string
  /** Data URI PNG (`data:image/png;base64,...`) ou `null` quando indisponível. */
  iconBase64: string | null
  totalTimeInForeground: number
  todayTimeInForeground: number
  last7DaysTimeInForeground: number
  lastTimeUsed: number
}

/** Ranking de aplicativos e total de tempo de uso no dia. */
export interface UsageReport {
  apps: AppUsage[]
  totalTodayMs: number
}

/** Verifica se o usuário concedeu "Acesso aos dados de uso". */
export async function checkPermission(): Promise<boolean> {
  return await invoke<boolean>('plugin:app-usage|check_permission')
}

/**
 * Abre as configurações de Acesso aos dados de uso.
 * A permissão é sempre concedida manualmente pelo usuário.
 */
export async function requestPermission(): Promise<boolean> {
  return await invoke<boolean>('plugin:app-usage|request_permission')
}

/** Consulta todos os apps com uso registrado, ordenados por tempo (ranking). */
export async function getAllAppUsage(days = 7): Promise<UsageReport> {
  return await invoke<UsageReport>('plugin:app-usage|get_all_app_usage', { days })
}

/** Consulta um único app pelo package name. */
export async function getAppUsage(packageName: string, days = 7): Promise<AppUsage | null> {
  return await invoke<AppUsage | null>('plugin:app-usage|get_app_usage', {
    packageName,
    days,
  })
}
