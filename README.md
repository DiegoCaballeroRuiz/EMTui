# EMTui 0.1
EMTui is the terminal user interface for buses operated by "EMT Madrid"

This is an example of the output:
```
emtui 3688

Moncloa
────────────────────────────────────────
Line 132   HOSPITAL LA PAZ           <1m
Line 133   MIRASIERRA                2m
Line 82    PITIS                     4m
Line 83    BARRIO DE LA PAZ          6m
Line 133   MIRASIERRA                6m
Line 132   HOSPITAL LA PAZ           7m
Line 82    PITIS                     11m
Line 83    BARRIO DE LA PAZ          14m
```

## Installation
1. Create [EMT-Madrid account](mobilitylabs.emtmadrid.es)
2. Grab the source code and build it with cargo or download the latest binary release (cargo crate releasing soon)

## Usage
- Use `emtui <bus-stop-code>` to get the two next arrivals of every bus in the stop
- Run with the `--bus` or `-B` flags and specify a `<bus-number>` afterwards to only get arrivals of said bus

Examples:
```
emtui 3688
emtui 3688 --bus 133
```

## Notes
You will be required to login upon first launch of the program, your credentials are stored in plain text in a config file stored here:
| OS      | Config file                                               |
| ------- | --------------------------------------------------------- |
| Windows | `{FOLDERID_RoamingAppData}\emtui\credentials.txt`         |
| linux   | `$XDG_CONFIG_HOME/emtui/credentials.txt`                  |
| macOS   | `$HOME/Library/Application Support/emtui/credentials.txt` |
