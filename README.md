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
docker compose -f docker-compose.dev.yaml up --build
```
- Prod: 
```bash 
docker-compose -f docker-compose.prod.yaml up
```
### Frontend
- Dev: `vite --mode dev`
- Prod: `vite build --mode prod`


to start debuggin in the container use gdb:
gdbserver :2345 target/debug/backend


### sea orm migrations:
```bash 
sea-orm-cli generate entity -o backend/entity/src --lib
```

### tesseract depend on :
```bash 
sudo pacman -S --needed cmake base-devel
```
