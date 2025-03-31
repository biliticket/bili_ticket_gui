use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomConfig{
    pub open_custom_ua: bool, //是否开启自定义UA
    pub custom_ua: String,      //自定义UA
    pub chapcha_mode: usize,     //验证码模式   //0:本地打码  1：ttocr
    pub ttocr_key: String,      //ttocr key
    pub preinput_phone: String, //预填手机号
    

}

impl CustomConfig{
    pub fn new() -> Self{
        Self{
            open_custom_ua: true,
            custom_ua: String::from("Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Mobile Safari/537.36"),
            chapcha_mode: 0,
            ttocr_key: String::new(),
            preinput_phone: String::new(),
        }
    }
}