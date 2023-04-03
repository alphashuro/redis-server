#[derive(Debug)]
pub enum RespType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<RespType>),
}

#[derive(Debug)]
pub enum SubCommand {
    None,
    Docs,
}

#[derive(Debug)]
pub enum RedisCmd {
    Ping,
    Echo(String),
    Command(SubCommand),
}

pub fn parse_command(strings: &Vec<String>) -> Result<RedisCmd, String> {
    let lower: Vec<String> = strings.iter().map(|s| s.to_lowercase()).collect();
    let lower_str: Vec<&str> = lower.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    match lower_str.as_slice() {
        ["ping"] => Ok(RedisCmd::Ping),
        ["command", "docs"] => Ok(RedisCmd::Command(SubCommand::Docs)),
        _ => Err(format!("command not known: {:?}", strings)),
    }
}

pub fn exec_command(cmd: &RedisCmd) -> RespType {
    match cmd {
        RedisCmd::Ping => RespType::SimpleString("PONG".to_string()),
        RedisCmd::Command(SubCommand::Docs) => RespType::SimpleString("OK".to_string()),
        RedisCmd::Echo(text) => RespType::SimpleString(text.to_string()),
        _ => todo!(),
    }
}

pub fn format_response(response: &RespType) -> String {
    match response {
        RespType::SimpleString(text) => format!("+{}\r\n", text),
        _ => todo!(),
    }
}

pub fn run(request: &Vec<String>) -> String {
    let command = parse_command(request).unwrap();
    let response = exec_command(&command);

    format_response(&response)
}
