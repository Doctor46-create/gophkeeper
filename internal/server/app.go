package server

import (
	"context"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"go.uber.org/zap"

	"github.com/Doctor46-create/gophkeeper/internal/config"
	"github.com/Doctor46-create/gophkeeper/internal/logger"
	"github.com/Doctor46-create/gophkeeper/internal/repository"
	"github.com/Doctor46-create/gophkeeper/internal/server/service"
	"github.com/Doctor46-create/gophkeeper/internal/server/transport"
)

type ServerApp struct {
	cfg       config.ServerConfig
	storage   repository.Storage
	svc       service.Service
	transport transport.Transport
	server    *http.Server
}

func Execute() {
	cfg := config.NewServerConfig()

	if err := logger.Initialize(cfg.GetLogLevel()); err != nil {
		panic(err)
	}

	defer logger.Sync()

	logger.Log.Infow("Starting Gophkeeper Server v2.0", zap.String("mode", "prod"))

	var storage repository.Storage
	dsn := cfg.GetDatabaseDSN()

	if dsn != "" {
		logger.Log.Info("Database provided, initializing Postgres...")

		if err := repository.RunMigrations(dsn); err != nil {
			logger.Log.Fatal("Migrations failed", zap.Error(err))
		}

		dbcfg := cfg.GetDBConfig()
		pgStorage, err := repository.NewPostgresSQLStorage(dsn, dbcfg)
		if err != nil {
			logger.Log.Fatal("Failed to init DB", zap.Error(err))
		}

		storage = pgStorage
	} else {
		logger.Log.Warn("Database DSN is empty, using In-Memory Storage. Data will be lost on restart!")
		storage = repository.NewMemoryStorage()
	}

	serverService := service.New(storage, cfg.GetSecretKey())

	serverTransport := transport.New(serverService)

	app := &ServerApp{
		cfg:       cfg,
		storage:   storage,
		svc:       serverService,
		transport: serverTransport,
	}

	if err := app.Run(); err != nil {
		logger.Log.Error("Server run error", zap.Error(err))
	}
}

func (a *ServerApp) Run() error {
	mux := http.NewServeMux()
	mux.HandleFunc("/api/register", a.transport.RegisterHandler)
	mux.HandleFunc("/api/login", a.transport.LoginHandler)
	mux.Handle("/api/data", transport.AuthMiddleware(
    a.cfg.GetSecretKey(), 
    http.HandlerFunc(a.transport.DataHandler),
))

	handlerChain := transport.LoggingMiddleware(transport.PanicRecoveryMiddleware(mux))

	a.server = &http.Server{
		Addr:    a.cfg.GetRunAddress(),
		Handler: handlerChain,
	}

	go func() {
		logger.Log.Infow("Server is listening", zap.String("addr", a.cfg.GetRunAddress()))
		if err := a.server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			logger.Log.Fatal("Server listen fatal error", zap.Error(err))
		}
	}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	sig := <-quit

	logger.Log.Infow("Received shutdown signal", zap.String("signal", sig.String()))

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := a.server.Shutdown(ctx); err != nil {
		logger.Log.Error("HTTP server shutdown error", zap.Error(err))
		return err
	}

	logger.Log.Info("Closing database connections")
	if err := a.storage.Close(); err != nil {
		logger.Log.Error("Storage close error", zap.Error(err))
	}

	logger.Log.Info("Server shutdown complete")
	return nil
}
