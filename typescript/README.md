# Dir Manager

This script is command line interface (CLI) tool that allows you to perform various actions related to managing a directory.

The tool has several options that you can specify when you run it:

-l, --ls [value]: List the contents of the specified directory. If no value is provided, it will list the contents of the current directory.

-m, --mkdir <value>: Create a new directory with the specified name.

-t, --touch <value>: Create a new empty file with the specified name.

The script also includes functionality to display a banner using the figlet library, and to handle errors that may occur while reading a directory or creating a file.

Finally, if no options are provided when the script is run, it will display a help message with usage information for the available options.
