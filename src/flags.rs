pub struct Flags {
    pub stop_code: String,
    pub bus_filter: Option<String>,
}

impl Flags {
    pub fn parse(args: Vec<String>) -> Result<Self, &'static str> {
        // If no arguments are provided or argument 1 is --help
        if args.len() < 2 || args[1] == "-h" || args[1] == "--help" {
            return Err("Usage");
        }

        // Check that the stop is a parseable into a number
        if args[1].parse::<u32>().is_err() {
            return Err("Stop code must be a number");
        }
        // Otherwise save the stop argument
        let stop_code = args[1].clone();

        // If there are no flags, continue with this info
        if args.len() < 3 {
            return Ok(Flags {
                stop_code,
                bus_filter: None,
            });
        }

        // If there are flags, check if the optional bus number exists
        let bus_filter: Option<String> =
            if args[2] == "--bus" || args[2] == "-B" {
                if args.len() < 4 {
                    return Err("Bus flag requires a bus number");
                }
                Some(args[3].clone())
            } else {
                None
            };

        return Ok(Flags {
            stop_code,
            bus_filter,
        });
    }
}
