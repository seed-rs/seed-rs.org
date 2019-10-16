import { LitElement, html, customElement } from 'lit-element';

@customElement('code-block')
export class CodeBlock extends LitElement {
  // @property() name = 'World';

  render() {
    console.log("sefsef");

    // return html`<p>Hello, ${this.name}!</p>`;
    return html`<p>Hello!</p>`;
  }
}
