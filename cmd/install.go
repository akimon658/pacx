package cmd

import (
	"errors"
	"fmt"

	"github.com/spf13/cobra"

	"github.com/akimon658/pacx/config"
)

var installCmd = &cobra.Command{
	Use: "install",
	RunE: func(cmd *cobra.Command, args []string) error {
		if len(args) == 0 {
			return errors.New("no package specified")
		}

		return install(argsToPkgInfo(args))
	},
}

func install(pkgs []pkgInfo) error {
	var errs error

	for i := range pkgs {
		cfg, err := config.Load(pkgs[i].manager)
		if err != nil {
			return err
		}
		defer cfg.Close()

		if err := cfg.Install(pkgs[i].name); err != nil {
			errs = fmt.Errorf("\n%w", err)
		}
	}

	return errs
}
