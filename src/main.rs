use ini::Ini;

struct ToolChain {
    name: String,
    path: String,
    cmd: String,
}

impl ToolChain {
    fn new(name: String, path: String, cmd: String) -> ToolChain {
        ToolChain {
            name,
            path,
            cmd,
        }
    }
}

fn main() {
    let conf = Ini::load_from_file("config.ini").unwrap();

    let current_tool = conf.section(Some("Current Tool")).unwrap();
    let tool_name = current_tool.get("name").unwrap();

    let mut tool_chains: Vec<ToolChain> = Vec::new();

    // get all tool chain
    for (sec, prop) in &conf {
        if sec != "Current Tool" {
            tool_chains.push(ToolChain::new(
                sec.clone().unwrap(),
                prop.get("path").unwrap().clone(),
                prop.get("cmd").unwrap().clone(),
            ));
        }
    }
}
