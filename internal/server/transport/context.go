package transport

type ctxKey string

const (
	ReqIDKey          ctxKey = "request_id"
	UserLoginKey      ctxKey = "user_login"
)
