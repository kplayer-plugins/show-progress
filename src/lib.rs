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
        String::from("show-progress")
    }
    fn get_args(
        &mut self,
        _custom_args: std::collections::HashMap<String, String>,
    ) -> std::vec::Vec<std::string::String> {
        // get history message
        let history_message = kplayer::get_history_message(
            kplayer::proto::keys::EventMessageAction::EVENT_MESSAGE_ACTION_RESOURCE_CHECKED,
        );

        let mut duration_format = String::from("00:00:00");
        if history_message != "history cannot be found" {
            let value: serde_json::Value = serde_json::from_str(history_message.as_str()).unwrap();
            let duration_str = value
                .get("inputAttribute")
                .unwrap()
                .get("duration")
                .unwrap()
                .as_str()
                .unwrap();
            let duration_u64 = String::from(duration_str).parse::<u64>().unwrap();
            duration_format = format!(
                "{:0>2}:{:0>2}:{:0>2}",
                (duration_u64 / 3600) as i32,
                (duration_u64 % 3600 / 60) as i32,
                duration_u64 % 60
            );
        }

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
    fn get_allow_custom_args(&self) -> Vec<&'static str> {
        vec!["fontsize", "fontcolor", "fontfile", "x", "y"]
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
    fn validate_user_args(
        &self,
        _args: std::collections::HashMap<String, String>,
    ) -> std::result::Result<bool, &'static str> {
        for (key, value) in _args {
            // validate font file exist
            if key == "fontfile" {
                if !kplayer::util::os::file_exist(&value) {
                    self.print_log(
                        kplayer::util::os::PrintLogLevel::ERROR,
                        format!("font file not eixst: {}", &value).as_str(),
                    );
                    return Err("font file not exist");
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
            let duration_str = value
                .get("inputAttribute")
                .unwrap()
                .get("duration")
                .unwrap()
                .as_str()
                .unwrap();
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
