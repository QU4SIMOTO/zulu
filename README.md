# ZULU
Zebra Utilities for Linux Users

## Description
Annoyed at netcatting SDG and ZPL commands to ZD421 on linux, so very small cli to make my life easier.

In the future I may add some of the tools from the official zebra utilities app on windows and support other means of comunication to the printer such as usb.

## Usage
```
A small cli to make interacting with zebra printers on linux slightly less annoying.

Usage: zulu [OPTIONS] <COMMAND>

Commands:
  get     Get a configuration value by key.
  set     Set a configuration value by key.
  do      Perform an action by name.
  upload  Upload files such as firmware, certificates or keys.
  help    Print this message or the help of the given subcommand(s)

Options:
  -a, --addr <ADDR>        The address of the printer [default: 192.168.0.40:9100]
  -t, --timeout <TIMEOUT>  Timeout in seconds for network operations [default: 5]
  -h, --help               Print help
  -V, --version            Print version
```

## Examples
Set https port:
```
zulu set ip.https.port 443
```

Get https port:
```
zulu get ip.https.port
443
```

Reset the printer:
```
zulu do device.reset
```

## Installation
To install using cargo:
```
cargo install --git https://github.com/QU4SIMOT/zulu
```

