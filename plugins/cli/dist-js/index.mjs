import { invoke } from '@tauri-apps/api/tauri';

// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
/**
 * Parse the arguments provided to the current process and get the matches using the configuration defined [`tauri.cli`](https://tauri.app/v1/api/config/#tauriconfig.cli) in `tauri.conf.json`
 *
 * @example
 * ```typescript
 * import { getMatches } from 'tauri-plugin-cli-api';
 * const matches = await getMatches();
 * if (matches.subcommand?.name === 'run') {
 *   // `./your-app run $ARGS` was executed
 *   const args = matches.subcommand?.matches.args
 *   if ('debug' in args) {
 *     // `./your-app run --debug` was executed
 *   }
 * } else {
 *   const args = matches.args
 *   // `./your-app $ARGS` was executed
 * }
 * ```
 *
 * @since 1.0.0
 */
async function getMatches() {
    return await invoke('plugin:cli|cli_matches');
}

export { getMatches };
//# sourceMappingURL=index.mjs.map
