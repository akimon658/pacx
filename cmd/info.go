package cmd

import (
	"errors"
	"slices"
	"strings"

	"github.com/spf13/cobra"

	"github.com/akimon658/pacx/config"
)

var infoCmd = &cobra.Command{
	Use:   "info",
	Short: "Show information about packages",
	Long: `Show information about packages.
Arguments should be in the format of "manager:package".`,
	Args: cobra.MinimumNArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		return info(argsToPkgInfo(args))
	},
}

func info(pkgs []pkgInfo) error {
	var funcUndefinedManagers []string

	for i := range pkgs {
		cfg, err := config.Load(pkgs[i].manager)
		if err != nil {
			return err
		}
		defer cfg.Close()

		if err := cfg.Info(pkgs[i].name); err != nil {
			if errors.Is(err, config.ErrFunctionNotDefined) {
				funcUndefinedManagers = append(funcUndefinedManagers, pkgs[i].manager)
			} else {
				return err
			}
		}
	}

	if len(funcUndefinedManagers) > 0 {
		slices.Sort(funcUndefinedManagers)
		funcUndefinedManagers = slices.Compact(funcUndefinedManagers)

		return errors.New("function info is not defined for " + strings.Join(funcUndefinedManagers, ", "))
	}

	return nil
}
