package account

type LoginRequest struct {
	Username *string `json:"username" echotools:"required,not null"`
	Password *string `json:"password" echotools:"required,not null"`
}
