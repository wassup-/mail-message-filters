# Usage

```sh
Usage: mail-message-filters <COMMAND>

Commands:
  lint   Lint the configuration file
  print  Print the configuration file in a specific format
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Example usage

```sh
mail-message-filters lint < example.yaml
```

```sh
mail-message-filters print evolution < example.yaml > filters.xml
```

```sh
mail-message-filters print thunderbird < example.yaml > msgFilterRules.dat
```
