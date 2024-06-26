use std::collections::HashMap;
use ini::Ini;
use std::env;
use std::iter::Map;
use std::process::exit;

mod usage;
mod rider;
mod tool_chain;
mod command_sender;
mod gen_by_file_line;

use tool_chain::ToolChain;


fn main() {
    // read config file
    let mut conf_path = env::current_exe().unwrap().to_str().unwrap().to_string();
    conf_path = conf_path.replace("codelink.exe", "config.ini");
    if cfg!(debug_assertions) {
        println!("debug mode use local config file");
        conf_path = "config.ini".to_string();
    }
    let conf = Ini::load_from_file(conf_path).unwrap();

    let current_tool = conf.section(Some("Current Tool")).unwrap();
    let tool_name = current_tool.get("name").unwrap();

    let mut tool_chains: HashMap<String, tool_chain::ToolChain> = HashMap::new();

    // get all tool chain
    for (sec, prop) in &conf {
        if let Some(sec) = sec {
            match sec {
                "Current Tool" => continue,
                _ => {
                    tool_chains.insert(sec.clone().to_string(), ToolChain::new(
                        sec.clone().to_string(),
                        prop.get("path").unwrap().to_string(),
                        prop.get("cmd").unwrap().to_string(),
                    ));
                },
            };
        }
    }

    // get command arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage::print_usage();
        return;
    }
    match args[1].as_str() {
        "from_rider" => {
            rider::from_rider();
        },
        "to_rider" => {
            rider::to_rider();
        },
        "copy" => {
            if args.len() < 3 {
                usage::print_usage();
                return;
            }
            // copy link to clipboard
            let mut link: String = args[2].clone();
            clipboard_win::set_clipboard_string(&link).unwrap();
        },
        "open" => {
            if args.len() < 3 {
                usage::print_usage();
                return;
            }
            command_sender::open(&tool_chains, &args[2], &tool_name.to_string());
        }
        "gen" => {
            if args.len() < 4 {
                usage::print_usage();
                return;
            }
            gen_by_file_line::from_parameters(args[2].clone(), args[3].clone());
        },
        _ => {},
    }
}
