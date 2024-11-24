return {
  subcommands = {
    {
      name = "info",
      description = "Show information about packages",
    },
    {
      name = "install",
      description = "Install packages",
      aliases = {
        "add",
        "i",
      },
    },
    {
      name = "list",
      description = "List packages",
      aliases = {
        "ls",
      },
    },
    {
      name = "outdated",
      description = "List outdated packages",
    },
    {
      name = "unistall",
      description = "Uninstall packages",
      aliases = {
        "delete",
        "remove",
        "rm",
      },
    },
    {
      name = "upgrade",
      description = "Upgrade packages",
    },
    {
      name = "why",
      description = "Show why a package is installed",
    },
  },
}
