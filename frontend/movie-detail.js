// movie-detail.js
class MovieDetail extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

    static get observedAttributes() {
        return ['movie-data'];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === 'movie-data') {
            this.render(JSON.parse(newValue));
        }
    }

    render(movie) {
        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                    font-family: 'Arial', sans-serif;
                    background: white;
                    border-radius: 8px;
                    padding: 20px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    margin: 10px;
                }
                
                h2 {
                    color: #2c3e50;
                    margin: 0 0 15px 0;
                }
                
                .stars {
                    color: #f1c40f;
                    font-size: 24px;
                    margin: 10px 0;
                }
                
                .id {
                    color: #7f8c8d;
                    font-size: 0.9em;
                }
                
                .delete-btn {
                    background: #e74c3c;
                    color: white;
                    border: none;
                    padding: 8px 16px;
                    border-radius: 4px;
                    cursor: pointer;
                    margin-top: 10px;
                    transition: background 0.3s;
                }
                
                .delete-btn:hover {
                    background: #c0392b;
                }
            </style>
            
            <div class="movie-detail">
                <h2>${movie.name}</h2>
                <div class="stars">${'★'.repeat(movie.stars)}${'☆'.repeat(5-movie.stars)}</div>
                <div class="id">ID: ${movie.id}</div>
                <button class="delete-btn" onclick="this.getRootNode().host.deleteMovie(${movie.id})">
                    Delete Movie
                </button>
            </div>
        `;
    }

    deleteMovie(id) {
        fetch(`http://localhost:8080/movies/delete?id=${id}`, {
            method: 'DELETE'
        })
            .then(response => {
                if (response.ok) {
                    this.dispatchEvent(new CustomEvent('movieDeleted', {
                        bubbles: true,
                        composed: true,
                        detail: { id }
                    }));
                }
            })
            .catch(error => console.error('Error:', error));
    }
}

customElements.define('movie-detail', MovieDetail);