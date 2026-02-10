package service

import (
	"context"
	"crypto/sha256"
	"encoding/base64"
	"time"

	"github.com/golang-jwt/jwt/v5"

	"github.com/Doctor46-create/gophkeeper/internal/domain"
	"github.com/Doctor46-create/gophkeeper/internal/logger"
	"github.com/Doctor46-create/gophkeeper/internal/repository"
	"github.com/Doctor46-create/gophkeeper/internal/utils"
)

type serviceImplementation struct {
	repo      repository.Storage
	secretKey string
}

func New(repo repository.Storage, secret string) Service {
	return &serviceImplementation{
		repo:      repo,
		secretKey: secret,
	}
}

func (s *serviceImplementation) Register(ctx context.Context, login, password string) error {
	reqID := ctx.Value(utils.ReqIDKey).(string)
	logger.Log.Infow("Register attempt",
		"request_id", reqID,
		"login", login,
	)
	return s.repo.CreateUser(ctx, login, s.hashPass(password))
}

func (s *serviceImplementation) Login(ctx context.Context, login, password string) (string, error) {
	hash, err := s.repo.GetUser(ctx, login)
	if err != nil {
		return "", domain.ErrInvalidCreds
	}
	if hash != s.hashPass(password) {
		return "", domain.ErrInvalidCreds
	}
	claims := jwt.MapClaims{"login": login, "exp": time.Now().Add(24 * time.Hour).Unix()}
	return jwt.NewWithClaims(jwt.SigningMethodHS256, claims).SignedString([]byte(s.secretKey))
}

func (s *serviceImplementation) hashPass(p string) string {
	h := sha256.New()
	h.Write([]byte(p))
	return base64.StdEncoding.EncodeToString(h.Sum(nil))
}

func (s *serviceImplementation) SaveSecrets(ctx context.Context, userLogin string, secrets []domain.Secret) error {
	now := time.Now()
	
	for i := range secrets {
		secrets[i].UserLogin = userLogin
		
		if secrets[i].CreatedAt.IsZero() {
			secrets[i].CreatedAt = now
		}
		
		secrets[i].UpdatedAt = now
	}
	
	return s.repo.SaveSecrets(ctx, userLogin, secrets)
}

func (s *serviceImplementation) GetData(ctx context.Context, login string) ([]domain.Secret, error) {
	return s.repo.GetData(ctx, login)
}

func (s *serviceImplementation) DeleteSecret(ctx context.Context, login, id string) error {
	return s.repo.DeleteData(ctx, login, id)
}
