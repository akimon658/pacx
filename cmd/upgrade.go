package cmd

import (
	"errors"
	"fmt"

	"github.com/spf13/cobra"

	"github.com/akimon658/pacx/config"
)

var upgradeCmd = &cobra.Command{
	Use: "upgrade",
	RunE: func(cmd *cobra.Command, args []string) error {
		if len(args) == 0 {
			return errors.New("no package manager specified")
		}

		return upgrade(argsToPkgInfo(args))
	},
}

func upgrade(pkgs []pkgInfo) error {
	var errs error

	for i := range pkgs {
		cfg, err := config.Load(pkgs[i].manager)
		if err != nil {
			return err
		}
		defer cfg.Close()

		if err := cfg.Upgrade(pkgs[i].name); err != nil {
			errs = fmt.Errorf("\n%w", err)
		}
	}

	return errs
}
