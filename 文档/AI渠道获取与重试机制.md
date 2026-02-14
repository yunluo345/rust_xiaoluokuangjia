# AI 渠道获取与重试机制

## 概述

本文档说明 AI 渠道获取的完整流程，包括错误处理和自动重试机制。

## 功能特性

### 1. 智能渠道选择

- **状态过滤**：仅选择启用状态的渠道
- **优先级排序**：按优先级从高到低排序
- **负载均衡**：同优先级渠道随机选择，实现负载均衡

### 2. 错误处理

当渠道获取失败时，系统会返回明确的错误信息：

| 错误类型 | HTTP 状态码 | 错误消息 | 说明 |
|---------|-----------|---------|------|
| 没有可用渠道 | 503 | 没有可用的AI渠道，请稍后重试 | 数据库中无符合条件的渠道 |

### 3. 自动重试机制

通过配置文件 `peizhi/ai.json` 控制重试行为：

```json
{
  "qudaohuoqu": {
    "qiyongchongshi": true,
    "chongshicishu": 3,
    "chongshijiange": 1000
  }
}
```

**配置说明：**

- `qiyongchongshi`：是否启用重试（默认：`true`）
- `chongshicishu`：最大重试次数（默认：`3`）
- `chongshijiange`：重试间隔毫秒数（默认：`1000`）

## 实现架构

### 分层设计

```
接口层 (jiekou_aiduihua.rs)
    ↓ 调用
数据层 (shujucaozuo_aiqudao.rs)
    ↓ 查询
数据库 (PostgreSQL)
```

### 核心函数

#### 1. `lunxun(leixing: &str) -> Option<Value>`

基础渠道获取函数，单次尝试。

**参数：**
- `leixing`：渠道类型（如 "openai"、"xiangliang"）

**返回值：**
- `Some(Value)`：成功获取渠道数据
- `None`：未找到可用渠道

**使用场景：**
- 不需要重试的场景
- 自定义重试逻辑

#### 2. `lunxun_daichongshi(leixing: &str) -> Result<Value, QudaoCuowu>`

带重试机制的渠道获取函数，推荐使用。

**参数：**
- `leixing`：渠道类型

**返回值：**
- `Ok(Value)`：成功获取渠道数据
- `Err(QudaoCuowu)`：所有重试均失败

**重试流程：**

```
第1次尝试
    ↓ 失败
等待 chongshijiange 毫秒
    ↓
第2次尝试
    ↓ 失败
等待 chongshijiange 毫秒
    ↓
第3次尝试
    ↓ 失败
返回错误
```

**日志输出：**

```
[渠道获取] 第1次失败，1000毫秒后重试，类型: openai
[渠道获取] 第2次重试成功，类型: openai
```

或

```
[渠道获取] 第1次失败，1000毫秒后重试，类型: openai
[渠道获取] 第2次失败，1000毫秒后重试，类型: openai
[渠道获取] 第3次失败，1000毫秒后重试，类型: openai
[渠道获取] 重试3次后仍失败，类型: openai
```

### 错误类型定义

```rust
pub enum QudaoCuowu {
    MeiyouKeyongQudao,  // 没有可用的渠道
}

impl QudaoCuowu {
    pub fn xiaoxi(&self) -> &'static str;      // 获取错误消息
    pub fn zhuangtaima(&self) -> u16;          // 获取HTTP状态码
}
```

## 使用示例

### 接口层调用

```rust
async fn zhixing_duihua(qingqiu: Qingqiuti, miyao: Vec<u8>) -> HttpResponse {
    // 使用带重试的渠道获取
    let qudaoshuju = match qudaocaozuo::lunxun_daichongshi(&qingqiu.leixing).await {
        Ok(s) => s,
        Err(e) => return jiamicuowu(e.zhuangtaima(), e.xiaoxi(), &miyao),
    };
    
    // 解析渠道配置
    let peizhi = match Qudaopeizhi::cong_shuju(&qudaoshuju) {
        Some(p) => p,
        None => return jiamicuowu(500, "渠道配置解析失败", &miyao),
    };
    
    // 继续处理...
}
```

### 数据层调用

```rust
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao as qudaocaozuo;

// 方式1：带重试（推荐）
match qudaocaozuo::lunxun_daichongshi("openai").await {
    Ok(qudao) => {
        println!("获取成功: {:?}", qudao);
    }
    Err(e) => {
        eprintln!("获取失败: {}", e.xiaoxi());
    }
}

// 方式2：单次尝试
if let Some(qudao) = qudaocaozuo::lunxun("openai").await {
    println!("获取成功: {:?}", qudao);
} else {
    eprintln!("获取失败");
}
```

## 配置调优

### 场景1：高可用性要求

```json
{
  "qudaohuoqu": {
    "qiyongchongshi": true,
    "chongshicishu": 5,
    "chongshijiange": 500
  }
}
```

- 增加重试次数到 5 次
- 缩短重试间隔到 500ms
- 适用于对可用性要求极高的场景

### 场景2：快速失败

```json
{
  "qudaohuoqu": {
    "qiyongchongshi": false,
    "chongshicishu": 0,
    "chongshijiange": 0
  }
}
```

- 禁用重试机制
- 失败立即返回
- 适用于对响应时间要求严格的场景

### 场景3：平衡模式（默认）

```json
{
  "qudaohuoqu": {
    "qiyongchongshi": true,
    "chongshicishu": 3,
    "chongshijiange": 1000
  }
}
```

- 适度重试 3 次
- 间隔 1 秒
- 平衡可用性和响应时间

## 监控与日志

### 日志级别

- **INFO**：重试成功
- **WARN**：单次失败（会继续重试）
- **ERROR**：所有重试均失败

### 监控指标建议

1. **渠道获取成功率**：成功次数 / 总请求次数
2. **平均重试次数**：总重试次数 / 总请求次数
3. **渠道可用性**：各渠道的启用/禁用状态
4. **响应时间**：包含重试时间的总响应时间

## 故障排查

### 问题1：频繁重试

**现象：** 日志中大量重试记录

**可能原因：**
1. 数据库中没有启用的渠道
2. 渠道优先级配置错误
3. 数据库连接不稳定

**解决方案：**
```sql
-- 检查启用的渠道
SELECT * FROM aiqudao WHERE zhuangtai = '1';

-- 检查渠道优先级
SELECT mingcheng, leixing, youxianji, zhuangtai FROM aiqudao ORDER BY youxianji DESC;
```

### 问题2：503 错误

**现象：** 客户端收到 503 状态码

**原因：** 所有重试均失败，没有可用渠道

**解决方案：**
1. 确保数据库中至少有一个启用的渠道
2. 检查渠道配置是否正确
3. 增加重试次数或间隔

### 问题3：响应时间过长

**现象：** 接口响应缓慢

**原因：** 重试次数过多或间隔过长

**解决方案：**
1. 减少 `chongshicishu`
2. 缩短 `chongshijiange`
3. 或禁用重试机制

## 最佳实践

1. **生产环境**：启用重试，设置合理的次数（3-5次）和间隔（500-1000ms）
2. **开发环境**：可禁用重试，快速发现问题
3. **监控告警**：当重试率超过阈值时触发告警
4. **渠道管理**：定期检查渠道状态，及时启用/禁用
5. **日志分析**：定期分析重试日志，优化渠道配置

## 相关文档

- [配置系统 API 文档](./配置系统/API文档.md)
- [配置系统使用指南](./配置系统/使用指南.md)
