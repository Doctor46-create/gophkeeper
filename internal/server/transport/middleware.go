package transport

import (
	"context"
	"net/http"
	"runtime/debug"
	"strings"

	"github.com/golang-jwt/jwt/v5"
	"github.com/google/uuid"

	"github.com/Doctor46-create/gophkeeper/internal/logger"
	"github.com/Doctor46-create/gophkeeper/internal/utils"
)

func LoggingMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		reqID := r.Header.Get("X-Request-ID")
		if reqID == "" {
			reqID = uuid.NewString()
		}

		ctx := context.WithValue(r.Context(), utils.ReqIDKey, reqID)
		r = r.WithContext(ctx)
		w.Header().Set("X-Request-ID", reqID)

		logger.Log.Infow("Incoming Request",
			"request_id", reqID,
			"method", r.Method,
			"path", r.URL.Path,
		)

		next.ServeHTTP(w, r)
	})
}

func PanicRecoveryMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		defer func() {
			if err := recover(); err != nil {
				reqID, _ := r.Context().Value(utils.ReqIDKey).(string)

				logger.Log.Errorw("PANIC RECOVERED",
					"panic", err,
					"request_id", reqID,
					"stack_trace", string(debug.Stack()),
				)

				http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			}
		}()

		next.ServeHTTP(w, r)
	})
}

func AuthMiddleware(secret string, next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		authHeader := r.Header.Get("Authorization")
		if authHeader == "" {
			reqID, _ := r.Context().Value(utils.ReqIDKey).(string)
			logger.Log.Warnw("Auth failed: missing header", "request_id", reqID)
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}

		tokenString := strings.TrimPrefix(authHeader, "Bearer ")

		token, err := jwt.Parse(tokenString, func(t *jwt.Token) (any, error) {
			return []byte(secret), nil
		})
		if err != nil || !token.Valid {
			reqID, _ := r.Context().Value(utils.ReqIDKey).(string)
			logger.Log.Warnw("Auth failed: invalid token",
				"request_id", reqID,
				"error", err,
			)
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}

		if claims, ok := token.Claims.(jwt.MapClaims); ok {
			if login, ok := claims["login"].(string); ok {
				ctx := context.WithValue(r.Context(), utils.UserLoginKey, login)
				r = r.WithContext(ctx)
			} else {
				reqID, _ := r.Context().Value(utils.ReqIDKey).(string)
				logger.Log.Warnw("Auth failed: no login in token", "request_id", reqID)
				http.Error(w, "unauthorized", http.StatusUnauthorized)
				return
			}
		}

		next.ServeHTTP(w, r)
	})
}
