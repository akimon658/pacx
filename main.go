package main

import (
	"os"

	"github.com/akimon658/pacx/cmd"
)

func main() {
	if err := cmd.Execute(); err != nil {
		os.Exit(1)
	}
}
