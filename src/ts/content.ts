import { invoke } from '@tauri-apps/api/tauri'
import { listen, Event } from '@tauri-apps/api/event'
import { WikiLink } from './types'


export function setupContent(element: HTMLDivElement) {
    async function updateMainContent() {
        element.innerHTML = await invoke('main_content')
    }

    updateMainContent()

    listen('page-transition', (event: Event<{ wikiLink: WikiLink }>) => {
        updateMainContent()
    })
}
