use std::env;

pub fn parse_file_args() -> Option<String>
{
    let args: Vec<String> = env::args().collect();
    return if args.len() > 1 {
        Some(args[args.len() - 1].to_string()) // last argument
    } else {
        None
    };
}