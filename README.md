## Seed Users

These are the initial users for development:

| Username       | Email                     | Role       | Password      |
|----------------|---------------------------|------------|---------------|
| `normaluser`   | `user@example.com`        | USER       | `password123` |
| `moderator`    | `moderator@example.com`   | MODERATOR  | `password123` |
| `admin`        | `admin@example.com`       | ADMIN      | `password123` |

> **Note:** All passwords are hashed using Argon2 (via `libreauth`) before storing in the database.