use std::time::Instant;
use reqwest::Client;
use crate::push::PushConfig;
use crate::utility::CustomConfig;
use crate::show_orderlist::OrderResponse;
use crate::ticket::{*};



// 任务状态枚举
#[derive(Clone,Debug)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed(bool),
    Failed(String),
}

// 票务结果
#[derive(Clone)]
pub struct TicketResult {
    pub success: bool,
    pub order_id: Option<String>,
    pub message: Option<String>,
    pub ticket_info: TicketInfo,
    pub timestamp: Instant,
}

// 任务信息
pub enum Task {
    
    QrCodeLoginTask(QrCodeLoginTask),
    LoginSmsRequestTask(LoginSmsRequestTask),
    PushTask(PushTask),
    SubmitLoginSmsRequestTask(SubmitLoginSmsRequestTask),
    GetAllorderRequestTask(GetAllorderRequest),
    GetTicketInfoTask(GetTicketInfoTask),
    
}

// 任务请求枚举
pub enum TaskRequest {
    
    QrCodeLoginRequest(QrCodeLoginRequest),
    LoginSmsRequest(LoginSmsRequest),
    PushRequest(PushRequest),
    SubmitLoginSmsRequest(SubmitLoginSmsRequest),
    GetAllorderRequest(GetAllorderRequest),
    GetTicketInfoRequest(GetTicketInfoRequest),
}

// 任务结果枚举
#[derive(Clone)]
pub enum TaskResult {
    
    QrCodeLoginResult(TaskQrCodeLoginResult),
    LoginSmsResult(LoginSmsRequestResult),
    PushResult(PushRequestResult),
    SubmitSmsLoginResult(SubmitSmsLoginResult),
    GetAllorderRequestResult(GetAllorderRequestResult),
    GetTicketInfoResult(GetTicketInfoResult),
}

//请求project_id票详情
#[derive(Clone,Debug)]
pub struct GetTicketInfoRequest {
    pub task_id : String,
    pub project_id : String,
    pub client: Client,

}

#[derive(Clone,Debug)]
pub struct GetTicketInfoResult {
    pub task_id : String,
    pub ticket_info: TicketInfo,
    pub success: bool,
    pub message : String,
}

#[derive(Clone,Debug)]
pub struct GetTicketInfoTask {
    pub task_id : String,
    pub project_id : String,
    pub status: TaskStatus,
    pub start_time : Option<Instant>,
    pub client: Client,
}


#[derive(Clone)]
pub struct PushRequest{
    pub title: String,
    pub message: String,
    pub push_config: PushConfig,
    pub push_type : PushType,
}

//推送类型
#[derive(Clone,Debug)]
pub enum PushType {
    All,
    Bark,
    PushPlus,
    Fangtang,
    Dingtalk,
    WeChat,
    Smtp,
}

// 推送结果结构体
#[derive(Clone)]
pub struct PushRequestResult {
    pub task_id: String,
    pub success: bool,
    pub message: String,
    pub push_type: PushType,
}


#[derive(Clone)]
pub struct PushTask {
    pub task_id: String,
    pub title: String,
    pub message:String,
    pub push_type: PushType,
    pub status: TaskStatus,
    pub start_time: Option<Instant>,    
}

pub struct TicketTask {
    pub task_id: String,
    pub account_id: String,
    pub ticket_id: String,
    pub status: TaskStatus,
    pub start_time: Option<Instant>,
    pub result: Option<TicketResult>,
}

pub struct QrCodeLoginTask {
    pub task_id: String,
    pub qrcode_key: String,
    pub qrcode_url: String,
    pub status: TaskStatus,
    pub start_time: Option<Instant>,
    
}

pub struct LoginSmsRequestTask {
    pub task_id: String,
    pub phone : String,
    pub status: TaskStatus,
    pub start_time: Option<Instant>,
    
}

pub struct SubmitLoginSmsRequestTask {
    pub task_id: String,
    pub phone : String,
    pub code: String,
    pub captcha_key: String,
    pub status: TaskStatus,
    pub start_time: Option<Instant>,
}

//获取全部订单信息
pub struct GetAllorderRequest {
    pub task_id: String,
    pub client: Client,
    pub status: TaskStatus,
    pub cookies: String,
    pub account_id: String,
    pub start_time: Option<Instant>,
   
}

#[derive(Clone)]
pub struct GetAllorderRequestResult {
    pub task_id: String,
    pub account_id: String,
    pub success: bool,
    pub message: String,
    pub order_info: Option<OrderResponse>,
    pub timestamp: Instant,
}

pub struct GetAllorderTask {
    pub task_id: String,
    pub account_id: String,
    pub status: TaskStatus,
    pub start_time: Option<Instant>,
}


pub struct TicketRequest {
    pub ticket_id: String,
    pub account_id: String,
    // 其他请求参数...
}

pub struct QrCodeLoginRequest {
    pub qrcode_key: String,
    pub qrcode_url: String,
    pub user_agent: Option<String>,
}

pub struct LoginSmsRequest {
    pub phone: String,
    pub client: Client,
    pub custom_config: CustomConfig,
}

pub struct SubmitLoginSmsRequest {
    pub phone : String,
    pub code: String,
    pub captcha_key: String,
    pub client: Client,

}



#[derive(Clone)]
pub struct TaskTicketResult {
    pub task_id: String,
    pub account_id: String,
    pub result: Result<TicketResult, String>,
}

#[derive(Clone)]
pub struct TaskQrCodeLoginResult {
    pub task_id: String,
    pub status: crate::login::QrCodeLoginStatus,
    pub cookie: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct LoginSmsRequestResult {
    pub task_id: String,
    pub phone: String,
    pub success: bool,
    pub message: String,
}

#[derive(Clone)]
pub struct SubmitSmsLoginResult {
    pub task_id: String,
    pub phone: String,
    pub success: bool,
    pub message: String,
    pub cookie: Option<String>,
}
// 更新 TaskManager trait
pub trait TaskManager: Send + 'static {
    // 创建新的任务管理器
    fn new() -> Self where Self: Sized;
    
    // 提交任务
    fn submit_task(&mut self, request: TaskRequest) -> Result<String, String>;
    
    // 获取可用结果，返回 TaskResult 枚举
    fn get_results(&mut self) -> Vec<TaskResult>;
    
    // 取消任务
    fn cancel_task(&mut self, task_id: &str) -> Result<(), String>;

    // 获取任务状态
    fn get_task_status(&self, task_id: &str) -> Option<TaskStatus>;
     
     // 关闭任务管理器
    fn shutdown(&mut self);
}

pub const DISCLAIMER_TEXT_ENCODED: &str = "4p2k77iP5pys6aG555uu5a6M5YWo5YWN6LS55byA5rqQ77yM56aB5q2i5ZWG55So5oiW5pS26LS577yM5byA5Y+R5Zui6Zif5LiN5om/5ouF5Lu75L2V5rOV5b6L6LSj5Lu7";

pub fn TaskManager_debug() -> String {
    let bytes = base64::decode(DISCLAIMER_TEXT_ENCODED).unwrap_or_default();
    String::from_utf8(bytes).unwrap_or_else(|_| "本项目免费开源".to_string())
}