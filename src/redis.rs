use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub struct Server {
    store: HashMap<String, Data>,
}

struct Data {
    value: String,
    inserted_at: Instant,
    expires_in: Option<Duration>,
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
            Set(key, value, options) => {
                let data = Data {
                    value: value.to_string(),
                    inserted_at: Instant::now(),
                    expires_in: options.px.map(|exp| Duration::from_millis(exp)),
                };
                self.store.insert(key.to_string(), data);

                SimpleString("OK".to_string())
            }
            Get(key) => match self.store.get(key) {
                Some(data) => {
                    if let Some(duration) = data.expires_in {
                        if Instant::now() > (data.inserted_at + duration) {
                            self.store.remove(key);

                            return BulkString("nil".to_string());
                        }
                    }

                    BulkString(data.value.to_string())
                }
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
    Set(String, String, SetOptions),
    Get(String),
}

#[derive(Debug)]
pub struct SetOptions {
    px: Option<u64>,
}

impl SetOptions {
    fn new() -> Self {
        SetOptions { px: None }
    }

    fn set_px(&mut self, value: u64) {
        self.px = Some(value);
    }
}

pub fn parse_command(strings: &Vec<String>) -> Result<RedisCmd, String> {
    use RedisCmd::*;

    let lower: Vec<String> = strings.iter().map(|s| s.to_lowercase()).collect();
    let lower_str: Vec<&str> = lower.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    match lower_str.as_slice() {
        ["ping"] => Ok(Ping),
        ["command", "docs"] => Ok(Command(SubCommand::Docs)),
        ["echo", text] => Ok(Echo(text.to_string())),
        ["set", key, value, rest @ ..] => {
            let mut set_options = SetOptions::new();

            let mut iter = rest.iter();
            loop {
                let option = iter.next();
                match option {
                    Some(&"px") => {
                        let value = iter.next().unwrap().parse().unwrap();
                        set_options.set_px(value);
                    }
                    Some(_) => continue,
                    None => break,
                }
            }

            Ok(Set(key.to_string(), value.to_string(), set_options))
        }
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
