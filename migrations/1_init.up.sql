CREATE TABLE IF NOT EXISTS users (
    login VARCHAR(255) PRIMARY KEY,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS secrets (
    id VARCHAR(255) NOT NULL,
    user_login VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL,
    data TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (id, user_login),
    CONSTRAINT fk_user FOREIGN KEY(user_login)
        REFERENCES users(login)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_secrets_user_login ON secrets(user_login);
