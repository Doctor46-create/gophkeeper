package repository

import (
	"fmt"
	"log"
	"path/filepath"
	"runtime"

	"github.com/golang-migrate/migrate/v4"
	_ "github.com/golang-migrate/migrate/v4/database/postgres"
	_ "github.com/golang-migrate/migrate/v4/source/file"
)

func RunMigrations(dsn string) error {
	_, filename, _, _ := runtime.Caller(0)
	projectRoot := filepath.Dir(filepath.Dir(filepath.Dir(filename)))
	migrationsPath := filepath.Join(projectRoot, "migrations")

	migrationsURL := "file://" + migrationsPath

	log.Printf("Running migrations from: %s", migrationsURL)

	m, err := migrate.New(migrationsURL, dsn)
	if err != nil {
		return fmt.Errorf("migration instance: %w", err)
	}
	defer m.Close()

	if err := m.Up(); err != nil {
		if err.Error() != "no change" && err != migrate.ErrNoChange {
			return fmt.Errorf("migrations failed: %w", err)
		}
	}

	log.Println("Migrations Up-to-date")
	return nil
}
