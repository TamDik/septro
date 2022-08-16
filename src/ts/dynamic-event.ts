import { invoke } from '@tauri-apps/api/tauri'
import { emit } from '@tauri-apps/api/event'
import { WikiLink } from './types'


function addDynamicEventLister<K extends keyof HTMLElementTagNameMap>(
    type: string,
    tagName: K,
    listener: (event: Event, element: HTMLElementTagNameMap[K]) => boolean,
    options: boolean=false
): void {
    const upperTageName: string = tagName.toUpperCase();
    document.body.addEventListener(type, (event: Event) => {
        let element: HTMLElement | null = event.target as HTMLElement;
        while (element && element !== document.body) {
            if (element.nodeName === upperTageName) {
                if (listener(event, element as HTMLElementTagNameMap[K])) {
                    break;
                }
            }
            element = element.parentNode as HTMLElement;
        }
    }, options);
}


export default function setup() {
    addDynamicEventLister('click', 'a', (event, element) => {
        console.log(element.href);
        console.log(new URL(element.href));
        let url = element.dataset.wikilink;
        if (url) {
            invoke<WikiLink>('parse_url', { url })
            .then(wikiLink => {
                emit('page-transition', { wikilink: wikiLink })
            })
        }
        event.preventDefault();
        return true;
    });
}
