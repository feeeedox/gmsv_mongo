import mermaid from 'mermaid'

let initialized = false
let idCounter   = 0
let queue       = Promise.resolve()

function init() {
    if (initialized) return
    initialized = true
    mermaid.initialize({
        startOnLoad:   false,
        theme:         document.documentElement.classList.contains('dark') ? 'dark' : 'default',
        securityLevel: 'loose',
    })
}

export function renderMermaid(source: string): Promise<string> {
    const p = queue.then(async () => {
        init()
        const id     = `mermaid-r${++idCounter}`
        const result = await mermaid.render(id, source)

        document.getElementById(id)?.remove()
        document.getElementById(`d${id}`)?.remove()

        return result.svg
            .replace(/\s+width="[^"]*"/, '')
            .replace(/\s+height="[^"]*"/, '')
    })

    // @ts-ignore
    queue = p.catch(() => {})

    return p
}