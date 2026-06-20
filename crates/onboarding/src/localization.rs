pub(crate) fn tr(text: &'static str) -> &'static str {
    match text {
        "Already have an account? " => "已有账号？",
        "Log in" => "登录",
        "Welcome to Warp" => "欢迎使用 Warp",
        "A modern terminal with state of the art agents built in." => "内置先进 Agent 的现代终端。",
        "Get started" => "开始使用",
        "How do you want to work?" => "你想怎样开始工作？",
        "Build faster with AI agents" => "使用 AI Agent 更快构建",
        "An agent-first experience with best in class terminal support. Get terminal and agent driven development AI features like:" => {
            "Agent 优先的体验，同时保留一流终端支持。你将获得终端和 Agent 驱动开发的 AI 功能，例如："
        },
        "Just use the terminal" => "只使用终端",
        "No AI features" => "不启用 AI 功能",
        "A modern terminal optimized for speed, context, and control without AI." => {
            "一个为速度、上下文和可控性优化的现代终端，不包含 AI 功能。"
        },
        "Back" => "返回",
        "Next" => "下一步",
        "Get Warping" => "开始使用 Warp",
        "Open a project" => "打开项目",
        "Set up a project to optimize it for coding in Warp." => {
            "设置一个项目，让它更适合在 Warp 中编写代码。"
        },
        "Open local folder" => "打开本地文件夹",
        "Skip" => "跳过",
        "Initialize project automatically" => "自动初始化项目",
        "Prepares the project environment, builds an index of your code, and generates project rules—giving the agent deeper understanding and better performance." => {
            "准备项目环境、建立代码索引并生成项目规则，让 Agent 获得更深入的理解和更好的表现。"
        },
        "Choose a theme" => "选择主题",
        "Click or use arrow keys to select, Enter to confirm." => "点击或使用方向键选择，按 Enter 确认。",
        "Sync light/dark theme with OS" => "跟随系统同步浅色/深色主题",
        "If you'd like to opt out of analytics, you can adjust your " => "如果你想关闭分析数据收集，可以调整",
        "Privacy Settings" => "隐私设置",
        "By continuing, you agree to Warp's " => "继续即表示你同意 Warp 的",
        "Terms of Service" => "服务条款",
        "Customize your Warp" => "自定义你的 Warp",
        "Tailor your features and UI to your working style." => "按照你的工作方式调整功能和界面。",
        "Tab styling" => "标签页样式",
        "Vertical" => "垂直",
        "Horizontal" => "水平",
        "File explorer" => "文件资源管理器",
        "Conversation history" => "对话历史",
        "Global file search" => "全局文件搜索",
        "Tools panel" => "工具面板",
        "Enabled" => "启用",
        "Disabled" => "禁用",
        "Code review" => "代码评审",
        "Choose your AI setup" => "选择你的 AI 设置",
        "Choose if you'd like to use Warp Agent or third party agents." => "选择使用 Warp Agent 或第三方 Agent。",
        "Use Warp Agent" => "使用 Warp Agent",
        "Access more models" => "访问更多模型",
        "State of the art agent harness deeply integrated into the terminal." => {
            "深度集成到终端中的先进 Agent 运行环境。"
        },
        "Best harness for terminal tasks and agentic coding" => "最适合终端任务和 Agent 编码的运行环境",
        "Frontier models from OpenAI, Anthropic, and Google" => "来自 OpenAI、Anthropic 和 Google 的前沿模型",
        "Model routing across frontier and open-weight models" => "在前沿模型和开放权重模型之间智能路由",
        "Multi-agent orchestration" => "多 Agent 编排",
        "Use third party agents" => "使用第三方 Agent",
        "Use agents like Claude Code, Codex, and Gemini." => "使用 Claude Code、Codex、Gemini 等 Agent。",
        "I don't want AI" => "我不想使用 AI",
        "Customize your Warp Agent" => "自定义你的 Warp Agent",
        "Select your Warp Agent's defaults." => "选择 Warp Agent 的默认设置。",
        "Recommended" => "推荐",
        "Default model" => "默认模型",
        "Autonomy" => "自主程度",
        "Set by Team Workspace" => "由团队工作区设置",
        "Autonomy settings are configured as part of your team workspace." => "自主程度设置由你的团队工作区配置。",
        "Full" => "完整",
        "Warp Agent runs commands, writes code, and reads files without asking." => {
            "Warp Agent 可在不询问的情况下运行命令、编写代码并读取文件。"
        },
        "Partial" => "部分",
        "Warp Agent can plan, read files, and execute low-risk commands. Asks before making any changes or executing sensitive commands." => {
            "Warp Agent 可以制定计划、读取文件并执行低风险命令；在修改内容或执行敏感命令前会询问。"
        },
        "None" => "无",
        "Warp Agent takes no actions without your approval." => "未经你批准，Warp Agent 不会执行任何操作。",
        "Customize third party agents" => "自定义第三方 Agent",
        "Select defaults for using agents like Claude Code, Codex, and Gemini." => {
            "为 Claude Code、Codex、Gemini 等 Agent 选择默认设置。"
        },
        "CLI agent toolbar" => "CLI Agent 工具栏",
        "Notifications" => "通知",
        "Choose how to access AI" => "选择如何访问 AI",
        "Save with a recurring plan, or use your own key or endpoint." => {
            "使用订阅套餐节省费用，或使用你自己的密钥或端点。"
        },
        "Subscription" => "订阅",
        "Best value" => "最划算",
        "Choose plan" => "选择套餐",
        "Use my own key or endpoint" => "使用自己的密钥或端点",
        "Use your own API key or OpenAI-compatible endpoint with Warp for free." => {
            "免费在 Warp 中使用你自己的 API 密钥或 OpenAI 兼容端点。"
        },
        "+ Add key" => "+ 添加密钥",
        "+ Add custom endpoint" => "+ 添加自定义端点",
        "Warp Agent requires a subscription or inference supplied by you" => "Warp Agent 需要订阅或由你提供推理能力",
        "Click here" => "点击这里",
        "If your browser hasn't launched, " => "如果浏览器没有打开，",
        " and open the page manually. " => "并手动打开页面。",
        " to paste your token from the browser." => "以粘贴浏览器中的令牌。",
        "Give me AI features" => "启用 AI 功能",
        "Are you sure you don't want AI?" => "确定不使用 AI 吗？",
        "Warp is better with AI. By continuing, you won't have access to any of the following features:" => {
            "启用 AI 后 Warp 会更强大。继续后，你将无法使用以下功能："
        },
        "Plan successfully activated!" => "套餐已成功激活！",
        "Starting at $18 / mo, available with monthly or annual plans. Includes base credits, frontier models, cloud agents, collaboration, and more." => {
            "月付或年付套餐每月 $18 起，包含基础额度、前沿模型、云端 Agent、协作等功能。"
        },
        _ => text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_core_onboarding_copy() {
        assert_eq!(tr("Welcome to Warp"), "欢迎使用 Warp");
        assert_eq!(tr("Get started"), "开始使用");
        assert_eq!(tr("Initialize project automatically"), "自动初始化项目");
        assert_eq!(tr("not mapped"), "not mapped");
    }
}
