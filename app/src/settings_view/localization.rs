use std::borrow::Cow;
use std::sync::atomic::{AtomicU8, Ordering};

use settings::Setting as _;
use warpui::{AppContext, SingletonEntity as _};

use crate::settings::{LanguageSettings, UserInterfaceLanguage};

static CURRENT_LANGUAGE: AtomicU8 = AtomicU8::new(UserInterfaceLanguage::English as u8);

pub(super) fn sync_from_settings(app: &AppContext) {
    set_language(*LanguageSettings::as_ref(app).user_interface_language.value());
}

pub(super) fn set_language(language: UserInterfaceLanguage) {
    CURRENT_LANGUAGE.store(language as u8, Ordering::Relaxed);
}

pub(super) fn tr(text: &str) -> Cow<'_, str> {
    if CURRENT_LANGUAGE.load(Ordering::Relaxed) != UserInterfaceLanguage::Chinese as u8 {
        return Cow::Borrowed(text);
    }

    chinese_settings_text(text)
        .map(Cow::Borrowed)
        .unwrap_or(Cow::Borrowed(text))
}

fn chinese_settings_text(text: &str) -> Option<&'static str> {
    Some(match text {
        "Account" => "账号",
        "Billing and usage" => "账单与用量",
        "Keyboard shortcuts" => "键盘快捷键",
        "Shared blocks" => "共享块",
        "MCP Servers" => "MCP 服务器",
        "MCP servers" => "MCP 服务器",
        "Scripting" => "脚本",
        "Warp Drive" => "Warp Drive",
        "Warp Agent" => "Warp Agent",
        "Profiles" => "配置档案",
        "Knowledge" => "知识库",
        "Third party CLI agents" => "第三方 CLI Agent",
        "Indexing and projects" => "索引与项目",
        "Editor and Code Review" => "编辑器与代码评审",
        "Environments" => "环境",
        "Oz Cloud API Keys" => "Oz Cloud API Keys",
        "Teams" => "团队",
        "Appearance" => "外观",
        "Features" => "功能",
        "Privacy" => "隐私",
        "Referrals" => "推荐",
        "About" => "关于",
        "Code" => "代码",
        "AI" => "AI",
        "Platform" => "平台",

        "Themes" => "主题",
        "Icon" => "图标",
        "Window" => "窗口",
        "Input" => "输入",
        "Panes" => "窗格",
        "Blocks" => "块",
        "Text" => "文本",
        "Cursor" => "光标",
        "Tabs" => "标签页",
        "Full-screen Apps" => "全屏应用",
        "General" => "通用",
        "Notifications" => "通知",
        "Session" => "会话",
        "Keys" => "按键",
        "Text Editing" => "文本编辑",
        "Terminal Input" => "终端输入",
        "Editor" => "编辑器",
        "Terminal" => "终端",
        "System" => "系统",
        "Language" => "语言",
        "Subshells" => "子 shell",
        "SSH" => "SSH",
        "Codebase Indexing" => "代码库索引",
        "Code Editor and Review" => "代码编辑器与评审",

        "Settings sync" => "设置同步",
        "Sign up" => "注册",
        "Log out" => "退出登录",
        "Free" => "免费",
        "Compare plans" => "比较套餐",
        "Contact support" => "联系支持",
        "Manage billing" => "管理账单",
        "Upgrade to Turbo plan" => "升级到 Turbo 套餐",
        "Upgrade to Lightspeed plan" => "升级到 Lightspeed 套餐",
        "Earn rewards by sharing Warp with friends & colleagues" => "与朋友和同事分享 Warp 以获取奖励",

        "Create a Custom Theme" => "创建自定义主题",
        "Current theme" => "当前主题",
        "App icon" => "应用图标",
        "Show dock icon" => "显示 Dock 图标",
        "Sync with OS" => "与操作系统同步",
        "Customize your app icon" => "自定义应用图标",
        "Show Warp in Dock" => "在 Dock 中显示 Warp",
        "Open windows at custom size" => "以自定义大小打开窗口",
        "Background opacity" => "背景不透明度",
        "Background blur" => "背景模糊",
        "Background blur texture" => "背景模糊纹理",
        "Zoom level" => "缩放级别",
        "Tools panel state scope" => "工具面板状态范围",
        "Input type" => "输入类型",
        "Prompt" => "提示符",
        "Input position" => "输入位置",
        "Dim inactive panes" => "调暗非活动窗格",
        "Focus follows mouse" => "焦点跟随鼠标",
        "Compact mode" => "紧凑模式",
        "Jump to bottom of block button" => "跳到块底部按钮",
        "Show block dividers" => "显示块分隔线",
        "AI font" => "AI 字体",
        "Terminal font" => "终端字体",
        "Notebook font size" => "Notebook 字号",
        "Thin strokes" => "细字重",
        "Minimum contrast" => "最低对比度",
        "Ligatures" => "连字",
        "Cursor type" => "光标类型",
        "Blinking cursor" => "光标闪烁",
        "Tab close button position" => "标签页关闭按钮位置",
        "Tab indicator" => "标签页指示器",
        "Code Review button" => "Code Review 按钮",
        "Preserve active tab color" => "保留活动标签页颜色",
        "Vertical tabs" => "垂直标签页",
        "Show vertical tab panel in restored windows" => "在恢复窗口中显示垂直标签页面板",
        "Hide title bar search bar in vertical tabs" => "在垂直标签页中隐藏标题栏搜索框",
        "Use latest user prompt as conversation title in tab names" => {
            "在标签页名称中使用最新用户提示作为会话标题"
        },
        "Edit toolbar" => "编辑工具栏",
        "Directory tab colors" => "目录标签页颜色",
        "Alt screen padding" => "备用屏幕内边距",
        "Tools panel visibility is consistent across tabs" => "工具面板可见性在标签页间保持一致",

        "Open links in desktop app" => "在桌面应用中打开链接",
        "Restore windows, tabs, and panes on startup" => "启动时恢复窗口、标签页和窗格",
        "Show sticky command header" => "显示固定命令标题",
        "Show tooltip on click on links" => "点击链接时显示工具提示",
        "Receive desktop notifications from Warp" => "接收来自 Warp 的桌面通知",
        "Notify when an agent completes a task" => "Agent 完成任务时通知",
        "Notify when a command or agent needs your attention to continue" => {
            "命令或 Agent 需要你关注才能继续时通知"
        },
        "Play notification sounds" => "播放通知声音",
        "Show in-app agent notifications" => "显示应用内 Agent 通知",
        "Confirm before closing shared session" => "关闭共享会话前确认",
        "Global hotkey:" => "全局热键：",

        "Send crash reports" => "发送崩溃报告",
        "Store AI conversations in the cloud" => "在云端存储 AI 对话",
        "Network log console" => "网络日志控制台",
        "View network logging" => "查看网络日志",

        "English" => "English",
        "中文" => "中文",
        "Never" => "从不",
        "Always" => "始终",
        "Normal" => "常规",
        "Bold" => "粗体",
        "Light" => "浅色",
        "Dark" => "深色",
        "Classic" => "经典",
        "Universal" => "通用",
        "Pinned to bottom" => "固定到底部",
        "Pinned to top" => "固定到顶部",
        "Waterfall" => "瀑布流",
        "Block" => "块状",
        "Underline" => "下划线",
        "Bar" => "竖线",
        "Left" => "左侧",
        "Right" => "右侧",
        "Reset" => "重置",
        "Add" => "添加",
        "Remove" => "移除",
        "Save" => "保存",
        "Cancel" => "取消",
        "Delete" => "删除",
        "Edit" => "编辑",
        "Enabled" => "已启用",
        "Disabled" => "已禁用",

        "Click to learn more in docs" => "点击在文档中了解更多",
        "This setting is not synced to your other devices" => "此设置不会同步到其他设备",
        "This option is enforced by your organization's settings and cannot be customized." => {
            "此选项由组织设置强制执行，无法自定义。"
        },
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_settings_text_for_selected_language() {
        set_language(UserInterfaceLanguage::Chinese);
        assert_eq!(tr("Appearance"), "外观");
        assert_eq!(tr("Settings sync"), "设置同步");
        assert_eq!(tr("openai"), "openai");
        assert_eq!(tr("Custom model name"), "Custom model name");

        set_language(UserInterfaceLanguage::English);
        assert_eq!(tr("Appearance"), "Appearance");
    }
}
