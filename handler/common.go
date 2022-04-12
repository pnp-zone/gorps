package handler

import (
	"github.com/labstack/echo/v4"
	"gorm.io/gorm"
)

type Context struct {
	Context echo.Context
	DB      *gorm.DB
}
