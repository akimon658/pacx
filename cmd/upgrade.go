package cmd

import (
	"errors"

	"github.com/spf13/cobra"
)

var upgradeCmd = &cobra.Command{
	Use: "upgrade",
	RunE: func(cmd *cobra.Command, args []string) error {
		if len(args) == 0 {
			return errors.New("not specified a package manager")
		}
		return upgrade(args[0], args[1:])
	},
}

func upgrade(pkgManager string, pkgs []string) error {
	return nil
}
