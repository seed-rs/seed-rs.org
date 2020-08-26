import { LitElement, html, property, customElement } from 'lit-element';
import { unsafeHTML } from 'lit-html/directives/unsafe-html';

@customElement('code-block')
export class CodeBlock extends LitElement {
  @property() lang = '';
  @property() code = '';

  render() {
    const highlightedCode = highlightCode(this.code, this.lang)

    return html`<pre><code>${unsafeHTML(highlightedCode)}</code></pre>`;
  }

  createRenderRoot() {
    return this;
  }
}

function highlightCode(code: string, lang: string): string {
  // https://highlightjs.readthedocs.io/en/latest/api.html#highlightauto-value-languagesubset
  return (window as any)
    .hljs
    .highlightAuto(code, lang ? [lang] : undefined)
    .value;
}
