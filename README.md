## Seed Users

These are the initial users for development:

| Username       | Email                     | Role       | Password      |
|----------------|---------------------------|------------|---------------|
| `normaluser`   | `user@example.com`        | USER       | `password123` |
| `moderator`    | `moderator@example.com`   | MODERATOR  | `password123` |
| `admin`        | `admin@example.com`       | ADMIN      | `password123` |

> **Note:** All passwords are hashed using Argon2 (via `libreauth`) before storing in the database.



## Environments

This project does not rely on `.env` at runtime.

### Backend
- Dev (Linux): `direnv` or `make run-dev`
```bash 

```
- Dev (Windows):
```bash 
docker-compose -f docker-compose.dev.yaml up
```
- Prod: 
```bash 
docker-compose -f docker-compose.prod.yaml up
```
### Frontend
- Dev: `vite --mode dev`
- Prod: `vite build --mode prod`
