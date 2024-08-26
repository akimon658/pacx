package cmd

import (
	"errors"
	"slices"
	"strings"

	"github.com/spf13/cobra"

	"github.com/akimon658/pacx/config"
)

var upgradeCmd = &cobra.Command{
	Use:   "upgrade",
	Short: "Upgrade packages",
	Long: `Upgrade packages.
Arguments can be in the format of "manager:package" or just "manager".
Note that whether "package" is specified or not should be handled in your Lua code.`,
	Args: cobra.MinimumNArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		return upgrade(argsToPkgInfo(args))
	},
}

func upgrade(pkgs []pkgInfo) error {
	var funcUndefinedManagers []string

	for i := range pkgs {
		cfg, err := config.Load(pkgs[i].manager)
		if err != nil {
			return err
		}
		defer cfg.Close()

		if err := cfg.Upgrade(pkgs[i].name); err != nil {
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

		return errors.New("function upgrade is not defined for " + strings.Join(funcUndefinedManagers, ", "))
	}

	return nil
}
