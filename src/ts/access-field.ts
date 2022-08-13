import { invoke } from '@tauri-apps/api/tauri'
import { emit } from '@tauri-apps/api/event'


export function setupAccessField(element: HTMLInputElement) {
    element.addEventListener('keypress', async event => {
        if (event.key !== 'Enter') {
            return;
        }
        const url = element.value;
        const wikiLink = await invoke('parse_url', { url })
        emit('page-transition', { wikilink: wikiLink })
    })
}
