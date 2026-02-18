# AI渠道管理工具使用说明

## 概述

新增了 `aiqudao_guanli` 工具，让AI在对话中能够管理AI渠道的增删改查操作。该工具需要用户令牌进行权限验证。

## 工具功能

### 1. 查询全部渠道
```json
{
  "caozuo": "chaxun_quanbu"
}
```

### 2. 查询启用的渠道
```json
{
  "caozuo": "chaxun_qiyong"
}
```

### 3. 按ID查询渠道
```json
{
  "caozuo": "chaxun_id",
  "id": "1"
}
```

### 4. 新增渠道
```json
{
  "caozuo": "xinzeng",
  "mingcheng": "OpenAI官方",
  "leixing": "openapi",
  "jiekoudizhi": "https://api.openai.com/v1",
  "miyao": "sk-xxx",
  "moxing": "gpt-4",
  "wendu": "0.7",
  "beizhu": "官方接口"
}
```

### 5. 更新渠道
```json
{
  "caozuo": "gengxin",
  "id": "1",
  "ziduanlie": [
    ["mingcheng", "新名称"],
    ["wendu", "0.8"]
  ]
}
```

### 6. 删除渠道
```json
{
  "caozuo": "shanchu",
  "id": "1"
}
```

### 7. 切换渠道状态
```json
{
  "caozuo": "qiehuanzhuangtai",
  "id": "1"
}
```

### 8. 更新优先级
```json
{
  "caozuo": "gengxinyouxianji",
  "id": "1",
  "youxianji": "10"
}
```

## 权限要求

- 工具执行需要有效的JWT令牌
- 令牌通过HTTP请求头的Authorization字段传递
- 令牌验证失败会返回权限错误

## 返回格式

成功时：
```json
{
  "chenggong": true,
  "shuju": { ... }
}
```

失败时：
```json
{
  "cuowu": "错误信息"
}
```

## 技术实现

1. **工具注册**：在 `gongjuji/mod.rs` 中注册新工具
2. **令牌传递**：扩展了工具执行签名，支持令牌参数
3. **权限验证**：使用 `jwtgongju::yanzheng` 验证令牌有效性
4. **数据操作**：复用现有的 `shujucaozuo_aiqudao` 模块
5. **ReAct循环**：修改了 `react_xunhuan` 和相关函数以传递令牌

## 使用示例

用户可以在AI对话中说："帮我查看所有AI渠道"，AI会自动调用 `aiqudao_guanli` 工具执行查询操作。