package main

import (
	"github.com/hellflame/argparse"
	"github.com/pnp-zone/gorps/app"
)

func main() {
	parser := argparse.NewParser("gorps-server", "", &argparse.ParserConfig{
		DisableDefaultShowHelp: true,
	})
	configPath := parser.String("", "config-path", &argparse.Option{
		Default: "config.toml",
		Help:    "Specify the path to the config",
	})

	if err := parser.Parse(nil); err != nil {
		panic(err)
	}

	switch {
	case parser.Invoked:
		app.StartServer(*configPath)
	}
}
