import '/src/css/reset.css'
import '/src/css/wiki.css'
import { setupAccessField } from './access-field'
import { setupContent } from './content'


setupAccessField(document.querySelector<HTMLInputElement>('#access-field')!)
setupContent(document.querySelector<HTMLDivElement>('#content')!)
