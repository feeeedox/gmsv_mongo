const PLANTUML_ALPHABET =
    '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_'

function encode6bit(b: number): string {
    // @ts-ignore
    return PLANTUML_ALPHABET[b & 0x3f]
}

function append3bytes(b1: number, b2: number, b3: number): string {
    const c1 = b1 >> 2
    const c2 = ((b1 & 0x3) << 4) | (b2 >> 4)
    const c3 = ((b2 & 0xf) << 2) | (b3 >> 6)
    const c4 = b3 & 0x3f
    return encode6bit(c1) + encode6bit(c2) + encode6bit(c3) + encode6bit(c4)
}

function encodePlantumlBytes(data: Uint8Array): string {
    let result = ''
    let i = 0
    while (i + 2 < data.length) {
        // @ts-ignore
        result += append3bytes(data[i], data[i + 1], data[i + 2])
        i += 3
    }
    if (i + 1 === data.length) {
        // @ts-ignore
        result += append3bytes(data[i], 0, 0).slice(0, 2)
    } else if (i + 2 === data.length) {
        // @ts-ignore
        result += append3bytes(data[i], data[i + 1], 0).slice(0, 3)
    }
    return result
}

async function encodePlantuml(source: string): Promise<string> {
    const utf8 = new TextEncoder().encode(source)

    const cs = new CompressionStream('deflate-raw')
    const writer = cs.writable.getWriter()
    writer.write(utf8)
    writer.close()

    const chunks: Uint8Array[] = []
    const reader = cs.readable.getReader()
    while (true) {
        const { done, value } = await reader.read()
        if (done) break
        chunks.push(value)
    }

    const total = chunks.reduce((n, c) => n + c.length, 0)
    const merged = new Uint8Array(total)
    let offset = 0
    for (const chunk of chunks) {
        merged.set(chunk, offset)
        offset += chunk.length
    }

    return encodePlantumlBytes(merged)
}

const SERVER =
    (import.meta.env?.VITE_PLANTUML_SERVER as string | undefined) ??
    'https://www.plantuml.com/plantuml'


export async function plantumlSvgUrl(source: string): Promise<string> {
    const wrapped =
        source.trimStart().startsWith('@')
            ? source
            : `@startuml\n${source}\n@enduml`

    const encoded = await encodePlantuml(wrapped)
    return `${SERVER}/svg/${encoded}`
}

export async function renderPlantuml(source: string): Promise<string> {
    const url = await plantumlSvgUrl(source)
    const res = await fetch(url)
    if (!res.ok) throw new Error(`PlantUML server error ${res.status}`)

    const svg = await res.text()

    if (svg.includes('erbasieerror') || svg.includes('Syntax Error')) {
        throw new Error('PlantUML syntax error — check your diagram source')
    }

    return svg
        .replace(/\s+width="[^"]*"/, '')
        .replace(/\s+height="[^"]*"/, '')
}