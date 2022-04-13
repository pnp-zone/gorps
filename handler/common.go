package handler

import (
	"github.com/labstack/echo/v4"
	"github.com/myOmikron/echotools/middleware"
	"gorm.io/gorm"
)

type Context struct {
	echo.Context
	middleware.SessionContext
	DB *gorm.DB
}
