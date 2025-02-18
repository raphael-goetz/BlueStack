# Bluefile

The .bluefile is a file extension to achieve the removal boilerplate code of a backend/frontend/database split project.

## Example

For the Curd of a movie you need at least:

```sql
CREATE TABLE movies
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    star INT          NOT NULL CHECK (star >= 1 AND star <= 5)
);
```

```go
type Movie struct {
ID        int       `json:"id"`
Name      string    `json:"name"`
Star      int       `json:"star"`
}

type MovieCreateRequest struct {
Name      string    `json:"name"`
Star      int       `json:"star"`
}

type MovieUpdateRequest type {
ID        int       `json:"id"`
Name      *string    `json:"name"`
Star      *int       `json:"star"`
}
```

```typescript
interface Movie {
    id: number;
    name: string;
    star: number;
}

interface MovieCreateRequest {
    name: string;
    star: number;
}

interface MovieUpdateRequest {
    id: number;
    name?: string;
    star?: number;
}
```

This is generally too much to write every time!

```
byte int float char string boolean []

model Movie {
    id int
    name string 
    star int
}
```