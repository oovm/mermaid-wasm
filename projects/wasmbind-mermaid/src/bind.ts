import mermaid from "mermaid";
import mermaidAPI from "mermaid/mermaidAPI";

export function draw_mermaid(e: Element, input: string, cfg: mermaidAPI.Config): string {
    //const cb = (svg) => { console.log(svg); };
    const cb = (_: any) => { };
    return mermaid.render('id1', input, cb)
}
