use std::collections::HashMap;

pub struct Server {
    store: HashMap<String, String>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            store: HashMap::new(),
        }
    }

    pub fn exec(&mut self, cmd: &RedisCmd) -> RespType {
        use RedisCmd::*;
        use RespType::*;

        match cmd {
            Ping => SimpleString("PONG".to_string()),
            Command(SubCommand::Docs) => SimpleString("OK".to_string()),
            Echo(text) => SimpleString(text.to_string()),
            Set(key, value) => {
                self.store.insert(key.to_string(), value.to_string());
                SimpleString("OK".to_string())
            }
            Get(key) => match self.store.get(key) {
                Some(value) => BulkString(value.to_string()),
                None => BulkString("nil".to_string()),
            },
            _ => todo!(),
        }
    }
}

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
    Set(String, String),
    Get(String),
}

pub fn parse_command(strings: &Vec<String>) -> Result<RedisCmd, String> {
    use RedisCmd::*;

    let lower: Vec<String> = strings.iter().map(|s| s.to_lowercase()).collect();
    let lower_str: Vec<&str> = lower.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    match lower_str.as_slice() {
        ["ping"] => Ok(Ping),
        ["command", "docs"] => Ok(Command(SubCommand::Docs)),
        ["echo", text] => Ok(Echo(text.to_string())),
        ["set", key, value] => Ok(Set(key.to_string(), value.to_string())),
        ["get", key] => Ok(Get(key.to_string())),
        _ => Err(format!("command not known: {:?}", strings)),
    }
}

pub fn format_response(response: &RespType) -> String {
    use RespType::*;

    match response {
        SimpleString(text) => format!("+{}\r\n", text),
        BulkString(text) => format!("${}\r\n{}\r\n", text.len(), text),
        Error(text) => format!("-{}\r\n", text),
        _ => todo!(),
    }
}
