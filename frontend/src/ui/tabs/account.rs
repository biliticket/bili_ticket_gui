use eframe::egui;
use crate::app::Myapp;

pub fn render(app: &mut Myapp, ui: &mut egui::Ui){
    ui.heading("我的账户");
    ui.separator();
    

    // 加载用户头像
    
    load_user_avatar(ui.ctx(), app);
    /* if let Some(texture) = &app.user_info.avatar_texture {
        draw_circular_image(ui, texture, 180.0);
    } else if let Some(default_texture) = &app.default_avatar_texture {
        draw_circular_image(ui, default_texture, 180.0);
    } */
    if let Some(texture) = &app.default_avatar_texture {
    rounded_rect_with_image_and_text(
        ui,
        texture,
        if app.user_info.is_logged { &app.user_info.username } else { "未登录" },
        if app.user_info.is_logged { 
             app.user_info.show_info.as_str()
        } else { 
            "点击登录以使用完整功能" 
            
        }
        
        
    );
    if !app.user_info.is_logged {
        
        let button = egui::Button::new(
            egui::RichText::new("登录").size(20.0).color(egui::Color32::WHITE)
        )
        .min_size(egui::vec2(30.0, 15.0))
        .fill(egui::Color32::from_rgb(66, 150, 250))
        .rounding(20.0);
         if ui.add(button).clicked() {
        app.is_loading = true;
        
    }
    }
}

    /* // 账号信息卡片
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(240, 240, 250)) // 浅蓝色背景
        .rounding(15.0) // 圆角效果
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 230))) // 边框
        .inner_margin(egui::style::Margin::same(16.0)) // 内边距
        .show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                // 头像容器（圆形）
let avatar_size = 80.0;
let (rect, _) = ui.allocate_exact_size(
    egui::Vec2::new(avatar_size, avatar_size),
    egui::Sense::hover()
);

// 获取头像纹理
let avatar_texture = if app.user_info.is_logged && app.user_info.avatar_texture.is_some() {
    &app.user_info.avatar_texture
} else {
    &app.default_avatar_texture
};

if let Some(texture) = avatar_texture {
    // 创建圆形裁剪区域
    ui.painter().rect_filled(
        rect,
        avatar_size / 2.0, // 完全圆形的圆角
        egui::Color32::from_rgb(220, 220, 240) // 背景色
    );
    
    // 在裁剪区域内绘制图像
    let clip_rect = ui.painter().clip_rect();
    let clip_shape = egui::Shape::circle_filled(
        rect.center(), 
        avatar_size / 2.0 - 1.0, // 微小边距
        egui::Color32::WHITE
    );
    let new_clip_rect = clip_rect.intersect(rect); // 创建交集
    
    ui.painter().with_clip_rect(new_clip_rect).add(clip_shape);
    ui.painter().with_clip_rect(new_clip_rect).image(
        texture.id(),
        rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE
    );
} else {
    // 如果没有纹理，只绘制背景圆形
    ui.painter().circle_filled(
        rect.center(),
        avatar_size / 2.0,
        egui::Color32::from_rgb(220, 220, 240)
    );
}
                
                ui.add_space(16.0);
                
                // 用户信息
                ui.vertical(|ui| {
                    let username = if app.user_info.is_logged { 
                        &app.user_info.username 
                    } else { 
                        "未登录" 
                    };
                    
                    ui.heading(username);
                    
                    if app.user_info.is_logged {
                        // 已登录用户信息
                        ui.horizontal(|ui| {
                            ui.label("LV");
                            ui.add(egui::widgets::Label::new(
                                egui::RichText::new(format!("{}", app.user_info.level))
                                    .color(egui::Color32::from_rgb(66, 150, 250))
                                    .strong()
                            ));
                        });
                        
                        // 会员标签
                        egui::Frame::none()
                            .rounding(4.0)
                            .fill(egui::Color32::from_rgb(249, 204, 40)) // 金色
                            .inner_margin(egui::style::Margin::symmetric(6.0, 2.0))
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("哔哩大会员").color(egui::Color32::BLACK).small());
                            });
                    } else {
                        // 未登录提示
                        ui.label(egui::RichText::new("请登录以使用完整功能").color(egui::Color32::GRAY));
                        
                        if ui.button(egui::RichText::new("立即登录").strong()).clicked() {
                            // TODO: 实现登录功能
                            show_login_dialog(app);
                        }
                    }
                });
            });
        });
        
    // 用户卡片下方的其他信息
    if app.user_info.is_logged {
        ui.add_space(16.0);
        
        egui::CollapsingHeader::new("账号详情")
            .default_open(false)
            .show(ui, |ui| {
                ui.label("UID: 10086123");
                ui.label("注册时间: 2021-05-01");
                ui.label("绑定手机: 138****8888");
                ui.label("安全等级: 高");
            });
        
        ui.add_space(8.0);
        
        if ui.button("退出登录").clicked() {
            app.user_info.is_logged = false;
        }
    } */
}
/// 将任意图片显示为圆形
/// - texture: 要显示的图像纹理
/// - size: 圆形图片的直径大小
fn draw_circular_image(ui: &mut egui::Ui, texture: &egui::TextureHandle, size: f32) -> egui::Response {
    // 分配正方形区域
    let (rect, response) = ui.allocate_exact_size(
        egui::Vec2::new(size, size),
        egui::Sense::click()
    );
    
    if ui.is_rect_visible(rect) {
        // 创建一个离屏渲染的自定义形状层
        let layer_id = egui::layers::LayerId::new(
            egui::layers::Order::Middle, 
            egui::Id::new("circular_image")
        );
        
        let painter = ui.ctx().layer_painter(layer_id);
        
        // 绘制圆形背景 (这一步可选)
        painter.circle_filled(
            rect.center(), 
            size / 2.0,
            egui::Color32::from_rgb(220, 220, 240)
        );
        
        // 使用圆形纹理蒙版技术
        // 1. 创建一个与图像大小相同的圆形遮罩
        let circle_mask = egui::Shape::circle_filled(
            rect.center(), 
            size / 2.0 - 1.0,
            egui::Color32::WHITE
        );
        
        // 2. 将图像绘制为自定义着色器，使用圆形遮罩
        let uv = egui::Rect::from_min_max(
            egui::pos2(0.0, 0.0),
            egui::pos2(1.0, 1.0)
        );
        
        // 使用裁剪圆绘制
        painter.add(circle_mask);
        
        // 以混合模式绘制图像，只在圆形区域内可见
        painter.image(
            texture.id(),
            rect,
            uv,
            egui::Color32::WHITE
        );
        
        // 添加边框
        painter.circle_stroke(
            rect.center(),
            size / 2.0,
            egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(180, 180, 180, 180))
        );
    }
    
    response
}
fn load_texture_from_path(ctx: &egui::Context, path: &str, name: &str) -> Option<egui::TextureHandle> {
    use std::io::Read;
    
    match std::fs::File::open(path) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            if file.read_to_end(&mut bytes).is_ok() {
                match image::load_from_memory(&bytes) {
                    Ok(image) => {
                        let size = [image.width() as usize, image.height() as usize];
                        let image_buffer = image.to_rgba8();
                        let pixels = image_buffer.as_flat_samples();
                        
                        Some(ctx.load_texture(
                            name,
                            egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
                            Default::default()
                        ))
                    }
                    Err(_) => None,
                }
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn load_user_avatar(ctx: &egui::Context, app: &mut Myapp) {
    // 确保默认头像已加载（作为回退选项）
    if app.default_avatar_texture.is_none() {
        load_default_avatar(ctx, app);
    }
    
    // 如果用户已登录且提供了头像路径，尝试加载
    if app.user_info.is_logged && app.user_info.avatar_texture.is_none() {
        if let Some(avatar_path) = &app.user_info.avatar_path {
            // 尝试加载用户头像
            app.user_info.avatar_texture = load_texture_from_path(ctx, avatar_path, "user_avatar");
            
            // 如果加载失败，记录日志
            if app.user_info.avatar_texture.is_none() {
                println!("无法加载用户头像: {}", avatar_path);
                // 用户也可以在这里添加一个日志
                app.add_log(&format!("无法加载头像: {}", avatar_path));
            }
        }
    }
}
// 加载默认头像
fn load_default_avatar(ctx: &egui::Context, app: &mut Myapp) {
    // 使用include_bytes!宏将图片直接嵌入到二进制文件中
    // 路径是相对于项目根目录的
    let default_avatar_bytes = include_bytes!("../../../assets/default_avatar.jpg");
    
    // 从内存中加载图片
    match image::load_from_memory(default_avatar_bytes) {
        Ok(image) => {
            let size = [image.width() as usize, image.height() as usize];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            
            app.default_avatar_texture = Some(ctx.load_texture(
                "default_avatar",
                egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
                Default::default()
            ));
        },
        Err(_) => {
            // 图片加载失败，生成占位符头像
            app.default_avatar_texture = generate_placeholder_avatar(ctx);
        }
    }
}

// 生成一个占位符头像
fn generate_placeholder_avatar(ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let size = 128; // 头像尺寸
    let mut image_data = vec![0; size * size * 4];
    
    // 生成一个简单的渐变图案
    for y in 0..size {
        for x in 0..size {
            let i = (y * size + x) * 4;
            // 浅蓝色调渐变
            image_data[i] = 180; // R
            image_data[i + 1] = 180 + (y as u8) / 2; // G
            image_data[i + 2] = 230; // B
            image_data[i + 3] = 255; // A
        }
    }
    
    Some(ctx.load_texture(
        "default_avatar",
        egui::ColorImage::from_rgba_unmultiplied([size, size], &image_data),
        Default::default()
    ))
}

fn rounded_rect_with_image_and_text(
    ui: &mut egui::Ui, 
    texture: &egui::TextureHandle, 
    title: &str,
    description: &str
) {
    // 创建圆角长方形框架
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(245, 245, 250))  // 背景色
        .rounding(12.0)  // 圆角半径
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 220)))  // 边框
        .inner_margin(egui::style::Margin::symmetric(16.0, 12.0))  // 内边距
        .show(ui, |ui| {
            // 水平布局放置图片和文字
            ui.horizontal(|ui| {
                // 左侧图片区域，这里使用小尺寸的圆形图片
                let image_size = 64.0;
                draw_circular_image(ui, texture, image_size);
                
                ui.add_space(12.0);  // 图片和文字之间的间距
                
                // 右侧文字区域
                ui.vertical(|ui| {
                    // 标题文字
                    ui.add(egui::widgets::Label::new(
                        egui::RichText::new(title)
                            .size(18.0)
                            .strong()
                            .color(egui::Color32::from_rgb(60, 60, 80))
                    ));
                    
                    // 描述文字
                    ui.label(
                        egui::RichText::new(description)
                            .color(egui::Color32::from_rgb(100, 100, 120))
                            .size(14.0)
                    );
                });
            });
        });
}
// 显示登录对话框（仅为示例，需要实现具体功能）
fn show_login_dialog(app: &mut Myapp) {
    // 这里可以触发登录弹窗或其他操作
    // 实际项目中，可能是显示一个新窗口或与后端交互
    println!("显示登录对话框");


}