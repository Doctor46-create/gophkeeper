package transport

import "net/http"

type Transport interface {
	RegisterHandler(w http.ResponseWriter, r *http.Request)
	LoginHandler(w http.ResponseWriter, r *http.Request)
	DataHandler(w http.ResponseWriter, r *http.Request)
}
