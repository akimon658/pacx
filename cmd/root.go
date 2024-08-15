package cmd

import "github.com/spf13/cobra"

var rootCmd = &cobra.Command{
	Use: "pacx",
}

func Execute() error {
	return rootCmd.Execute()
}

func init() {
	rootCmd.AddCommand(outdatedCmd)
	rootCmd.AddCommand(upgradeCmd)
}
