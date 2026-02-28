# 后端AI调用总结

## 一、AI底层基础设施

### 1.1 AI配置（Aipeizhi）
- **文件**: `src/gongju/ai/openai/aipeizhi.rs`
- **功能**: AI调用的核心配置结构体，包含：接口地址、密钥、模型名、温度、最大token、超时时间、重试次数
- **支持的AI类型**: OpenAI、Claude/Anthropic、DeepSeek、Google/Gemini、Ollama、Groq、XAI
- **配置来源**: 从数据库AI渠道表中随机获取可用渠道（`shujucaozuo_aiqudao::suiji_huoqu_qudao`）

### 1.2 消息管理器（Xiaoxiguanli）
- **文件**: `src/gongju/ai/openai/aixiaoxiguanli.rs`
- **功能**: 管理对话上下文（系统提示词、消息列表、工具列表），支持上下文裁剪防止超token

### 1.3 OpenAI主体（openaizhuti）
- **文件**: `src/gongju/ai/openai/openaizhuti.rs`
- **提供的AI调用方式**:
  - `putongqingqiu` — 非流式调用，仅返回文本
  - `putongqingqiu_daisikao` — 非流式调用，返回文本+思考过程
  - `putongqingqiu_react` — 非流式ReAct单次调用，返回文本或工具调用
  - `liushiqingqiu` — 流式调用，返回字节流响应
- **内置能力**: HTTP重试+指数退避、响应体限流检测、`<tool_call>`标签格式兼容解析

### 1.4 通用AI工具（gongyong）
- **文件**: `src/gongju/ai/openai/feiduihuagongju/gongyong.rs`
- **功能**:
  - `ai_putongqingqiu_wenben` — 封装通用AI文本请求（获取渠道配置→构建消息→调用AI）
  - `jinghua_json_huifu` — 清洗AI返回的markdown代码块标记
  - `anzi_fenduan` — 按字符数分段（支持重叠），用于长文本分段处理
  - `yanzheng_bitian_biaoqian` — 验证必填标签是否齐全

### 1.5 AI JSON执行器（ai_zhixingqi）
- **文件**: `src/gongju/ai/openai/feiduihuagongju/ai_zhixingqi.rs`
- **功能**: 统一AI JSON执行管道（提示词→AI调用→JSON清洗→反序列化校验），被跨日报分析功能使用

---

## 二、前端对话类接口（直接与AI交互）

### 2.1 AI对话接口（非流式）
- **接口路径**: `POST /ai/duihua`
- **文件**: `src/jiekouxt/jiekou_nr/ai/jiekou_aiduihua.rs`
- **AI调用流程**:
  1. 获取AI渠道配置（`huoqu_peizhi`）
  2. 意图分析：AI分析用户消息→判断"工具调用"或"普通对话"，提取关键词（调用`putongqingqiu_daisikao`）
  3. 上下文摘要：取最近N条有效消息压缩为摘要（调用`putongqingqiu`）
  4. 工具筛选：根据意图关键词匹配相关工具
  5. ReAct循环：多轮"AI思考→工具调用→反馈结果"直到AI返回最终文本（调用`putongqingqiu_react`）
- **涉及的AI调用次数**: 每次对话最少3次AI调用（摘要+意图分析+最终回复），有工具调用时更多

### 2.2 AI对话接口（流式）
- **接口路径**: `POST /ai/duihualiushi`
- **文件**: `src/jiekouxt/jiekou_nr/ai/jiekou_aiduihualiushi.rs`
- **AI调用流程**: 与非流式相同，但最终回复通过SSE逐字推送给前端
- **额外特性**: 实时推送意图分析结果、工具调用事件、思考过程

### 2.3 系统提示词
- **文件**: `src/jiekouxt/jiekou_nr/ai/mod.rs`
- **三套提示词**:
  - `xitongtishici` — AI日报助手的角色定义和行为规范
  - `yitu_tishici` — 意图分析助手，判断"工具调用"vs"普通对话"
  - `zhaiyao_tishici` — 上下文摘要助手，压缩对话历史

---

## 三、AI工具集（ReAct循环中被AI调用的工具）

### 3.1 工具注册中心
- **文件**: `src/gongju/ai/openai/gongjuji/mod.rs`
- **已注册的4个工具**:

| 工具名 | 功能 | 是否调用AI |
|--------|------|-----------|
| `shijian_chaxun` | 查询服务器时间 | ❌ 不调用AI |
| `aiqudao_guanli` | AI渠道增删改查 | ❌ 不调用AI |
| `ribao_jiancha` | 日报审核+提交 | ✅ 调用AI提取标签 |
| `ribao_renwubiaoqian_chuli` | 日报标签任务处理 | ✅ 调用多次AI |

### 3.2 日报检查工具（ribao_jiancha）
- **文件**: `src/gongju/ai/openai/gongjuji/ribao/gongju_ribaojiancha.rs`
- **AI调用**: 当用户提交的是纯文本日报（非JSON格式）时，调用AI提取标签信息和生成标题
- **调用函数**: `tiqu_biaoqian` → `openaizhuti::putongqingqiu`
- **超时**: 30秒，重试1次

### 3.3 日报任务处理工具（ribao_renwubiaoqian_chuli）
- **文件**: `src/gongju/ai/openai/gongjuji/ribao/gongju_ribaorenwuchuli.rs`
- **关联文件**: `src/gongju/ai/openai/feiduihuagongju/renwuchuli.rs`
- **AI调用**:  调度数据库中的待处理任务，每个任务触发多次AI调用（详见第四节）

---

## 四、非对话类AI调用（后台自动处理）

### 4.1 任务处理核心流程（renwuchuli + renwubuzhou）
- **文件**: `src/gongju/ai/openai/feiduihuagongju/renwuchuli.rs`、`renwubuzhou.rs`
- **触发方式**: 日报提交后自动创建标签任务 → 调度器或手动触发处理
- **6个步骤中有3个调用AI**:

#### 步骤2：AI标签提取
- **文件**: `src/gongju/ai/openai/feiduihuagongju/biaoqiantiqu.rs`
- **函数**: `ai_tiqu_biaoqian`
- **调用方式**: `openaizhuti::putongqingqiu`
- **功能**: 从日报内容中提取标签（我方人员、对方人员、项目名等）
- **超时**: 60秒，重试1次
- **降级策略**: AI失败时降级为字符串匹配（`tichubiaoqianxiang`）

#### 步骤5：AI内容丰富（4个AI任务并发执行）
并发调用4个AI生成任务：

**a) AI标题生成**
- **文件**: `src/gongju/ai/openai/feiduihuagongju/aishengcheng.rs`
- **函数**: `ai_shengcheng_biaoti`
- **超时**: 30秒

**b) AI摘要生成**
- **文件**: `src/gongju/ai/openai/feiduihuagongju/aishengcheng.rs`
- **函数**: `ai_shengcheng_zhaiyao`
- **超时**: 60秒

**c) AI思维导图生成**
- **文件**: `src/gongju/ai/openai/feiduihuagongju/aishengcheng.rs`
- **函数**: `ai_shengcheng_siweidaotu`
- **超时**: 120秒
- **输出**: JSON格式思维导图，维度由配置文件定义（客户分析、员工表现、工作内容、风险与待办）

**d) AI关系分析**
- **文件**: `src/gongju/ai/openai/feiduihuagongju/guanxifenxi.rs`
- **函数**: `ai_shengcheng_guanxifenxi`
- **超时**: 60秒/段
- **特殊处理**: 长文本自动分段（默认2500字/段，300字重叠），分段结果合并去重
- **输出**: 人物关系JSON（ren1、ren2、关系类型、描述、置信度、证据片段、情感倾向）

### 4.2 跨日报分析（kuaribaofenxi）
- **文件**: `src/gongju/ai/openai/feiduihuagongju/kuaribaofenxi.rs`
- **业务入口**: `src/yewu/ribao_fenxi/fenxi_yongli.rs`

#### a) 交流内容分析
- **函数**: `ai_jiaoliu_fenxi`
- **接口操作**: `fenxi_jiaoliu_neirong`
- **功能**: 输入按时间排列的交流记录 → AI输出结构化分析JSON
- **超时**: 120秒

#### b) 深度分析
- **函数**: `ai_ribao_shendu_fenxi`
- **接口操作**: `fenxi_ai_shendu`
- **功能**: 输入日报原文+分析维度 → AI按维度进行深度分析
- **超时**: 180秒

#### c) 项目关联分析 / 实体关联分析
- **函数**: `ai_xiangmu_guanlian_fenxi`
- **接口操作**: `fenxi_xiangmu_guanlian`、`fenxi_shiti_guanlian`
- **功能**: 输入多项目标签聚合数据 → AI输出项目间关联关系
- **超时**: 120秒

#### d) 深度关联分析（综合关联）
- **函数**: `ai_guanlian_shendu_fenxi`
- **接口操作**: `fenxi_zonghe_guanlian`
- **功能**: 标签聚合+日报原文+用户自定义提示 → AI深度关联分析
- **超时**: 240秒

---

## 五、调度器、并发控制与队列机制分析

### 5.1 现状总结：没有统一调度器，没有全局AI并发计数

**当前系统不存在**：
- ❌ 全局AI调用计数器（不知道当前有多少个AI请求正在执行）
- ❌ 全局AI并发限流器（没有 Semaphore/令牌桶/滑动窗口等限流机制）
- ❌ 统一的AI调用队列（各个调用点独立发起请求，互不感知）
- ❌ AI调用监控/统计（无法知道一段时间内总共调了多少次AI、花了多少token）

**各AI调用链路完全独立，互不感知**，具体表现如下：

### 5.2 三条独立的AI调用链路

```
链路A：AI对话（用户主动触发）
  前端请求 → /ai/duihua 或 /ai/duihualiushi
  → 每个HTTP请求独立执行，无排队、无并发限制
  → 每次对话可能产生3~20+次AI调用（摘要+意图+ReAct多轮）
  → 多个用户同时对话 = 同时N个ReAct循环各自跑

链路B：日报标签任务调度器（后台自动/手动触发）
  日报提交 → 创建任务 → 调度器处理
  → 有一个简单的单实例调度器（AtomicBool互斥）
  → 调度器内部用 buffer_unordered(bingfa) 控制并发
  → 但只控制「任务级」并发，不控制「AI调用级」并发
  → 每个任务内部5次AI调用是串行+并发混合（标签提取串行，标题/摘要/导图/关系并发）

链路C：跨日报分析（前端管理员触发）
  前端请求 → /ribao/guanli 的 fenxi_* 操作
  → 每个请求独立执行，无排队、无并发限制
  → 与链路A、链路B完全无关联
```

### 5.3 唯一存在的调度器：标签任务调度器

**文件**: `src/shujuku/psqlshujuku/shujubiao_nr/ribao/shujucaozuo_ribao_biaoqianrenwu.rs`

这是系统中**唯一的调度/队列机制**，但只管标签任务，不管AI调用：

**调度器特性**：
- **单实例保证**: 通过 `static yunxingzhong: OnceLock<AtomicBool>` 全局原子标志，`compare_exchange` 保证同一时刻只有一个调度器实例运行
- **任务队列**: 基于PostgreSQL表 `ribao_biaoqianrenwu`，状态机为：`false`(等待) → `processing`(处理中) → `true`(成功) / `shibai`(失败)
- **原子领取**: 使用 `FOR UPDATE SKIP LOCKED` 实现无锁并发领取，避免重复处理
- **并发度配置**: `ribao_biaoqianrenwu_bingfashuliang`（默认1），通过 `stream::buffer_unordered(bingfa)` 控制
- **熔断机制**: 连续5批全部失败时触发熔断停止，指数退避（2^n秒，最大60秒）
- **重试机制**: 每个任务有 `changshicishu` / `zuidachangshicishu`（默认3次），失败后自动回到等待队列
- **可停止**: 通过 `tingzhi()` 设置原子标志，调度循环检测后退出

**调度器局限**：
- 只管「任务粒度」的并发，不管「AI调用粒度」的并发
- 并发度=1时，一个任务内部的步骤5仍会并发4个AI调用（`futures::join!`）
- 不感知链路A（对话）和链路C（分析）的AI调用

### 5.4 各调用点的并发行为详情

#### AI对话（链路A）
- **并发控制**: 无。每个HTTP请求独立处理，actix-web框架层的连接数是唯一限制
- **ReAct循环**: 最大轮数由配置 `zuida_xunhuancishu`（默认20）限制
- **重复检测**: 相同工具调用签名（hash比较）连续出现超过阈值（非流式1次，流式2次）时，移除工具强制输出文本
- **上下文裁剪**: `caijian_shangxiawen` 按token上限裁剪历史消息，防止超长

#### 标签任务（链路B）
- **任务队列**: PostgreSQL表，支持状态流转和重试
- **并发控制**: `buffer_unordered(bingfa)` 控制同时处理的任务数
- **任务内部并发**: 步骤5用 `futures::join!` 并发4个AI调用，步骤2串行1个AI调用
- **实际AI并发数**: 若 bingfa=N，峰值 AI 并发 = N×4（每个任务步骤5的4个并发） + N×1（步骤2如果正好也在跑）

#### 日报提交异步启动（链路B的特殊入口）
- **文件**: `src/gongju/ai/openai/gongjuji/ribao/gongju_ribaotijiao.rs`
- **行为**: 日报提交后，通过 `actix_web::rt::spawn` 异步启动单个任务处理
- **与调度器的关系**: 如果调度器已在运行，只入队不启动；调度器未运行时，直接 spawn 处理单个任务（绕过调度器）

#### 跨日报分析（链路C）
- **并发控制**: 无。每个分析请求直接发起1次AI调用，无排队

### 5.5 HTTP层面的限流：仅有响应体限流检测

**文件**: `src/gongju/ai/openai/openaizhuti.rs` → `shifou_xianliu_xiangying`

- 当AI供应商返回HTTP 429或响应体含限流信息时，会指数退避重试（最多3次，每次5×n秒）
- 这是**被动限流**（等供应商告诉你被限了才退避），不是**主动限流**（提前控制发送速率）
- HTTP层面的重试由 `fasong_qingqiu` 统一处理（`chongshicishu` 次重试，429时退避更长）

### 5.6 潜在风险

1. **AI调用洪峰**: 多用户同时对话 + 调度器批量处理 + 管理员触发分析 = 三条链路同时向AI供应商发请求，可能触发限流
2. **费用不可控**: 没有全局AI调用计数/费用预算，无法知道一天用了多少token
3. **资源竞争**: 三条链路共用同一个AI渠道池，高峰期互相抢渠道
4. **无优先级**: 用户实时对话和后台批处理任务使用相同优先级，批量任务可能拖慢对话响应

### 5.7 并发控制相关配置项汇总

| 配置项 | 默认值 | 作用范围 | 说明 |
|--------|--------|---------|------|
| `ribao_biaoqianrenwu_bingfashuliang` | 1 | 标签任务调度器 | 同时处理几个任务（每个任务内还会并发4个AI） |
| `bingxingrenwushu` | 5 | 配置中存在但未找到使用处 | 可能预留的并发参数 |
| `zuida_xunhuancishu` | 20 | AI对话ReAct循环 | 单次对话最大AI调用轮数 |
| `chongshicishu`（Aipeizhi） | 0 | 单次HTTP请求 | AI HTTP请求失败重试次数 |
| `chaoshishijian`（Aipeizhi） | 240秒 | 单次HTTP请求 | AI HTTP请求超时 |
| `ribao_biaoqianrenwu_chongshi_cishu` | 3 | 标签任务 | 单个任务最大尝试次数 |

---

## 六、接口与AI调用关系汇总

### 直接对话接口（前端用户触发）
| 接口路径 | 操作 | AI调用说明 |
|----------|------|-----------|
| `POST /ai/duihua` | AI对话 | 意图分析+摘要生成+ReAct多轮对话 |
| `POST /ai/duihualiushi` | AI流式对话 | 同上，SSE推送 |

### 日报管理接口中的AI操作（管理员触发）
| 接口路径 | 操作名 | AI调用说明 |
|----------|--------|-----------|
| `POST /ribao/guanli` | `renwu_biaoqian_ai_chuli` | 批量调度标签任务（每个任务5次AI调用） |
| `POST /ribao/guanli` | `renwu_dange_chuli` | 单个任务处理（5次AI调用） |
| `POST /ribao/guanli` | `fenxi_jiaoliu_neirong` | 交流内容AI分析（1次AI调用） |
| `POST /ribao/guanli` | `fenxi_ai_shendu` | 深度维度AI分析（1次AI调用） |
| `POST /ribao/guanli` | `fenxi_shiti_guanlian` | 实体关联AI分析（1次AI调用） |
| `POST /ribao/guanli` | `fenxi_xiangmu_guanlian` | 项目关联AI分析（1次AI调用） |
| `POST /ribao/guanli` | `fenxi_zonghe_guanlian` | 综合关联AI分析（1次AI调用） |

### 日报提交时自动触发（用户无感知）
| 触发方式 | AI调用说明 |
|----------|-----------|
| 日报提交（通过AI对话工具`ribao_jiancha`） | 1次AI提取标签 |
| 日报提交后异步任务处理 | 5次AI（标签提取+标题+摘要+思维导图+关系分析） |

---

## 七、AI配置体系

### 7.1 AI渠道管理
- **数据库表**: `ai_qudao`（AI渠道表）
- **管理方式**: 通过AI对话工具`aiqudao_guanli`或系统接口`/xitong/aiqudao`管理
- **选择策略**: 按优先级随机选取启用的渠道

### 7.2 AI行为配置
- **配置文件**: `peizhi/ai.json`（通过 `peizhi_ai.rs` 加载）
- **关键配置项**:
  - `zuida_xunhuancishu` — ReAct循环最大轮数（默认20）
  - `ribao_biaoqian` — 标签定义（名称、描述、是否必填、是否多值、别称）
  - `siweidaotu_weidu` — 思维导图分析维度
  - `guanxifenxi_tishici` — 关系分析提示词
  - `guanxifenxi_danpian_zifushangxian` — 单篇字符上限（默认4000）
  - `guanxifenxi_fenduan_daxiao` — 分段大小（默认2500）
  - `jiaoliu_fenxi_tishici` — 交流分析提示词
  - `shendu_fenxi_tishici` — 深度分析提示词
  - `xiangmu_guanlian_tishici` — 项目关联分析提示词
  - `guanlian_shendu_tishici` — 关联深度分析提示词
  - `biaoti_shengcheng_tishici` — 标题生成提示词
  - `zhaiyao_shengcheng_tishici` — 摘要生成提示词

---

## 八、文件索引

| 模块 | 文件路径 | 说明 |
|------|----------|------|
| AI配置 | `src/gongju/ai/openai/aipeizhi.rs` | AI调用配置结构体 |
| AI通用 | `src/gongju/ai/aitongyonggongju.rs` | 网关地址补全 |
| OpenAI主体 | `src/gongju/ai/openai/openaizhuti.rs` | HTTP请求发送、响应解析 |
| 消息管理 | `src/gongju/ai/openai/aixiaoxiguanli.rs` | 对话上下文管理 |
| 通用工具 | `src/gongju/ai/openai/feiduihuagongju/gongyong.rs` | AI文本请求、JSON清洗、分段 |
| AI执行器 | `src/gongju/ai/openai/feiduihuagongju/ai_zhixingqi.rs` | 统一AI JSON执行管道 |
| 标签提取 | `src/gongju/ai/openai/feiduihuagongju/biaoqiantiqu.rs` | AI提取日报标签 |
| 内容生成 | `src/gongju/ai/openai/feiduihuagongju/aishengcheng.rs` | 标题/摘要/思维导图生成 |
| 关系分析 | `src/gongju/ai/openai/feiduihuagongju/guanxifenxi.rs` | 人物关系AI分析 |
| 跨日报分析 | `src/gongju/ai/openai/feiduihuagongju/kuaribaofenxi.rs` | 交流/深度/关联分析 |
| 任务处理 | `src/gongju/ai/openai/feiduihuagongju/renwuchuli.rs` | 任务调度+执行 |
| 任务步骤 | `src/gongju/ai/openai/feiduihuagongju/renwubuzhou.rs` | 6步骤拆分 |
| 工具集 | `src/gongju/ai/openai/gongjuji/mod.rs` | 工具注册+分发 |
| 日报检查工具 | `src/gongju/ai/openai/gongjuji/ribao/gongju_ribaojiancha.rs` | AI审核日报 |
| 日报任务工具 | `src/gongju/ai/openai/gongjuji/ribao/gongju_ribaorenwuchuli.rs` | AI标签任务处理 |
| 日报提交 | `src/gongju/ai/openai/gongjuji/ribao/gongju_ribaotijiao.rs` | 提交日报+自动启动任务 |
| AI对话接口 | `src/jiekouxt/jiekou_nr/ai/mod.rs` | 意图分析、ReAct循环 |
| 非流式对话 | `src/jiekouxt/jiekou_nr/ai/jiekou_aiduihua.rs` | POST /ai/duihua |
| 流式对话 | `src/jiekouxt/jiekou_nr/ai/jiekou_aiduihualiushi.rs` | POST /ai/duihualiushi |
| 日报管理接口 | `src/jiekouxt/jiekou_nr/ribao/jiekou_ribao.rs` | 管理员日报操作(含AI分析) |
| 用户日报接口 | `src/jiekouxt/jiekou_nr/ribao/jiekou_ribao_yonghu.rs` | 普通用户日报操作 |
| 分析用例 | `src/yewu/ribao_fenxi/fenxi_yongli.rs` | 交流分析/深度分析/关联分析 |
| AI行为配置 | `src/peizhixt/peizhi_nr/peizhi_ai.rs` | AI配置结构体+默认值 |
