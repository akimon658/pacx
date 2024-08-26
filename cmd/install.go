package cmd

import (
	"errors"
	"slices"
	"strings"

	"github.com/spf13/cobra"

	"github.com/akimon658/pacx/config"
)

var installCmd = &cobra.Command{
	Use:   "install",
	Short: "Install packages",
	Long: `Install packages.
Arguments should be in the format of "manager:package".`,
	RunE: func(cmd *cobra.Command, args []string) error {
		if len(args) == 0 {
			return errors.New("no package specified")
		}

		return install(argsToPkgInfo(args))
	},
}

func install(pkgs []pkgInfo) error {
	var funcUndefinedManagers []string

	for i := range pkgs {
		cfg, err := config.Load(pkgs[i].manager)
		if err != nil {
			return err
		}
		defer cfg.Close()

		if err := cfg.Install(pkgs[i].name); err != nil {
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

		return errors.New("function install is not defined for " + strings.Join(funcUndefinedManagers, ", "))
	}

	return nil
}
