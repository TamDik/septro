import { listen, Event } from '@tauri-apps/api/event'
import { UpdateContentPayload, CoreErrorPayload } from './types'
import { setupEditor } from './content/editor'


function addDynamicCSS(rawCSS: string) {
    const styleTag = document.createElement('style');
    styleTag.classList.add('dynamic-style');
    styleTag.innerHTML = rawCSS;
    document.head.appendChild(styleTag);
}

export function setupContent(element: HTMLDivElement) {
    listen('core-error', (event: Event<CoreErrorPayload>) => {
        const { message } = event.payload
        element.innerHTML = message
    });

    listen('update-content', (event: Event<UpdateContentPayload>) => {
        document.querySelectorAll('style.dynamic-style').forEach(style => style.remove());

        // body
        element.innerHTML = event.payload.body;

        // script
        for (const script of event.payload.scripts) {
            if (script == 'markdownEditor') {
                const css = setupEditor();
                addDynamicCSS(css);
            }
        }
    });
}
