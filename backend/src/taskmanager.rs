use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use common::account;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use common::taskmanager::{*};
use common::login::{send_loginsms,sms_login};
use crate::show_orderlist::get_orderlist;
use crate::api::{*};


pub struct TaskManagerImpl {
    task_sender: mpsc::Sender<TaskMessage>,
    result_receiver: mpsc::Receiver<TaskResult>,
    running_tasks: HashMap<String, Task>, // 使用 Task 枚举
    runtime: Arc<Runtime>,
    _worker_thread: Option<thread::JoinHandle<()>>,
}

enum TaskMessage {
    SubmitTask(TaskRequest),
    CancelTask(String),
    Shutdown,
}

impl TaskManager for TaskManagerImpl {
    fn new() -> Self {
        // 创建通道
        let (task_tx, mut task_rx) = mpsc::channel(100);
        let (result_tx, result_rx) = mpsc::channel(100);
        
        // 创建tokio运行时
        let runtime = Arc::new(Runtime::new().unwrap());
        let rt = runtime.clone();
        
        // 启动工作线程
        let worker = thread::spawn(move || {
            rt.block_on(async {
                while let Some(msg) = task_rx.recv().await {
                    match msg {
                        TaskMessage::SubmitTask(request) => {
                            let task_id = uuid::Uuid::new_v4().to_string();
                            let result_tx = result_tx.clone();
                            
                            // 根据任务类型处理
                            match request {
                                
                                TaskRequest::QrCodeLoginRequest(qrcode_req) => {
                                    tokio::spawn(async move {
                                        // 二维码登录逻辑
                                        let status = poll_qrcode_login(&qrcode_req.qrcode_key,qrcode_req.user_agent.as_deref()).await;
                                        
                                        let (cookie, error) = match &status {
                                            common::login::QrCodeLoginStatus::Success(cookie) => 
                                                (Some(cookie.clone()), None),
                                            common::login::QrCodeLoginStatus::Failed(err) => 
                                                (None, Some(err.clone())),
                                            _ => (None, None)
                                        };
                                        
                                        // 创建正确的结果类型
                                        let task_result = TaskResult::QrCodeLoginResult(TaskQrCodeLoginResult {
                                            task_id,
                                            status,
                                            cookie,
                                            error,
                                        });
                                        
                                        let _ = result_tx.send(task_result).await;
                                    });
                                }
                                TaskRequest::LoginSmsRequest(login_sms_req) => {
                                    let task_id = uuid::Uuid::new_v4().to_string();
                                    let phone = login_sms_req.phone.clone();
                                    let client = login_sms_req.client.clone();
                                    let custom_config = login_sms_req.custom_config.clone();
                                    let result_tx = result_tx.clone();
                                    

                                    /* let client = match reqwest::Client::builder()
                                        .user_agent(user_agent.clone())
                                        .cookie_store(true)
                                        .build() {
                                            Ok(client) => client,
                                            Err(err) => {
                                               // 记录错误并发送错误结果
                                               log::error!("创建请求客户端失败 ID: {}, 错误: {}", task_id, err);
                
                                               let task_result = TaskResult::LoginSmsResult(LoginSmsRequestResult {
                                                    task_id,
                                                    phone,
                                                    success: false,
                                                    message: format!("创建客户端失败: {}", err),
                                                    });
                
                                               let _ = result_tx.send(task_result).await;
                                               return; 
                                               }
                                               }; */
                                    

                                    tokio::spawn(async move{
                                        log::info!("开始发送短信验证码 ID: {}", task_id);

                                        let result = async{
                                            log::info!("开始发送短信验证码 ID: {}", task_id);
                                            let response = send_loginsms(&phone, &client, custom_config).await;
                                            log::info!("开始发送短信验证码 ID: {}", task_id);
                                            let success = response.is_ok();
                                            let message = match &response {
                                                    Ok(msg) => msg.clone(),
                                                    Err(err) => {
                                                        log::error!("发送短信验证码失败: {}", err);
                                                        err.to_string()
                                                    },
                                                };
                                            log::info!("发送短信任务完成 ID: {}, 结果: {}", 
                                                task_id, 
                                                if success { "成功" } else { "失败" }
                                            );

                                            let task_result = TaskResult::LoginSmsResult(LoginSmsRequestResult {
                                                task_id,
                                                phone,
                                                success,
                                                message,
                                            });

                                            let _ = result_tx.send(task_result).await;

                                        }.await;
                                        


                                    });
                                
                                }
                                TaskRequest::PushRequest(push_req) => {
                                    let task_id = uuid::Uuid::new_v4().to_string();
                                    let push_config = push_req.push_config.clone();
                                    let title = push_req.title.clone();
                                    let message = push_req.message.clone();
                                    let push_type = push_req.push_type.clone();
                                    let result_tx = result_tx.clone();
                                    
                                    // 启动异步任务处理推送
                                    tokio::spawn(async move {
                                        log::info!("开始处理推送任务 ID: {}, 类型: {:?}", task_id, push_type);
                                        
                                        let (success, result_message) = match push_type {
                                            PushType::All => {
                                                push_config.push_all_async( &title, &message).await
                                            },
                                            
                                            // 其他推送类型的处理...
                                            _ => (false, "未实现的推送类型".to_string())
                                        };
                                        
                                        // 创建任务结果
                                        let task_result = TaskResult::PushResult(PushRequestResult {
                                            task_id: task_id.clone(),
                                            success,
                                            message: result_message,
                                            push_type: push_type.clone(),
                                        });
                                        
                                        // 发送结果
                                        if let Err(e) = result_tx.send(task_result).await {
                                            log::error!("发送推送任务结果失败: {}", e);
                                        }
                                        
                                        log::info!("推送任务 ID: {} 完成, 结果: {}", task_id, 
                                                  if success { "成功" } else { "失败" });
                                    });
                                    
                                 
                                }
                                TaskRequest::SubmitLoginSmsRequest(login_sms_req) => {
                                    let task_id = uuid::Uuid::new_v4().to_string();
                                    let phone = login_sms_req.phone.clone();
                                    let client = login_sms_req.client.clone();
                                    let captcha_key = login_sms_req.captcha_key.clone();
                                    let code = login_sms_req.code.clone();
                                    let result_tx = result_tx.clone();

                                    tokio::spawn(async move{
                                        log::info!("短信验证码登录进行中 ID: {}", task_id);
                                        
                                        let result = async{
                                            let response = sms_login(&phone,  &code,&captcha_key, &client).await;
                                            let success = response.is_ok();
                                            let message: String = match &response {
                                                    Ok(msg) => msg.clone(),
                                                    Err(err) => {
                                                        log::error!("提交短信验证码失败: {}", err);
                                                        err.to_string()
                                                    },
                                                };
                                            let cookie = match &response {
                                                Ok(msg) => Some(msg.clone()),
                                                Err(_) => None,
                                            };
                                            log::info!("提交短信任务完成 ID: {}, 结果: {}", 
                                                task_id, 
                                                if success { "成功" } else { "失败" }
                                            );

                                            let task_result = TaskResult::SubmitSmsLoginResult(SubmitSmsLoginResult {
                                                task_id,
                                                phone,
                                                success,
                                                message,
                                                cookie,
                                            });

                                            let _ = result_tx.send(task_result).await;

                                        }.await;
                                    });
                                }
                                TaskRequest::GetAllorderRequest(get_order_req) => {
                                    let client = get_order_req.client.clone();
                                    let result_tx = result_tx.clone();
                                    let task_id = get_order_req.task_id;
                                    let account_id = get_order_req.account_id.clone();
                                    let cookies = get_order_req.cookies.clone();
                                    tokio::spawn(async move{
                                        log::info!("正在获取全部订单 ID: {}", task_id);
                                        let response = get_orderlist(client,cookies.as_str()).await;
                                        let success = response.is_ok();
                                        let data = match &response {
                                            Ok(order_resp) => {order_resp.clone()},
                                            Err(err) => {
                                                log::error!("获取全部订单失败: {}", err);
                                                return;
                                            }
                                        };
                                        let message = match &response {
                                            Ok(msg) => {format!("获取全部订单成功: {}", msg.data.total)},
                                            Err(err) => {
                                                log::error!("获取全部订单失败: {}", err);
                                                err.to_string()
                                            },
                                        };

                                        let task_result = TaskResult::GetAllorderRequestResult(GetAllorderRequestResult {
                                            task_id: task_id.clone(),
                                            success,
                                            message,
                                            order_info: Some(data.clone()),
                                            account_id: account_id.clone(),
                                            timestamp: std::time::Instant::now(),
                                        });

                                        let _ = result_tx.send(task_result).await;
                                    });
                                }
                                TaskRequest::GetTicketInfoRequest(get_ticketinfo_req) => {
                                    let client = get_ticketinfo_req.client.clone();
                                    let task_id = get_ticketinfo_req.task_id.clone();
                                    let result_tx = result_tx.clone();
                                    let project_id = get_ticketinfo_req.project_id.clone();
                                    let uid = get_ticketinfo_req.uid.clone();
                                    tokio::spawn(async move{
                                        log::debug!("正在获取project{}",task_id);
                                        let response  = get_project(client, &project_id).await;
                                        let success = response.is_ok();
                                        let ticket_info = match &response{
                                            Ok(info) => {Some(info.clone())},
                                            Err(e) => {
                                                log::error!("获取项目时失败，原因：{}",e);
                                                None
                                            }
                                        };
                                        let message = match &response{
                                            Ok(info) => {
                                                //log::debug!("项目{}请求成功",info.errno);
                                                format!("项目{}请求成功",info.errno)
                                            }
                                            Err(e) => {
                                                e.to_string()
                                            }
                                        };
                                        let task_result = TaskResult::GetTicketInfoResult(GetTicketInfoResult{
                                            task_id : task_id.clone(),
                                            uid: uid.clone(),
                                            ticket_info : ticket_info.clone(),
                                            success : success,
                                            message : message.clone(),

                                        });
                                        let _ = result_tx.send(task_result).await;
                                    });
                                }
                                TaskRequest::GetBuyerInfoRequest(get_buyerinfo_req)=>{
                                    let client = get_buyerinfo_req.client.clone();
                                    let task_id = get_buyerinfo_req.task_id.clone();
                                    let result_tx = result_tx.clone();
                                    let uid = get_buyerinfo_req.uid.clone();
                                    tokio::spawn(async move{
                                        log::debug!("正在获取购票人信息{}",task_id);
                                        let response  = get_buyer_info(client).await;
                                        let success = response.is_ok();
                                        let buyer_info = match &response{
                                            Ok(info) => {Some(info.clone())},
                                            Err(e) => {
                                                log::error!("获取购票人信息失败，原因：{}",e);
                                                None
                                            }
                                        };
                                        let message = match &response{
                                            Ok(info) => {
                                                //log::debug!("项目{}请求成功",info.errno);
                                                format!("购票人信息请求成功")
                                            }
                                            Err(e) => {
                                                e.to_string()
                                            }
                                        };
                                        let task_result = TaskResult::GetBuyerInfoResult(GetBuyerInfoResult{
                                            task_id : task_id.clone(),
                                            uid: uid.clone(),
                                            buyer_info : buyer_info.clone(),
                                            success : success,
                                            message : message.clone(),

                                        });
                                        let _ = result_tx.send(task_result).await;
                                        
                                    });
                                }
                            }
                        },
                        TaskMessage::CancelTask(_task_id) => {
                            // 取消任务逻辑
                        },
                        TaskMessage::Shutdown => break,
                    }
                }
            });
        });
        
        Self {
            task_sender: task_tx,
            result_receiver: result_rx,
            running_tasks: HashMap::new(),
            runtime: runtime,
            _worker_thread: Some(worker),
        }
    }
    
    fn submit_task(&mut self, request: TaskRequest) -> Result<String, String> {
        // 生成任务ID
        let task_id = uuid::Uuid::new_v4().to_string();
        
        // 根据请求类型创建相应的任务
        match &request {
            
            TaskRequest::QrCodeLoginRequest(qrcode_req) => {
                log::info!("提交二维码登录任务 ID: {}", task_id);
                // 创建二维码登录任务
                let task = QrCodeLoginTask {
                    task_id: task_id.clone(),
                    qrcode_key: qrcode_req.qrcode_key.clone(),
                    qrcode_url: qrcode_req.qrcode_url.clone(),
                    status: TaskStatus::Pending,
                    start_time: Some(std::time::Instant::now()),
                };
                
                // 保存任务
                self.running_tasks.insert(task_id.clone(), Task::QrCodeLoginTask(task));
            }
            TaskRequest::LoginSmsRequest(login_sms_req) => {
                log::info!("提交短信验证码任务 ID: {}, 手机号: {}", task_id, login_sms_req.phone);
                
                // 创建短信任务
                let task = LoginSmsRequestTask {
                    task_id: task_id.clone(),
                    phone: login_sms_req.phone.clone(),
                    status: TaskStatus::Pending,
                    start_time: Some(std::time::Instant::now()),
                };
                
                // 保存任务
                self.running_tasks.insert(task_id.clone(), Task::LoginSmsRequestTask(task));
            }
            TaskRequest::PushRequest(push_req) => {
                log::info!("提交推送任务 ID: {}", task_id);
                // 创建推送任务
                let task = PushTask {
                    task_id: task_id.clone(),
                    push_type: push_req.push_type.clone(),  // 使用push_type
                    title: push_req.title.clone(),
                    message: push_req.message.clone(),
                    status: TaskStatus::Pending,
                    start_time: Some(std::time::Instant::now()),
                };
                
                // 保存任务
                self.running_tasks.insert(task_id.clone(), Task::PushTask(task));
            }

            TaskRequest::SubmitLoginSmsRequest(login_sms_req) => {
                log::info!("提交短信验证码登录任务 ID: {}, 手机号: {}", task_id, login_sms_req.phone);
                
                // 创建短信验证码登录任务
                let task = SubmitLoginSmsRequestTask {
                    task_id: task_id.clone(),
                    phone: login_sms_req.phone.clone(),
                    code: login_sms_req.code.clone(),
                    captcha_key: login_sms_req.captcha_key.clone(),
                    status: TaskStatus::Pending,
                    start_time: Some(std::time::Instant::now()),
                };
                
                // 保存任务
                self.running_tasks.insert(task_id.clone(), Task::SubmitLoginSmsRequestTask(task));
            }
            TaskRequest::GetAllorderRequest(get_order_req) => {
                log::info!("提交获取全部订单任务 ID: {}", task_id);
                
                // 创建获取全部订单任务
                let task = GetAllorderRequest {
                    task_id: task_id.clone(),
                    client: get_order_req.client.clone(),
                    status: TaskStatus::Pending,
                    cookies: get_order_req.cookies.clone(),
                    account_id: get_order_req.account_id.clone(),
                    start_time: Some(std::time::Instant::now()),
                };
                
                // 保存任务
                self.running_tasks.insert(task_id.clone(), Task::GetAllorderRequestTask(task));
            }
            TaskRequest::GetTicketInfoRequest(get_ticketinfo_req) => {
                log::info!("{}",task_id);
                let task = GetTicketInfoTask{
                    task_id : task_id.clone(),
                    project_id: get_ticketinfo_req.project_id.clone(),
                    status: TaskStatus::Running,
                    start_time: Some(std::time::Instant::now()),
                    client: get_ticketinfo_req.client.clone(), 
                };
                self.running_tasks.insert(task_id.clone(),Task::GetTicketInfoTask(task));
            }
            TaskRequest::GetBuyerInfoRequest(get_buyerinfo_req) => {
                log::info!("提交获取购票人信息任务 ID: {}", task_id);
                
                //创建任务
                let task = GetBuyerInfoTask {
                    uid: get_buyerinfo_req.uid.clone(),
                    task_id: task_id.clone(),
                    client: get_buyerinfo_req.client.clone(),
                    status: TaskStatus::Pending,
                    start_time: Some(std::time::Instant::now()),
                    
                };
                
                // 保存任务
                self.running_tasks.insert(task_id.clone(), Task::GetBuyerInfoTask(task));
            }

        }
        
        // 发送任务
        if let Err(e) = self.task_sender.blocking_send(TaskMessage::SubmitTask(request)) {
            return Err(format!("无法提交任务: {}", e));
        }
        
        Ok(task_id)
    }
    
    fn get_results(&mut self) -> Vec<TaskResult> {
        let mut results = Vec::new();
        
        // 非阻塞方式获取所有可用结果
        while let Ok(result) = self.result_receiver.try_recv() {
            results.push(result);
        }
        
        results
    }
    
    fn cancel_task(&mut self, task_id: &str) -> Result<(), String> {
        if !self.running_tasks.contains_key(task_id) {
            return Err("任务不存在".to_string());
        }
        
        if let Err(e) = self.task_sender.blocking_send(TaskMessage::CancelTask(task_id.to_owned())) {
            return Err(format!("无法取消任务: {}", e));
        }
        
        Ok(())
    }
    
    fn get_task_status(&self, task_id: &str) -> Option<TaskStatus> {
        if let Some(task) = self.running_tasks.get(task_id) {
            match task {
                
                Task::QrCodeLoginTask(t) => Some(t.status.clone()),
                Task::LoginSmsRequestTask(t) => Some(t.status.clone()),
                Task::PushTask(t) => Some(t.status.clone()),
                Task::SubmitLoginSmsRequestTask(t) => Some(t.status.clone()),
                Task::GetAllorderRequestTask(t) => Some(t.status.clone()),
                Task::GetTicketInfoTask(t) => Some(t.status.clone()),
                Task::GetBuyerInfoTask(t) => Some(t.status.clone()),
            }
        } else {
            None
        }
    }
    
    fn shutdown(&mut self) {
        let _ = self.task_sender.blocking_send(TaskMessage::Shutdown);
        if let Some(handle) = self._worker_thread.take() {
            let _ = handle.join();
        }
    }
}

