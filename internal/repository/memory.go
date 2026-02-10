package repository

import (
	"context"
	"sync"
	"time"

	"github.com/Doctor46-create/gophkeeper/internal/domain"
)

type MemoryStorage struct {
	mu          sync.RWMutex
	users       map[string]string
	userSecrets map[string][]domain.Secret
}

func NewMemoryStorage() *MemoryStorage {
	return &MemoryStorage{
		users:       make(map[string]string),
		userSecrets: make(map[string][]domain.Secret),
	}
}

func (s *MemoryStorage) Close() error {
	return nil
}

func (s *MemoryStorage) RunInTransaction(ctx context.Context, fn func(ctx context.Context) error) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	select {
	case <-ctx.Done():
		return ctx.Err()
	default:
	}

	return fn(ctx)
}

func (s *MemoryStorage) CreateUser(ctx context.Context, login, hash string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if _, ok := s.users[login]; ok {
		return domain.ErrUserExists
	}

	s.users[login] = hash
	return nil
}

func (s *MemoryStorage) GetUser(ctx context.Context, login string) (string, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	hash, ok := s.users[login]
	if !ok {
		return "", domain.ErrInvalidCreds
	}

	return hash, nil
}

func (s *MemoryStorage) SaveSecrets(ctx context.Context, login string, secrets []domain.Secret) error {
	if len(secrets) == 0 {
		return nil
	}

	s.mu.Lock()
	defer s.mu.Unlock()

	now := time.Now()

	existingSecrets := s.userSecrets[login]

	existingMap := make(map[string]int)
	for i, secret := range existingSecrets {
		existingMap[secret.ID] = i
	}

	for _, incomingSecret := range secrets {
		secret := incomingSecret

		secret.UserLogin = login

		secret.UpdatedAt = now

		if idx, exists := existingMap[secret.ID]; exists {
			secret.CreatedAt = existingSecrets[idx].CreatedAt
			existingSecrets[idx] = secret
		} else {
			secret.CreatedAt = now
			existingSecrets = append(existingSecrets, secret)
			existingMap[secret.ID] = len(existingSecrets) - 1
		}
	}

	s.userSecrets[login] = existingSecrets

	return nil
}

func (s *MemoryStorage) AddData(ctx context.Context, login string, secret domain.Secret) error {
	return s.SaveSecrets(ctx, login, []domain.Secret{secret})
}

func (s *MemoryStorage) GetData(ctx context.Context, login string) ([]domain.Secret, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	items := s.userSecrets[login]
	result := make([]domain.Secret, len(items))
	copy(result, items)

	return result, nil
}

func (s *MemoryStorage) DeleteData(ctx context.Context, login, id string) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	items, exists := s.userSecrets[login]
	if !exists {
		return domain.ErrSecretNotFound
	}

	var newItems []domain.Secret
	found := false

	for _, v := range items {
		if v.ID == id {
			found = true
			continue
		}
		newItems = append(newItems, v)
	}

	if !found {
		return domain.ErrSecretNotFound
	}

	s.userSecrets[login] = newItems
	return nil
}
