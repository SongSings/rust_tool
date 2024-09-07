#[macro_use]
extern crate cocoa;
#[macro_use]
extern crate objc;
extern crate notify_rust;

use cocoa::appkit::{NSApp, NSApplication, NSMenu, NSMenuItem, NSStatusBar, NSStatusItem, NSWindow};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use notify_rust::Notification;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};

fn main() {
    unsafe {
        // 创建自动释放池
        let _pool = NSAutoreleasePool::new(nil);

        // 初始化应用程序
        let app = NSApp();
        app.setActivationPolicy_(cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);

        // 获取状态栏并创建一个状态栏项
        let status_item = NSStatusBar::systemStatusBar(nil).statusItemWithLength_(-1.0);

        // 设置状态栏图标（使用 Unicode 字符作为图标）
        let icon = NSString::alloc(nil).init_str("🟢");
        status_item.button().setTitle_(icon);

        // 创建菜单
        let menu = NSMenu::new(nil).autorelease();

        // 创建显示通知菜单项
        let item_show_notification: id = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            NSString::alloc(nil).init_str("显示通知"),
            sel!(showNotification),
            NSString::alloc(nil).init_str(""),
        ).autorelease();

        // 创建退出菜单项
        let item_quit: id = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            NSString::alloc(nil).init_str("退出"),
            sel!(terminate:),
            NSString::alloc(nil).init_str(""),
        ).autorelease();

        // 将菜单项添加到菜单中
        menu.addItem_(item_show_notification);
        menu.addItem_(item_quit);

        // 将菜单连接到状态栏项
        status_item.setMenu_(menu);

        // 创建自定义类，用来处理菜单项的回调
        let app_delegate = create_delegate();

        // 为菜单项设置回调目标
        let _: () = msg_send![item_show_notification, setTarget: app_delegate]; // 设置显示通知回调目标
        let _: () = msg_send![item_quit, setTarget: app];                       // 退出目标为 app

        // 启动应用程序事件循环
        app.run();
    }
}

// 创建一个 Objective-C 类来处理显示通知的操作
fn create_delegate() -> id {
    unsafe {
        // 创建自定义 Objective-C 类
        let superclass = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new("AppDelegate", superclass).unwrap();

        // 添加 `showNotification` 方法
        decl.add_method(
            sel!(showNotification),
            show_notification as extern "C" fn(&Object, Sel),
        );

        // 注册类并返回实例
        let delegate_class = decl.register();
        let delegate: id = msg_send![delegate_class, new];
        delegate
    }
}

// 显示通知的回调函数
extern "C" fn show_notification(_self: &Object, _cmd: Sel) {
    Notification::new()
        .summary("状态栏工具通知")
        .body("这是一个通过 Rust 弹出的 macOS 通知。")
        .show()
        .unwrap();
}
