package service

import (
	"context"

	"github.com/Doctor46-create/gophkeeper/internal/domain"
)

type Service interface {
	Register(ctx context.Context, login, password string) error
	Login(ctx context.Context, login, password string) (string, error)

	SaveSecrets(ctx context.Context, userLogin string, secrets []domain.Secret) error
	GetData(ctx context.Context, login string) ([]domain.Secret, error)
	DeleteSecret(ctx context.Context, login, id string) error
}
