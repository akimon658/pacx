package cmd

import (
	"errors"
	"slices"
	"strings"

	"github.com/spf13/cobra"

	"github.com/akimon658/pacx/config"
)

var whyCmd = &cobra.Command{
	Use:   "why",
	Short: "Show why a package is installed",
	Long: `Show why a package is installed.,
Arguments should be in the format of "manager:package".`,
	Args: cobra.MinimumNArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		return why(argsToPkgInfo(args))
	},
}

func why(pkgs []pkgInfo) error {
	var funcUndefinedManagers []string

	for i := range pkgs {
		cfg, err := config.Load(pkgs[i].manager)
		if err != nil {
			return err
		}
		defer cfg.Close()

		if err := cfg.Why(pkgs[i].name); err != nil {
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

		return errors.New("function why is not defined for " + strings.Join(funcUndefinedManagers, ", "))
	}

	return nil
}
