package domain

import "errors"

var (
	ErrUserExists     = errors.New("user already exists")
	ErrInvalidCreds   = errors.New("invalid credentials")
	ErrSecretNotFound = errors.New("secret not found")
)
