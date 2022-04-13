package app

import (
	"errors"
	"fmt"
	"github.com/myOmikron/echotools/color"
	"github.com/pelletier/go-toml"
	"io/fs"
	"io/ioutil"
	"os"
)

type Server struct {
	ListenAddress string
	StaticDirPath string
}

type Config struct {
	Server Server
}

func ParseConfig(configPath string) *Config {
	var c Config

	if configBytes, err := ioutil.ReadFile(configPath); errors.Is(err, fs.ErrNotExist) {
		color.Printf(color.RED, "Config was not found at %s\n", configPath)
		b, _ := toml.Marshal(&c)
		fmt.Print(string(b))
		os.Exit(1)
	} else {
		if err := toml.Unmarshal(configBytes, &c); err != nil {
			panic(err)
		}
	}

	return &c
}
