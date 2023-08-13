use std::collections::HashMap;
use crate::tool_chain::ToolChain;
use regex::Regex;
use urlencoding::decode;

pub fn open(tool_chains: &HashMap<String, ToolChain>, link: &String, tool_name:&String) {
    // Check tool chain exits
    if !tool_chains.contains_key(tool_name) {
        println!("Tool chain {} not found", tool_name);
        return;
    }
    let tool_chain = &tool_chains[tool_name];
    let mut cmd = tool_chain.cmd.clone();

    // do not check link format, get line number and file path
    let lnum_regex = Regex::new(r"\:\d+").unwrap();
    let mut link:String = decode(link).unwrap().to_string();
    if link.ends_with("/") {
        link.pop();
    }
    println!("link: {:?}", link);
    let mut line_num : String = lnum_regex.captures_iter(&link).next().unwrap().get(0).unwrap().as_str().to_string();
    println!("line_num: {:?}", line_num);
    let mut file_path = link.replace(&line_num, "").replace("codelink://", "");
    line_num = line_num.replace(":", "");

    cmd = cmd.replace("%file%", &file_path)
        .replace("%line%", &line_num)
        .replace("%path%", &tool_chain.path);
    println!("cmd: {}", cmd);
    let _output = std::process::Command::new("cmd")
        .args(&["/C", &cmd])
        .output()
        .expect("failed to execute process");
}