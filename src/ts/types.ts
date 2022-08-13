export type WikiType = 'Page' | 'File' | 'Category' | 'Special'

export type WikiLink = {
    namespace: string,
    wiki_type: WikiType,
    name: string,
    fragment: string | null,
    queries: Map<string, string>,
}

export type CoreErrorPayload = {
    message: string,
}

export type UpdateContentPayload = {
    href: string,
    body: string,
    tabs: {
        "href": string,
        "title": string,
        "selected": boolean,
    }[]
}
