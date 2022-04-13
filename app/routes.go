package app

import (
	"github.com/labstack/echo/v4"
	"github.com/pnp-zone/gorps/handler/account"
	"gorm.io/gorm"
)

func AddRoutes(e *echo.Echo, db *gorm.DB, config *Config) {
	accHandler := account.Handler{
		DB: db,
	}
	e.POST("/api/v1/login", accHandler.Login())
}
