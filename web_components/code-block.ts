import { LitElement, html, customElement } from 'lit-element';
import { unsafeHTML } from 'lit-html/directives/unsafe-html';

@customElement('code-block')
export class CodeBlock extends LitElement {
  // @property() name = 'World';

  render() {
    const code = this.shadowRoot!.host.innerHTML;
    const highlightedCode = (window as any).hljs.highlightAuto(code).value;
    return html`
    <style>
      :host {
      }
    </style>
      ${unsafeHTML(highlightedCode)}
    `;
  }
}
