# tauri-plugin-app-usage

Plugin **Tauri 2** para ler o **tempo de uso de aplicativos no Android** usando
`UsageStatsManager`. Funciona em uma única consulta nativa para todos os apps —
não faz uma chamada por pacote.

> ⚠️ Este plugin é **Android-only**. No desktop os comandos retornam um erro
> (`app usage statistics are only available on Android`).

## O que ele fornece

Para cada aplicativo com uso registrado:

- `appName` — nome legível (via `PackageManager`)
- `packageName` — identificador do pacote
- `iconBase64` — ícone PNG como data URI (`data:image/png;base64,...`)
- `totalTimeInForeground` — tempo total no período (ms)
- `todayTimeInForeground` — tempo de hoje (ms)
- `last7DaysTimeInForeground` — tempo dos últimos 7 dias (ms)
- `lastTimeUsed` — timestamp do último uso (ms)

E, na resposta da listagem, `totalTodayMs` com o total de uso do dia.

O resultado já vem ordenado pelo maior tempo (ranking de apps mais usados).

## Instalação

### Rust

```sh
cargo add tauri-plugin-app-usage
# ou, no Cargo.toml:
# tauri-plugin-app-usage = "0.1"
```

Registre o plugin no `src-tauri/src/lib.rs`:

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_app_usage::init())
    // ...
```

### JavaScript / TypeScript

```sh
pnpm add @reimaohenrique/plugin-app-usage-api
# npm install @reimaohenrique/plugin-app-usage-api
```

As permissões padrão do plugin (`allow-check-permission`,
`allow-request-permission`, `allow-get-all-app-usage`, `allow-get-app-usage`)
já vêm habilitadas pelo `permissions/default.toml`.

## Uso

```ts
import {
  checkPermission,
  requestPermission,
  getAllAppUsage,
  getAppUsage,
} from '@reimaohenrique/plugin-app-usage-api'

const granted = await checkPermission()

if (!granted) {
  // Abre Configurações → Acesso aos dados de uso.
  // A permissão NÃO é concedida automaticamente.
  await requestPermission()
}

const { apps, totalTodayMs } = await getAllAppUsage(7) // 1, 7 ou 30 dias
const instagram = await getAppUsage('com.instagram.android', 7)
```

Exemplo de resposta:

```json
{
  "totalTodayMs": 13680000,
  "apps": [
    {
      "packageName": "com.instagram.android",
      "appName": "Instagram",
      "iconBase64": "data:image/png;base64,...",
      "totalTimeInForeground": 36024000,
      "todayTimeInForeground": 5220000,
      "last7DaysTimeInForeground": 36024000,
      "lastTimeUsed": 1763650000000
    }
  ]
}
```

## Permissão (importante)

`android.permission.PACKAGE_USAGE_STATS` é uma **permissão protegida**. O
Android **nunca** mostra o diálogo de concessão, mesmo estando declarada no
manifesto do plugin (ela é mesclada no manifesto do app automaticamente).

O fluxo correto é:

1. `checkPermission()` → estado pendente;
2. `requestPermission()` → abre `Settings.ACTION_USAGE_ACCESS_SETTINGS`;
3. o usuário liga o toggle **Acesso aos dados de uso** manualmente;
4. ao voltar, chame `checkPermission()` novamente.

Nada é enviado para fora do dispositivo: ícones são convertidos localmente em
data URI e todos os dados ficam no processo do app.

## API Rust

Os mesmos comandos estão disponíveis via `AppHandle::app_usage()`:

```rust
use tauri_plugin_app_usage::AppUsageExt;

let plugin = app.app_usage();
let granted = plugin.check_permission()?;
let report = plugin.get_all_app_usage(7)?;
let one = plugin.get_app_usage("com.instagram.android", 7)?;
```

## Como funciona

- `queryUsageStats(INTERVAL_BEST, ...)` em uma única chamada, agregando por
  `packageName` com `getTotalTimeInForeground()`;
- `queryEvents(...)` preenche apps com uso recente ainda não consolidado;
- `AppOpsManager` verifica a permissão especial sem concedê-la;
- o Rust converte a resposta Kotlin em tipos serde tipados e expõe os comandos.

## Exemplo

Há um app de exemplo em [`examples/tauri-app`](./examples/tauri-app):

```sh
cd examples/tauri-app
pnpm install
pnpm tauri android dev
```

## Licença

[MIT](./LICENSE)
