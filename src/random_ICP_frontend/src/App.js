import { html, render } from 'lit-html';
import { random_ICP_backend } from 'declarations/random_ICP_backend';

class App {
  joke = '';

  constructor() {
    this.#render();
  }

  #handleSubmit = async (e) => {
    //Display an hourglass during the fetch
    document.body.style.cursor = 'wait';

    this.joke = await random_ICP_backend.random_joke();

    //Cancel the hourglass
    document.body.style.cursor = 'default';

    this.#render();
  };

  #render() {
    let body = html`
      <main>
        <form action="#">
          <button type="submit">Jakiś żart</button>
        </form>
        <section id="joke" style={{ textAlign: 'center' }}>
          ${this.joke}
        </section> 
      </main>
    `;
    render(body, document.getElementById('root'));
    document
      .querySelector('form')
      .addEventListener('submit', this.#handleSubmit);
  }
}

export default App;
