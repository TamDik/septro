import { invoke } from '@tauri-apps/api/tauri'
import { emit, listen, Event } from '@tauri-apps/api/event'
import { UpdateContentPayload, WikiLink } from './types'


export function setupAccessField(element: HTMLInputElement) {
    element.addEventListener('keypress', async event => {
        if (event.key !== 'Enter') {
            return;
        }
        const url = element.value;
        const wikiLink = await invoke<WikiLink>('parse_url', { url })
        emit('page-transition', { wikilink: wikiLink })
    })

    listen('update-content', (event: Event<UpdateContentPayload>) => {
        element.value = decodeURI(event.payload.href)
    })
}
