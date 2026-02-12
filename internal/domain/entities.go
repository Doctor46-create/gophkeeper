package domain

import "time"

type User struct {
	Login        string
	PasswordHash string
}

type Secret struct {
	ID        string     `json:"id"`
	UserLogin string     `json:"user_login"`
	Type      string     `json:"type"`
	Data      string     `json:"data"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}

type AuthRequest struct {
	Login    string `json:"login"`
	Password string `json:"password"`
}

type Token struct {
	JWT string `json:"token"`
}

type SyncRequest struct {
	Secrets []Secret `json:"secrets"`
}
