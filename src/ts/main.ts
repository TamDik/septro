import '/src/css/reset.css'
import '/src/css/wiki.css'
import { emit } from '@tauri-apps/api/event'
import { setupAccessField } from './access-field'
import { setupContent } from './content'
import { setupSideMenu, setupRightMenuTabs } from './navigation'
import setupDynamicEvents from './dynamic-event'


setupAccessField(document.querySelector<HTMLInputElement>('#access-field')!)
setupContent(document.querySelector<HTMLDivElement>('#content')!)
setupSideMenu(document.querySelector<HTMLDivElement>('#main-logo')!,
              document.querySelector<HTMLDivElement>('#wiki-side-menu')!)
setupRightMenuTabs(document.querySelector<HTMLDivElement>('#right-navigation .menu-tabs')!)
setupDynamicEvents()

emit('setup')
