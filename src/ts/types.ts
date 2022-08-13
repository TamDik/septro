export type WikiType = 'Page' | 'File' | 'Category' | 'Special'

export type WikiLink = {
    namespace: string,
    wiki_type: WikiType,
    name: string,
}

export type CoreErrorPayload = {
    message: string,
}

export type UpdateContentPayload = {
    body: string,
}
