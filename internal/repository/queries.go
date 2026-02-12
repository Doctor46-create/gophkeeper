package repository

const (
	userCreate = `INSERT INTO users (login, password_hash) VALUES ($1, $2)`
	userGet    = `SELECT password_hash FROM users WHERE login = $1`

	secretUpsert = `INSERT INTO secrets (id, user_login, type, data, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (id, user_login) DO UPDATE
SET type = EXCLUDED.type,
    data = EXCLUDED.data,
    updated_at = EXCLUDED.updated_at
WHERE secrets.updated_at < EXCLUDED.updated_at`

	secretGet    = `SELECT id, user_login, type, data, created_at, updated_at FROM secrets WHERE user_login = $1`
	secretDelete = `DELETE FROM secrets WHERE id = $1 AND user_login = $2`
)
