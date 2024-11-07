# Usage

```sh
Usage: hello-world <COMMAND>

Commands:
  lint   Lint the configuration file
  print  Print the configuration file in a specific format
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Example usage

```sh
hello-world lint < example.yaml
```

```sh
hello-world print evolution < example.yaml > filters.xml
```

```sh
hello-world print thunderbird < example.yaml > msgFilterRules.dat
```
