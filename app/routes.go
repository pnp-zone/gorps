package app

import (
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/pnp-zone/gorps/handler/account"
	"github.com/pnp-zone/gorps/handler/index"
	"gorm.io/gorm"
	"path/filepath"
)

func AddRoutes(e *echo.Echo, db *gorm.DB, config *Config) {
	accHandler := account.Handler{
		DB: db,
	}

	e.POST("/api/v1/login", accHandler.Login())
	e.POST("/api/v1/logout", accHandler.Logout())

	e.GET("/", index.Index())

	group := e.Group("/static")
	group.Use(middleware.Static(filepath.Join(config.Server.StaticDirPath)))
}
