import defaultIcon from '/src/images/default-icon.png'
import { listen, Event } from '@tauri-apps/api/event'
import { UpdateContentPayload } from './types'


export function setupSideMenu(mainLogo: HTMLDivElement, _sideMenu: HTMLDivElement) {
    mainLogo.innerHTML = `
      <a href="#" class="internal">
        <img src="${defaultIcon}">
      </a>
    `
}


export function setupRightMenuTabs(element: HTMLDivElement) {
    // TODO: partial updates
    listen('update-content', (event: Event<UpdateContentPayload>) => {
        let { tabs } = event.payload;
        let tabsHTML = '';
        for (const { title, selected, href } of tabs) {
            if (selected) {
                tabsHTML += `<li class="selected"><a href="#" class="internal" data-wikilink="${href}">${title}</a></li>`
            } else {
                tabsHTML += `<li><a href="#" class="internal" data-wikilink="${href}">${title}</a></li>`
            }
        }
        element.innerHTML = `
          <ul>
            ${tabsHTML}
            <li id="search-tab"><input type="search" placeholder="Search" id="search-field"></li>
          </ul>
        `
    })
}
