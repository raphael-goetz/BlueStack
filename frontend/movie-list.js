class MovieList extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({mode: 'open'});
        this.movies = [];
    }

    async connectedCallback() {
        this.render();
        await this.fetchMovies();

        // Attach event listener after rendering
        const form = this.shadowRoot.querySelector('form');
        form.addEventListener('submit', (event) => this.addMovie(event));

        // Listen for movie deletions
        this.addEventListener('movieDeleted', (e) => {
            this.movies = this.movies.filter(movie => movie.id !== e.detail.id);
            this.render();
        });
    }

    async fetchMovies() {
        try {
            const response = await fetch('http://localhost:8080/movies');
            this.movies = await response.json();
            this.render();
        } catch (error) {
            console.error('Error fetching movies:', error);
        }
    }

    async addMovie(event) {
        event.preventDefault();
        const form = this.shadowRoot.querySelector('form');
        const name = form.querySelector('#movieName').value;
        const stars = parseInt(form.querySelector('#movieStars').value);

        try {
            const response = await fetch('http://localhost:8080/movies/create', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({name, stars})
            });

            if (response.ok) {
                const movie = await response.json();
                this.movies.push(movie);
                this.render();
                form.reset();
            }
        } catch (error) {
            console.error('Error adding movie:', error);
        }
    }

    render() {
        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                    font-family: 'Arial', sans-serif;
                    max-width: 800px;
                    margin: 0 auto;
                    padding: 20px;
                }
                
                .add-movie-form {
                    background: #f8f9fa;
                    padding: 20px;
                    border-radius: 8px;
                    margin-bottom: 20px;
                }
                
                .form-group {
                    margin-bottom: 15px;
                }
                
                label {
                    display: block;
                    margin-bottom: 5px;
                    color: #2c3e50;
                }
                
                input, select {
                    width: 100%;
                    padding: 8px;
                    border: 1px solid #ddd;
                    border-radius: 4px;
                    box-sizing: border-box;
                }
                
                button {
                    background: #2ecc71;
                    color: white;
                    border: none;
                    padding: 10px 20px;
                    border-radius: 4px;
                    cursor: pointer;
                    transition: background 0.3s;
                }
                
                button:hover {
                    background: #27ae60;
                }
                
                .movies-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
                    gap: 20px;
                    padding: 20px 0;
                }
                
                .no-movies {
                    text-align: center;
                    color: #7f8c8d;
                    padding: 20px;
                }
            </style>
            
            <div class="movie-list">
                <form class="add-movie-form">
                    <div class="form-group">
                        <label for="movieName">Movie Name</label>
                        <input type="text" id="movieName" required>
                    </div>
                    <div class="form-group">
                        <label for="movieStars">Rating (1-5 stars)</label>
                        <input type="text" id="movieStars" required>
                    </div>
                    <button type="submit">Add Movie</button>
                </form>
                
                <div class="movies-grid">
                    ${this.movies.length ?
            this.movies.map(movie => `
                            <movie-detail movie-data='${JSON.stringify(movie)}'></movie-detail>
                        `).join('') :
            '<div class="no-movies">No movies added yet</div>'
        }
                </div>
            </div>
        `;
    }
}

customElements.define('movie-list', MovieList);
