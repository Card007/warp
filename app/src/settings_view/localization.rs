use std::borrow::Cow;
use std::sync::atomic::{AtomicU8, Ordering};

use settings::Setting as _;
use warpui::{AppContext, SingletonEntity as _};

use crate::settings::{LanguageSettings, UserInterfaceLanguage};

static CURRENT_LANGUAGE: AtomicU8 = AtomicU8::new(UserInterfaceLanguage::Chinese as u8);

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
        "Zoom" => "缩放",
        "Adjusts the default zoom level across all windows" => "调整所有窗口的默认缩放级别",
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
        "Agent font" => "Agent 字体",
        "Terminal font" => "终端字体",
        "Notebook font size" => "Notebook 字号",
        "Thin strokes" => "细字重",
        "Use thin strokes" => "使用细字重",
        "Minimum contrast" => "最低对比度",
        "Enforce minimum contrast" => "强制最低对比度",
        "Ligatures" => "连字",
        "Show ligatures in terminal" => "在终端中显示连字",
        "Cursor type" => "光标类型",
        "Blinking cursor" => "光标闪烁",
        "Tab close button position" => "标签页关闭按钮位置",
        "Tab indicator" => "标签页指示器",
        "Show tab indicators" => "显示标签页指示器",
        "Code Review button" => "Code Review 按钮",
        "Show code review button" => "显示代码评审按钮",
        "Preserve active tab color" => "保留活动标签页颜色",
        "Preserve active tab color for new tabs" => "为新标签页保留活动标签页颜色",
        "Vertical tabs" => "垂直标签页",
        "Use vertical tab layout" => "使用垂直标签页布局",
        "Show vertical tab panel in restored windows" => "在恢复窗口中显示垂直标签页面板",
        "Show vertical tabs panel in restored windows" => "在恢复窗口中显示垂直标签页面板",
        "Hide title bar search bar in vertical tabs" => "在垂直标签页中隐藏标题栏搜索框",
        "Hide search bar in vertical tab layout" => "在垂直标签页布局中隐藏搜索栏",
        "Use latest user prompt as conversation title in tab names" => {
            "在标签页名称中使用最新用户提示作为会话标题"
        },
        "Edit toolbar" => "编辑工具栏",
        "Header toolbar layout" => "标题栏工具布局",
        "Directory tab colors" => "目录标签页颜色",
        "Automatically color tabs based on the directory or repo you're working in." => {
            "根据当前工作目录或仓库自动为标签页着色。"
        },
        "Alt screen padding" => "备用屏幕内边距",
        "Use custom padding in alt-screen" => "在备用屏幕中使用自定义内边距",
        "Uniform padding (px)" => "统一内边距（px）",
        "Tools panel visibility is consistent across tabs" => "工具面板可见性在标签页间保持一致",
        "Line height" => "行高",
        "Font weight" => "字体粗细",
        "Font size (px)" => "字号（px）",
        "Reset to default" => "重置为默认值",
        "Match terminal" => "匹配终端",
        "View all available system fonts" => "查看所有可用系统字体",
        "Create your own custom theme" => "创建你自己的自定义主题",
        "Open new windows with custom size" => "使用自定义尺寸打开新窗口",
        "Open windows at custom size" => "以自定义大小打开窗口",
        "Columns" => "列",
        "Rows" => "行",
        "Window Opacity:" => "窗口不透明度：",
        "Use Window Blur (Acrylic texture)" => "使用窗口模糊（亚克力纹理）",

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
        "App analytics" => "应用分析",
        "Crash reporting" => "崩溃报告",
        "Secret redaction" => "敏感信息遮盖",
        "Cloud AI conversation storage" => "云端 AI 对话存储",

        "Copy on select" => "选中即复制",
        "Copy on select within the terminal" => "在终端中选中即复制",
        "Linux selection clipboard" => "Linux 选择剪贴板",
        "Autocomplete quotes, parentheses, and brackets" => "自动补全引号、圆括号和方括号",
        "Scroll reporting" => "滚动上报",
        "Mouse reporting" => "鼠标上报",
        "Focus reporting" => "焦点上报",
        "Completions while typing" => "输入时显示补全",
        "Command corrections" => "命令纠错",
        "Error underlining" => "错误下划线",
        "Syntax highlighting" => "语法高亮",
        "Audible terminal bell" => "终端提示音",
        "Autosuggestions" => "自动建议",
        "Autosuggestion keybinding hint" => "自动建议快捷键提示",
        "Autosuggestion ignore button" => "自动建议忽略按钮",
        "Mouse middle-click paste" => "鼠标中键粘贴",
        "Middle-click paste" => "中键粘贴",
        "Code as default editor" => "将 Code 设为默认编辑器",
        "Input hint text" => "输入提示文本",
        "Editing commands with Vim keybindings" => "使用 Vim 快捷键编辑命令",
        "Vim unnamed register as system clipboard" => "将 Vim 未命名寄存器作为系统剪贴板",
        "Vim status bar" => "Vim 状态栏",
        "Smart select" => "智能选择",
        "Terminal input message line" => "终端输入消息行",
        "'@' context menu in terminal mode" => "终端模式中的“@”上下文菜单",
        "Preserve input focus on block selection" => "选择块时保留输入焦点",
        "Slash commands in terminal mode" => "终端模式中的斜杠命令",
        "Codebase symbols in the '@' context menu" => "“@”上下文菜单中的代码库符号",
        "Global workflows in Command Search" => "命令搜索中的全局工作流",
        "Integrated GPU rendering (low power)" => "集成 GPU 渲染（低功耗）",
        "After all tabs" => "所有标签页之后",
        "After current tab" => "当前标签页之后",
        "Default" => "默认",
        "Completions" => "补全",
        "Autosuggestions" => "自动建议",
        "User defined" => "用户自定义",

        "Active AI" => "活动 AI",
        "Terminal command autodetection in agent input" => "Agent 输入中的终端命令自动检测",
        "Natural language detection" => "自然语言检测",
        "Agent prompt autodetection in terminal input" => "终端输入中的 Agent 提示自动检测",
        "Next Command" => "下一条命令",
        "Prompt suggestions" => "提示建议",
        "Code suggestions" => "代码建议",
        "Show agent tips" => "显示 Agent 提示",
        "Hide agent tips" => "隐藏 Agent 提示",
        "Natural language autosuggestions" => "自然语言自动建议",
        "Shared block title generation" => "共享块标题生成",
        "Commit and pull request generation" => "提交和拉取请求生成",
        "Voice input" => "语音输入",
        "Show \"Use Agent\" footer" => "显示“使用 Agent”页脚",
        "Hide \"Use Agent\" footer" => "隐藏“使用 Agent”页脚",
        "Include agent-executed commands in history" => "将 Agent 执行的命令纳入历史记录",
        "Conversation history in tools panel" => "工具面板中的对话历史",
        "Model picker in prompt" => "提示符中的模型选择器",
        "Coding agent toolbar" => "编码 Agent 工具栏",
        "Rules" => "规则",
        "Suggested Rules" => "建议规则",
        "Warp Drive as agent context" => "将 Warp Drive 作为 Agent 上下文",
        "Auto-spawn servers from third-party agents" => "从第三方 Agent 自动启动服务器",
        "Warp credit fallback" => "Warp 积分兜底",
        "Auto show or hide Rich Input based on agent status" => "根据 Agent 状态自动显示或隐藏 Rich Input",
        "Auto open Rich Input when a coding agent session starts" => {
            "编码 Agent 会话开始时自动打开 Rich Input"
        },
        "Auto dismiss Rich Input after prompt submission" => "提交提示后自动关闭 Rich Input",
        "Codebase index" => "代码库索引",
        "Auto-indexing" => "自动索引",
        "Auto open code review panel" => "自动打开代码评审面板",
        "Code review button" => "代码评审按钮",
        "Diff stats on code review button" => "代码评审按钮上的差异统计",
        "Project explorer" => "项目资源管理器",
        "Global file search" => "全局文件搜索",
        "Show hidden files in project explorer" => "在项目资源管理器中显示隐藏文件",

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
        "Pin to the bottom (Warp mode)" => "固定到底部（Warp 模式）",
        "Pin to the top (Reverse mode)" => "固定到顶部（反向模式）",
        "Start at the top (Classic mode)" => "从顶部开始（经典模式）",
        "Block" => "块状",
        "Underline" => "下划线",
        "Bar" => "竖线",
        "Left" => "左侧",
        "Right" => "右侧",
        "On low-DPI displays" => "在低 DPI 显示器上",
        "On high-DPI displays" => "在高 DPI 显示器上",
        "Only for named colors" => "仅用于命名颜色",
        "When windowed" => "窗口模式时",
        "Only on hover" => "仅悬停时",
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
        "When enabled, reopening or restoring a window opens the vertical tabs panel even if it was closed when the window was last saved." => {
            "启用后，重新打开或恢复窗口时会打开垂直标签页面板，即使上次保存窗口时它处于关闭状态。"
        },
        "When using the vertical tab layout, hide the search bar in the title bar. Search stays available via the command palette and keyboard shortcuts." => {
            "使用垂直标签页布局时，在标题栏中隐藏搜索栏。仍可通过命令面板和快捷键搜索。"
        },
        "Show the latest user prompt instead of the generated conversation title for Oz and third-party agent sessions in vertical tabs." => {
            "在垂直标签页中，为 Oz 和第三方 Agent 会话显示最新用户提示，而不是生成的会话标题。"
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
        assert_eq!(tr("Reset to default"), "重置为默认值");
        assert_eq!(tr("Use vertical tab layout"), "使用垂直标签页布局");
        assert_eq!(tr("openai"), "openai");
        assert_eq!(tr("Custom model name"), "Custom model name");

        set_language(UserInterfaceLanguage::English);
        assert_eq!(tr("Appearance"), "Appearance");
    }
}
