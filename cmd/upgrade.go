package cmd

import (
	"errors"
	"strings"

	"github.com/spf13/cobra"

	"github.com/akimon658/pacx/config"
)

type pkgInfo struct {
	manager string
	name    string
}

var upgradeCmd = &cobra.Command{
	Use: "upgrade",
	RunE: func(cmd *cobra.Command, args []string) error {
		if len(args) == 0 {
			return errors.New("not specified a package manager")
		}

		splitedArgs := make([]pkgInfo, len(args))
		for i := range args {
			splited := strings.Split(args[i], ":")

			splitedArgs[i].manager = splited[0]
			splitedArgs[i].name = strings.TrimPrefix(args[i], splited[0]+":")
		}

		return upgrade(splitedArgs)
	},
}

func upgrade(pkgs []pkgInfo) error {
	for i := range pkgs {
		cfg, err := config.Load(pkgs[i].manager)
		if err != nil {
			return err
		}
		defer cfg.Close()

		return cfg.Upgrade()
	}
	return nil
}
