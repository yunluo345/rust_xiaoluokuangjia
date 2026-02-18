# AI意图理解工具设计方案

## 一、核心问题

当前AI工具系统存在的问题：
- 工具数量增多后，AI难以准确选择合适的工具
- 所有工具同时开放，增加AI决策复杂度
- 缺乏意图分析机制，导致工具调用效率低下

## 二、解决方案

### 2.1 核心思路

实现**两阶段工具调用机制**：

```
阶段1：意图理解
用户请求 → AI（仅意图理解工具） → 意图分析 → 返回意图类型

阶段2：工具执行  
意图类型 → 动态注册相关工具 → AI（相关工具） → 执行操作
```

### 2.2 意图分类体系

| 意图类型 | 说明 | 开放工具 |
|---------|------|---------|
| `aiqudao_guanli` | AI渠道管理相关操作 | `aiqudao_guanli` |
| `shijian_chaxun` | 时间查询相关 | `shijian_chaxun` |
| `putong_duihua` | 普通对话，无需工具 | 无 |
| `hunhe_caozuo` | 需要多个工具配合 | 根据分析结果开放多个工具 |

## 三、技术实现

### 3.1 文件结构

```
src/gongju/ai/openai/gongjuji/
├── mod.rs                          # 工具注册主模块（需修改）
├── gongju_yitulilei.rs            # 新增：意图理解工具
├── gongju_shijianchaxun.rs        # 现有：时间查询工具
└── gongju_aiqudaoguanli.rs        # 现有：AI渠道管理工具
```

### 3.2 意图理解工具设计

#### 工具定义
```rust
pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function",
        function: FunctionTool {
            name: "yitu_lilei",
            description: "分析用户请求的意图类型，必须首先调用此工具理解用户需求",
            parameters: {
                "type": "object",
                "properties": {
                    "yonghushuru": {
                        "type": "string",
                        "description": "用户的原始输入内容"
                    }
                },
                "required": ["yonghushuru"]
            }
        }
    }
}
```

#### 执行逻辑
```rust
pub async fn zhixing(canshu: &str, lingpai: &str) -> String {
    // 1. 解析参数获取用户输入
    // 2. 关键词匹配分析意图
    // 3. 返回意图类型和建议开放的工具列表
    
    返回格式：
    {
        "yituleixing": "aiqudao_guanli",
        "xinxidu": 0.95,
        "jianyi_gongjulie": ["aiqudao_guanli"],
        "fenxi": "用户想要管理AI渠道"
    }
}
```

### 3.3 工具注册机制改造

#### 当前机制（mod.rs）
```rust
fn suoyouzhuce() -> Vec<Gongjuzhuce> {
    vec![
        时间查询工具,
        AI渠道管理工具,
        // 所有工具一次性注册
    ]
}
```

#### 改造后机制
```rust
// 阶段1：仅返回意图理解工具
pub fn huoqu_yitugongju() -> Vec<Tool> {
    vec![gongju_yitulilei::dinyi()]
}

// 阶段2：根据意图类型返回相关工具
pub fn huoqu_xiangguangongju(yituleixing: &str) -> Vec<Tool> {
    match yituleixing {
        "aiqudao_guanli" => vec![gongju_aiqudaoguanli::dinyi()],
        "shijian_chaxun" => vec![gongju_shijianchaxun::dinyi()],
        "hunhe_caozuo" => vec![
            gongju_aiqudaoguanli::dinyi(),
            gongju_shijianchaxun::dinyi(),
        ],
        _ => vec![]
    }
}
```

### 3.4 消息管理改造

#### Xiaoxiguanli 新增方法
```rust
impl Xiaoxiguanli {
    // 新增：清空工具列表
    pub fn qingkong_gongjulie(&mut self) {
        self.gongjulie.clear();
    }
    
    // 新增：批量添加工具
    pub fn piliang_tianjia_gongju(&mut self, gongjulie: Vec<Tool>) {
        self.gongjulie.extend(gongjulie);
    }
    
    // 新增：替换工具列表
    pub fn tihuan_gongjulie(&mut self, gongjulie: Vec<Tool>) {
        self.gongjulie = gongjulie;
    }
}
```

### 3.5 对话流程改造

#### 接口层调用流程（jiekou_aiduihua.rs）
```rust
// 阶段1：意图理解
let mut guanli = Xiaoxiguanli::xingjian()
    .shezhi_xitongtishici("你是AI助手")
    .piliang_tianjia_gongju(gongjuji::huoqu_yitugongju()); // 仅意图工具

guanli.zhuijia_yonghuxiaoxi(yonghu_shuru);

// 调用AI进行意图理解
let yitu_jieguo = openaizhuti.react_xunhuan(&mut guanli, &peizhi).await?;

// 解析意图结果
let yituleixing = jiexi_yitu(yitu_jieguo);

// 阶段2：根据意图开放工具
guanli.tihuan_gongjulie(gongjuji::huoqu_xiangguangongju(&yituleixing));

// 继续对话执行具体操作
let zuizhong_jieguo = openaizhuti.react_xunhuan(&mut guanli, &peizhi).await?;
```

## 四、意图识别算法

### 4.1 关键词匹配规则

```rust
fn shibie_yitu(yonghushuru: &str) -> String {
    let shuru_xiaoxie = yonghushuru.to_lowercase();
    
    // AI渠道管理相关
    if shuru_xiaoxie.contains("渠道") 
        || shuru_xiaoxie.contains("ai") && (
            shuru_xiaoxie.contains("查询") 
            || shuru_xiaoxie.contains("新增")
            || shuru_xiaoxie.contains("删除")
            || shuru_xiaoxie.contains("更新")
        ) {
        return "aiqudao_guanli".to_string();
    }
    
    // 时间查询相关
    if shuru_xiaoxie.contains("时间") 
        || shuru_xiaoxie.contains("现在几点")
        || shuru_xiaoxie.contains("当前时间") {
        return "shijian_chaxun".to_string();
    }
    
    // 默认普通对话
    "putong_duihua".to_string()
}
```

### 4.2 置信度计算

```rust
fn jisuan_xinxidu(yonghushuru: &str, yituleixing: &str) -> f32 {
    let guanjianci = huoqu_guanjianci(yituleixing);
    let pipei_shuliang = guanjianci.iter()
        .filter(|ci| yonghushuru.contains(*ci))
        .count();
    
    (pipei_shuliang as f32) / (guanjianci.len() as f32)
}
```

## 五、实现步骤

### 步骤1：创建意图理解工具
- 文件：`gongju_yitulilei.rs`
- 实现：`dinyi()` 和 `zhixing()` 函数
- 关键词匹配逻辑

### 步骤2：改造工具注册模块
- 修改：`mod.rs`
- 新增：`huoqu_yitugongju()` 函数
- 新增：`huoqu_xiangguangongju()` 函数

### 步骤3：扩展消息管理
- 修改：`aixiaoxiguanli.rs`
- 新增工具列表管理方法

### 步骤4：改造对话接口
- 修改：`jiekou_aiduihua.rs`
- 实现两阶段调用流程

### 步骤5：测试验证
- 测试意图识别准确性
- 测试工具动态加载
- 测试完整对话流程

## 六、优势分析

### 6.1 性能优势
- 减少AI每次需要处理的工具数量
- 降低token消耗（工具定义占用token）
- 提高响应速度

### 6.2 准确性优势
- 意图明确后，工具选择更精准
- 避免工具误用
- 减少无效工具调用

### 6.3 扩展性优势
- 新增工具只需在意图分类中添加映射
- 不影响现有工具实现
- 支持复杂意图组合

## 七、注意事项

### 7.1 边界情况处理
- 意图识别失败 → 开放所有工具或提示用户
- 多意图混合 → 开放所有相关工具
- 意图变更 → 支持中途重新识别

### 7.2 用户体验
- 意图理解过程对用户透明
- 识别错误时允许用户纠正
- 提供意图识别结果反馈（可选）

### 7.3 性能考虑
- 意图识别应快速完成（避免复杂算法）
- 关键词匹配优先于AI分析
- 缓存常见意图模式

## 八、未来扩展

### 8.1 机器学习增强
- 收集意图识别数据
- 训练意图分类模型
- 提高识别准确率

### 8.2 上下文感知
- 记忆历史对话意图
- 支持意图延续和切换
- 多轮对话意图追踪

### 8.3 工具推荐
- 根据意图推荐相关工具
- 学习用户工具使用习惯
- 智能工具组合建议
