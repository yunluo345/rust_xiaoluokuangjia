# ReAct 范式实现计划

## 目标

让 AI 对话接口支持 ReAct（Reasoning + Acting）循环：AI 可以多轮调用工具，每轮执行完工具后将结果喂回 AI，直到 AI 判断任务完成并返回纯文本回复。

## 核心流程

```
用户消息 → AI 请求
    ↓
AI 返回 tool_calls? ──是──→ 执行工具 → 结果喂回 AI → 回到 AI 请求
    │
    否
    ↓
返回文本给用户
```

## 终止条件（AI 如何知道循环结束）

AI 自身决定终止，不需要后端判断。具体机制：

1. AI 返回 `tool_calls()` 为 `None` 且 `text()` 有值 → 循环结束，返回文本
2. AI 返回 `tool_calls()` 为 `Some` → 继续执行工具并循环
3. 后端设置最大循环次数（如 10 次）→ 超过则强制终止，返回错误提示
4. 在系统提示词中明确告知 AI：完成任务后直接用文本回复，不要再调用工具

## 改动计划

### 第一步：改造 openaizhuti.rs

新增方法 `putongqingqiu_react`：
- 接收 `&Aipeizhi` 和 `&mut Xiaoxiguanli`（注意是 `&mut`，因为循环中要追加消息）
- 单次调用 AI，返回 `Option<ReactJieguo>` 枚举：
  - `Wenben(String)` — AI 返回了纯文本
  - `Gongjudiaoyong(Vec<ToolCall>)` — AI 要求调用工具
- 重试逻辑保留在单次调用内部

### 第二步：改造 aixiaoxiguanli.rs

新增方法：
- `zhuijia_zhushou_gongjudiaoyong(&mut self, diaoyong: Vec<ToolCall>)` — 追加 assistant 的 tool_call 消息（告诉 AI 它之前请求了哪些工具）
- 现有的 `zhuijia_gongjujieguo` 已经可以追加工具执行结果

### 第三步：改造 jiekou_aiduihua.rs

在 `chuliqingqiu` 中实现 ReAct 循环：

```
常量 zuida_xunhuancishu = 10

loop (计数 <= zuida_xunhuancishu):
    调用 putongqingqiu_react
    match 结果:
        Wenben(文本) → 返回给用户，break
        Gongjudiaoyong(调用列表) →
            guanli.zhuijia_zhushou_gongjudiaoyong(调用列表)
            遍历每个调用，执行对应工具函数
            guanli.zhuijia_gongjujieguo(执行结果)
            继续循环
        None → 返回错误，break

超过最大次数 → 返回"AI处理超时"
```

### 第四步：定义 ReactJieguo 枚举

放在 `openaizhuti.rs` 中：

```
pub enum ReactJieguo {
    Wenben(String),
    Gongjudiaoyong(Vec<llm::ToolCall>),
}
```

## 文件改动清单

| 文件 | 改动 |
|------|------|
| `src/gongju/ai/openai/openaizhuti.rs` | 新增 `ReactJieguo` 枚举 + `putongqingqiu_react` 方法 |
| `src/gongju/ai/openai/aixiaoxiguanli.rs` | 新增 `zhuijia_zhushou_gongjudiaoyong` 方法 |
| `src/jiekouxt/jiekou_nr/ai/jiekou_aiduihua.rs` | `chuliqingqiu` 改为 ReAct 循环 |

## 不改动

- `jiekou_aiduihualiushi.rs`（流式接口暂不改，后续再适配）
- 不新增任何工具定义（本次只搭框架）
- 不新增文件，只改现有文件
