package main

import (
	"encoding/json"
	"net/http"
	"strconv"
	"sync"
)

type Movie struct {
	Id    int    `json:"id"`
	Name  string `json:"name"`
	Stars int    `json:"stars"`
}

type MovieStore struct {
	sync.RWMutex
	movies []Movie
	nextID int
}

func NewMovieStore() *MovieStore {
	return &MovieStore{
		movies: make([]Movie, 0),
		nextID: 0,
	}
}

func (ms *MovieStore) Add(movie Movie) Movie {
	ms.Lock()
	defer ms.Unlock()

	movie.Id = ms.nextID
	ms.nextID++
	ms.movies = append(ms.movies, movie)
	return movie
}

func (ms *MovieStore) GetAll() []Movie {
	ms.RLock()
	defer ms.RUnlock()

	result := make([]Movie, len(ms.movies))
	copy(result, ms.movies)
	return result
}

func (ms *MovieStore) Get(id int) (Movie, bool) {
	ms.RLock()
	defer ms.RUnlock()

	for _, movie := range ms.movies {
		if movie.Id == id {
			return movie, true
		}
	}
	return Movie{}, false
}

func (ms *MovieStore) Remove(id int) bool {
	ms.Lock()
	defer ms.Unlock()

	for i, movie := range ms.movies {
		if movie.Id == id {
			ms.movies = append(ms.movies[:i], ms.movies[i+1:]...)
			return true
		}
	}
	return false
}

func respondWithError(w http.ResponseWriter, code int, message string) {
	respondWithJSON(w, code, map[string]string{"error": message})
}
func enableCors(handler http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// Allow requests from any origin
		w.Header().Set("Access-Control-Allow-Origin", "*")

		// Allow specific HTTP methods
		w.Header().Set("Access-Control-Allow-Methods", "GET, POST, DELETE, OPTIONS")

		// Allow specific headers
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type")

		// Handle preflight requests
		if r.Method == "OPTIONS" {
			w.WriteHeader(http.StatusOK)
			return
		}

		handler(w, r)
	}
}
func respondWithJSON(w http.ResponseWriter, code int, payload interface{}) {
	response, err := json.Marshal(payload)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(code)
	w.Write(response)
}

func handleAddMovie(store *MovieStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			respondWithError(w, http.StatusMethodNotAllowed, "Invalid method")
			return
		}

		var movie Movie
		if err := json.NewDecoder(r.Body).Decode(&movie); err != nil {
			respondWithError(w, http.StatusBadRequest, "Invalid request payload")
			return
		}

		if movie.Name == "" || movie.Stars < 0 || movie.Stars > 6 {
			respondWithError(w, http.StatusBadRequest, "Invalid movie data")
			return
		}

		store.Add(movie)
		respondWithJSON(w, http.StatusCreated, movie)
	}
}

func handleGetMovies(store *MovieStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			respondWithError(w, http.StatusMethodNotAllowed, "Invalid method")
			return
		}

		movies := store.GetAll()
		respondWithJSON(w, http.StatusOK, movies)
	}
}

func handleGetMovie(store *MovieStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			respondWithError(w, http.StatusMethodNotAllowed, "Invalid method")
			return
		}

		idStr := r.URL.Query().Get("id")
		id, err := strconv.Atoi(idStr)
		if err != nil {
			respondWithError(w, http.StatusBadRequest, "Invalid movie ID")
			return
		}

		movie, found := store.Get(id)
		if !found {
			respondWithError(w, http.StatusNotFound, "Movie not found")
			return
		}

		respondWithJSON(w, http.StatusOK, movie)
	}
}

func handleRemoveMovie(store *MovieStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodDelete {
			respondWithError(w, http.StatusMethodNotAllowed, "Invalid method")
			return
		}

		idStr := r.URL.Query().Get("id")
		id, err := strconv.Atoi(idStr)
		if err != nil {
			respondWithError(w, http.StatusBadRequest, "Invalid movie ID")
			return
		}

		if removed := store.Remove(id); !removed {
			respondWithError(w, http.StatusNotFound, "Movie not found")
			return
		}

		respondWithJSON(w, http.StatusOK, map[string]string{"message": "Movie removed"})
	}
}

func main() {
	store := NewMovieStore()

	http.HandleFunc("/movies/create", enableCors(handleAddMovie(store)))
	http.HandleFunc("/movies/delete", enableCors(handleRemoveMovie(store)))
	http.HandleFunc("/movies/get", enableCors(handleGetMovie(store)))
	http.HandleFunc("/movies", enableCors(handleGetMovies(store)))

	if err := http.ListenAndServe(":8080", nil); err != nil {
		panic(err)
	}
}
