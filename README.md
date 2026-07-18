# EMTui 0.2.1
EMTui is a terminal user interface for buses operated by "EMT Madrid"

It can display buses that will arrive to a stop:
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

Or get the stop number of a certain stop via pattern matchin:
```
emtui -l "Tirso"

Instituto Tirso de Molina                    -> 1011
Instituto Tirso de Molina                    -> 1012
Mercado Tirso de Molina                      -> 1453
Tirso de Molina                              -> 1919
Colegiata-Tirso de Molina                    -> 51146
Tirso de Molina                              -> 51229
Conde de Romanones-Tirso de Molina           -> 51231
Conde de Romanones-Tirso de Molina           -> 51232
```

## Installation
1. Create [EMT-Madrid account](mobilitylabs.emtmadrid.es)
2. Get the binary crate with `cargo install emtui` or download the source code and build it with `cargo build --release`

## Usage
- Use `emtui <bus-stop-code>` to get the two next arrivals of every bus in the stop
- Run with the `--bus` or `-B` flags and specify a `<bus-number>` afterwards to only get arrivals of said bus
- If you don't know a stop code but you know its name, try running `emtui -l <stop-name>`

Examples:
```
emtui 3688
emtui 3688 --bus 133
emtui -l "Moncloa"
```

## Notes
You will be required to login upon first launch of the program, your credentials are stored in plain text in a config file stored here:
| OS      | Config file                                               |
| ------- | --------------------------------------------------------- |
| Windows | `{FOLDERID_RoamingAppData}\emtui\credentials.txt`         |
| linux   | `$XDG_CONFIG_HOME/emtui/credentials.txt`                  |
| macOS   | `$HOME/Library/Application Support/emtui/credentials.txt` |
