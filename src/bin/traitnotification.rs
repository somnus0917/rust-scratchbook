fn main() {
    println!("helloworld");
    let ealert = emailnoti {
        email_address: String::from("somnus0917chen@hotmail.com"),
    };
    let smsalert = smsnoti {
        sms_address: String::from("111222333"),
    };
    trigger_alert(&ealert, 12);
    trigger_alert(&smsalert, 77);
}
pub trait notification {
    fn send(&self, message: &str);
}
pub struct emailnoti {
    pub email_address: String,
}
pub struct smsnoti {
    pub sms_address: String,
}
pub struct wechatnoti {
    pub wechat_address: String,
}
impl notification for emailnoti {
    fn send(&self, message: &str) {
        println!("send email to {}, content {}", self.email_address, message)
    }
}

impl notification for smsnoti {
    fn send(&self, message: &str) {
        println!("send sms to {}, content {}", self.sms_address, message)
    }
}
impl notification for wechatnoti {
    fn send(&self, message: &str) {
        println!(
            "send wechat to {}, content {}",
            self.wechat_address, message
        )
    }
}
pub fn trigger_alert(notifier: &impl notification, error_code: i32) {
    let msg = format!("alert error code :{}", error_code);
    notifier.send(&msg);
}
