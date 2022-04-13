package account

type LoginRequest struct {
	Username *string `json:"username" echotools:"required,not empty"`
	Password *string `json:"password" echotools:"required,not empty"`
}

type RegisterRequest struct {
	Username *string `json:"username" echotools:"required,not empty"`
	Password *string `json:"password" echotools:"required,not empty"`
}
