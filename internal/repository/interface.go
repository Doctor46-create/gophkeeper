package repository

import (
	"context"

	"github.com/Doctor46-create/gophkeeper/internal/domain"
)

type Storage interface {
	UserRepository
	DataRepository
	RunInTransaction(ctx context.Context, fn func(ctx context.Context) error) error
	Close () error
}

type UserRepository interface {
	CreateUser(ctx context.Context, login, hash string) error
	GetUser(ctx context.Context, login string) (string, error)
}

type DataRepository interface {
	AddData (ctx context.Context, login string, secret domain.Secret) error
	SaveSecrets(ctx context.Context, login string, secrets []domain.Secret) error
	GetData(ctx context.Context, login string) ([]domain.Secret, error)
	DeleteData(ctx context.Context, login, id string) error
}
