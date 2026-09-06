pub enum Flags {
    Display {
        stop_code: String,
        bus_filter: Option<String>,
    },

    List {
        search_pattern: Option<String>,
    },
}

impl Flags {
    pub fn parse(mut args: std::env::Args) -> Result<Self, ParseFlagsError> {
        // If no arguments are provided, usage should be printed
        let (Some(_), Some(arg_1)) = (args.next(), args.next()) else {
            return Err(ParseFlagsError::Usage);
        };

        // If arg_1 is --help or -h, usage should be printed
        if arg_1 == "-h" || arg_1 == "--help" {
            return Err(ParseFlagsError::Usage);
        }

        // If first argument is -l or --list, enter listing mode
        if arg_1 == "-l" || arg_1 == "--list" {
            let search_pattern = args.next();
            return Ok(Flags::List { search_pattern });
        }

        //Everything below is "display mode"

        // Get stop code as an argument
        let stop_code = arg_1;

        // If stop code isn't a number, the program should be interrupted
        if stop_code.parse::<u32>().is_err() {
            return Err(ParseFlagsError::UnparseableStopNumber);
        }

        // If there are no flags, continue execution without bus fiter
        let Some(flag) = args.next() else {
            return Ok(Flags::Display {
                stop_code,
                bus_filter: None,
            });
        };

        // If there are flags, check if the flag is known
        if flag != "-B" && flag != "--bus" {
            return Err(ParseFlagsError::UnkownFlag(flag));
        }

        // Check if bus flag had a bus number beside it
        let bus_filter = args.next();
        if bus_filter.is_none() {
            return Err(ParseFlagsError::BusWithoutNumber);
        };

        // Build flags using stop_code and bus_filter
        Ok(Flags::Display {
            stop_code,
            bus_filter,
        })
    }
}

pub enum ParseFlagsError {
    Usage,
    UnparseableStopNumber,
    UnkownFlag(String),
    BusWithoutNumber,
}
