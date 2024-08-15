package cmd

import (
	"errors"

	"github.com/spf13/cobra"

	"github.com/akimon658/pacx/config"
)

var outdatedCmd = &cobra.Command{
	Use: "outdated",
	RunE: func(cmd *cobra.Command, args []string) error {
		if len(args) == 0 {
			return errors.New("no package manager specified")
		}
		return outdated(args)
	},
}

func outdated(pkgManagers []string) error {
	for i := range pkgManagers {
		cfg, err := config.Load(pkgManagers[i])
		if err != nil {
			return err
		}
		defer cfg.Close()

		return cfg.Outdated()
	}

	return nil
}
