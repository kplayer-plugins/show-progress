extern crate kplayer_rust_wrap;
extern crate serde_json;

use kplayer_rust_wrap::kplayer;

struct ShowProgress {}
impl ShowProgress {
    fn new() -> Self {
        ShowProgress {}
    }
}

impl kplayer::plugin::BasePlugin for ShowProgress {
    fn get_name(&self) -> String {
        String::from("show-text")
    }
    fn get_args(&self) -> std::vec::Vec<std::string::String> {
        // get history message
        let history_message = kplayer::get_history_message(
            kplayer::proto::keys::EventMessageAction::EVENT_MESSAGE_ACTION_RESOURCE_CHECKED,
        );
        let value: serde_json::Value = serde_json::from_str(history_message.as_str()).unwrap();
        let duration_str = value.get("duration").unwrap().as_str().unwrap();
        let duration_u64 = String::from(duration_str).parse::<u64>().unwrap();

        let duration_format = format!(
            "{:0>2}:{:0>2}:{:0>2}",
            (duration_u64 / 3600) as i32,
            (duration_u64 % 3600 / 60) as i32,
            duration_u64 % 60
        );

        // set arg
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from(format!(
            "{}/{}",
            r#"text=%{pts:gmtime:0:%H\\:%M\\:%S}"#, duration_format
        )));
        args.push(String::from("fontsize=17"));
        args.push(String::from("fontcolor=white"));
        args.push(String::from("fontfile=resource/font.ttf"));
        args.push(String::from("x=0"));
        args.push(String::from("y=0"));

        args
    }
    fn get_author(&self) -> std::string::String {
        String::from("kplayer")
    }
    fn get_filter_name(&self) -> std::string::String {
        String::from("drawtext")
    }
    fn get_media_type(&self) -> kplayer::plugin::MediaType {
        kplayer::plugin::MediaType::MediaTypeVideo
    }
    fn validate_user_args(&self, _args: &Vec<String>) -> std::result::Result<bool, &'static str> {
        // get history message
        let history_message = kplayer::get_history_message(
            kplayer::proto::keys::EventMessageAction::EVENT_MESSAGE_ACTION_RESOURCE_CHECKED,
        );
        let value: serde_json::Value = serde_json::from_str(history_message.as_str()).unwrap();
        let duration_str = value.get("duration").unwrap().as_str().unwrap();
        let duration_u64 = String::from(duration_str).parse::<u64>().unwrap();

        let duration_format = format!(
            "{:0>2}:{:0>2}:{:0>2}",
            (duration_u64 / 3600) as i32,
            (duration_u64 % 3600 / 60) as i32,
            duration_u64 % 60
        );

        // set arg
        let validate_args = String::from(format!(
            "{}/{}",
            r#"%{pts:gmtime:0:%H\\:%M\\:%S}"#, duration_format
        ));

        for str in _args {
            let sp: Vec<&str> = str.split('=').collect();
            if sp.len() < 2 {
                self.print_log(
                    kplayer::util::os::PrintLogLevel::ERROR,
                    format!("validate args failed arg string: {}", str).as_str(),
                );
                return Err("args format error");
            }

            // validate font file exist
            if sp[0] == "fontfile" {
                if !kplayer::util::os::file_exist(sp[1].to_string()) {
                    self.print_log(
                        kplayer::util::os::PrintLogLevel::ERROR,
                        format!("font file not eixst: {}", str).as_str(),
                    );
                    return Err("font file not exist");
                }
                continue;
            }

            // validate text invalid
            if sp[0] == "text" {
                if sp[1] != validate_args {
                    return Err("text argument can not be custom");
                }
                continue;
            }
        }

        Ok(true)
    }
    fn register_message_keys(&self) -> Vec<kplayer::proto::keys::EventMessageAction> {
        let empty: Vec<kplayer::proto::keys::EventMessageAction> =
            vec![kplayer::proto::keys::EventMessageAction::EVENT_MESSAGE_ACTION_RESOURCE_CHECKED];
        empty
    }
    fn execute_message(&mut self, action: i32, body: String) {
        let start_value =
            kplayer::proto::keys::EventMessageAction::EVENT_MESSAGE_ACTION_RESOURCE_CHECKED as i32;
        if action == start_value {
            let value: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
            let duration_str = value.get("duration").unwrap().as_str().unwrap();
            let duration_u64 = String::from(duration_str).parse::<u64>().unwrap();
            let duration_format = format!(
                "{:0>2}:{:0>2}:{:0>2}",
                (duration_u64 / 3600) as i32,
                (duration_u64 % 3600 / 60) as i32,
                duration_u64 % 60
            );

            kplayer::util::core::update_args(
                String::from("text"),
                format!("{}/{}", r#"%{pts:gmtime:0:%H\\:%M\\:%S}"#, duration_format),
            )
            .unwrap();
        }
    }
}

kplayer_rust_wrap::export!(ShowProgress);
