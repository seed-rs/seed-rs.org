import { LitElement, html, property, customElement } from 'lit-element';
import { unsafeHTML } from 'lit-html/directives/unsafe-html';

@customElement('code-block')
export class CodeBlock extends LitElement {
  @property() lang = '';

  render() {
    const code = decodeHtml(this.innerHTML);
    const highlightedCode = highlightCode(code, this.lang)

    return html`<pre><code>${unsafeHTML(highlightedCode)}</code></pre>`;
  }

  createRenderRoot() {
    return this;
  }
}

function decodeHtml(html: string): string {
  return new DOMParser()
    .parseFromString(html, "text/html")
    .documentElement
    .textContent!;
}

function highlightCode(code: string, lang: string): string {
  // https://highlightjs.readthedocs.io/en/latest/api.html#highlightauto-value-languagesubset
  return (window as any)
    .hljs
    .highlightAuto(code, lang ? [lang] : undefined)
    .value;
}
