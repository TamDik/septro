import { listen, Event } from '@tauri-apps/api/event'
import { UpdateContentPayload, CoreErrorPayload } from './types'


export function setupContent(element: HTMLDivElement) {
    listen('core-error', (event: Event<CoreErrorPayload>) => {
        const { message } = event.payload
        element.innerHTML = message
    })

    listen('update-content', (event: Event<UpdateContentPayload>) => {
        const { body } = event.payload
        element.innerHTML = body
    })
}
