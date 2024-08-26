package cmd

import (
	"strings"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use: "pacx",
}

func Execute() error {
	return rootCmd.Execute()
}

func init() {
	rootCmd.AddCommand(infoCmd)
	rootCmd.AddCommand(installCmd)
	rootCmd.AddCommand(listCmd)
	rootCmd.AddCommand(outdatedCmd)
	rootCmd.AddCommand(upgradeCmd)
}

type pkgInfo struct {
	manager string
	name    string
}

func argsToPkgInfo(args []string) []pkgInfo {
	splitedArgs := make([]pkgInfo, len(args))
	for i := range args {
		splited := strings.Split(args[i], ":")

		splitedArgs[i].manager = splited[0]
		splitedArgs[i].name = strings.Join(splited[1:], "")
	}

	return splitedArgs
}
