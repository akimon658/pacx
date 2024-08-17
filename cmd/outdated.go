package cmd

import (
	"errors"
	"slices"
	"strings"

	"github.com/spf13/cobra"

	"github.com/akimon658/pacx/config"
)

var outdatedCmd = &cobra.Command{
	Use: "outdated",
	RunE: func(cmd *cobra.Command, args []string) error {
		if len(args) == 0 {
			return errors.New("no package manager specified")
		}

		slices.Sort(args)
		args = slices.Compact(args)

		return outdated(args)
	},
}

func outdated(pkgManagers []string) error {
	var funcUndefinedManagers []string

	for i := range pkgManagers {
		cfg, err := config.Load(pkgManagers[i])
		if err != nil {
			return err
		}
		defer cfg.Close()

		if err := cfg.Outdated(); err != nil {
			if errors.Is(err, config.ErrFunctionNotDefined) {
				funcUndefinedManagers = append(funcUndefinedManagers, pkgManagers[i])
			} else {
				return err
			}
		}
	}

	if len(funcUndefinedManagers) > 0 {
		errors.New("function outdated is not defined for " + strings.Join(funcUndefinedManagers, ", "))
	}

	return nil
}
