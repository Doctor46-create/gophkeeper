package transport

import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"net/http"
	"time"

	"github.com/Doctor46-create/gophkeeper/internal/domain"
	"github.com/Doctor46-create/gophkeeper/internal/logger"
	"github.com/Doctor46-create/gophkeeper/internal/server/service"
	"github.com/Doctor46-create/gophkeeper/internal/utils"
)

type HTTPTransport struct{ svc service.Service }

func New(svc service.Service) Transport { return &HTTPTransport{svc: svc} }

func (h *HTTPTransport) RegisterHandler(w http.ResponseWriter, r *http.Request) {
	reqID, _ := r.Context().Value(utils.ReqIDKey).(string)
	reqLogger := logger.Log.With("request_id", reqID)

	var req domain.AuthRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	if err := h.svc.Register(r.Context(), req.Login, req.Password); err != nil {
		reqLogger.Warnw("Reg fail", "error", err)
		http.Error(w, err.Error(), http.StatusConflict)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func (h *HTTPTransport) LoginHandler(w http.ResponseWriter, r *http.Request) {
	bodyBytes, _ := io.ReadAll(r.Body)
	logger.Log.Infow("LoginHandler - raw request",
		"body", string(bodyBytes),
		"content_length", len(bodyBytes),
		"content_type", r.Header.Get("Content-Type"),
	)

	r.Body = io.NopCloser(bytes.NewBuffer(bodyBytes))

	var req domain.AuthRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		logger.Log.Errorw("LoginHandler - JSON decode failed",
			"error", err.Error(),
			"raw_body", string(bodyBytes),
		)
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	logger.Log.Infow("LoginHandler - parsed request",
		"login", req.Login,
		"password_length", len(req.Password),
		"has_password", req.Password != "",
	)

	tok, err := h.svc.Login(r.Context(), req.Login, req.Password)
	if err != nil {
		logger.Log.Warnw("LoginHandler - service failed", 
			"error", err.Error(),
			"login", req.Login,
		)
		http.Error(w, err.Error(), http.StatusUnauthorized)
		return
	}
	logger.Log.Infow("LoginHandler - successful", 
		"login", req.Login,
		"token_length", len(tok),
	)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(domain.Token{JWT: tok})
}

func (h *HTTPTransport) DataHandler(w http.ResponseWriter, r *http.Request) {
	logger.Log.Infow("DataHandler started",
		"method", r.Method,
		"path", r.URL.Path,
		"has_login", r.Context().Value(utils.UserLoginKey) != nil,
	)

	loginVal := r.Context().Value(utils.UserLoginKey)
	if loginVal == nil {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	login, ok := loginVal.(string)
	if !ok {
		http.Error(w, "Internal server error", http.StatusInternalServerError)
		return
	}

	reqID := "unknown"
	if reqIDVal := r.Context().Value(utils.ReqIDKey); reqIDVal != nil {
		reqID, _ = reqIDVal.(string)
	}

	reqLogger := logger.Log.With("request_id", reqID, "login", login)

	switch r.Method {
	case "POST":
		ctx, cancel := context.WithTimeout(r.Context(), 5*time.Second)
		defer cancel()

		var req domain.SyncRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "bad request", http.StatusBadRequest)
			return
		}

		if err := h.svc.SaveSecrets(ctx, login, req.Secrets); err != nil {
			reqLogger.Errorw("Sync fail", "error", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)

	case "GET":
		sec, err := h.svc.GetData(r.Context(), login)
		if err != nil {
			reqLogger.Errorw("Get data fail", "error", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(sec)

	case "DELETE":
		id := r.URL.Query().Get("id")
		if id == "" {
			http.Error(w, "id parameter required", http.StatusBadRequest)
			return
		}

		if err := h.svc.DeleteSecret(r.Context(), login, id); err != nil {
			reqLogger.Errorw("Delete fail", "error", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)

	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}
