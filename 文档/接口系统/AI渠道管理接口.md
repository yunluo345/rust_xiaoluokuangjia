# AI渠道管理接口

## 接口信息

- **路径**: `/xitong/aiqudao`
- **方法**: POST
- **加密**: 是（需要加密传输）
- **权限**: 需要登录 + 管理员用户组

## 请求格式

所有请求通过 `caozuo` 参数区分操作类型，其他参数根据操作类型不同而不同。

### 1. 查询所有渠道

```json
{
  "caozuo": "chaxun_quanbu"
}
```

**响应示例**:
```json
{
  "zhuangtaima": 200,
  "xiaoxi": "查询成功",
  "shijianchuo": 1234567890,
  "shuju": [
    {
      "id": "1",
      "mingcheng": "OpenAI",
      "leixing": "openai",
      "jiekoudizhi": "https://api.openai.com",
      "miyao": "sk-xxx",
      "moxing": "gpt-4",
      "wendu": "0.7",
      "zhuangtai": "1",
      "youxianji": 0,
      "beizhu": "主要渠道",
      "chuangjianshijian": "1234567890",
      "gengxinshijian": "1234567890"
    }
  ]
}
```

### 2. 查询启用的渠道

```json
{
  "caozuo": "chaxun_qiyong"
}
```

### 3. 根据ID查询

```json
{
  "caozuo": "chaxun_id",
  "id": "1"
}
```

**响应示例**:
```json
{
  "zhuangtaima": 200,
  "xiaoxi": "查询成功",
  "shijianchuo": 1234567890,
  "shuju": {
    "id": "1",
    "mingcheng": "OpenAI",
    "leixing": "openai",
    ...
  }
}
```

### 4. 新增渠道

```json
{
  "caozuo": "xinzeng",
  "mingcheng": "通义千问",
  "leixing": "qwen",
  "jiekoudizhi": "https://dashscope.aliyuncs.com",
  "miyao": "sk-xxx",
  "moxing": "qwen-max",
  "wendu": "0.7",
  "beizhu": "备用渠道"
}
```

**响应示例**:
```json
{
  "zhuangtaima": 200,
  "xiaoxi": "新增成功",
  "shijianchuo": 1234567890,
  "shuju": {
    "id": "2"
  }
}
```

### 5. 更新渠道

```json
{
  "caozuo": "gengxin",
  "id": "1",
  "ziduanlie": [
    ["mingcheng", "OpenAI官方"],
    ["wendu", "0.8"],
    ["beizhu", "更新后的备注"]
  ]
}
```

**响应示例**:
```json
{
  "zhuangtaima": 200,
  "xiaoxi": "更新成功",
  "shijianchuo": 1234567890,
  "shuju": {
    "yingxiang": 1
  }
}
```

### 6. 删除渠道

```json
{
  "caozuo": "shanchu",
  "id": "1"
}
```

**响应示例**:
```json
{
  "zhuangtaima": 200,
  "xiaoxi": "删除成功",
  "shijianchuo": 1234567890,
  "shuju": {
    "yingxiang": 1
  }
}
```

### 7. 切换启用/禁用状态

```json
{
  "caozuo": "qiehuanzhuangtai",
  "id": "1"
}
```

**响应示例**:
```json
{
  "zhuangtaima": 200,
  "xiaoxi": "状态切换成功",
  "shijianchuo": 1234567890,
  "shuju": {
    "yingxiang": 1
  }
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

**响应示例**:
```json
{
  "zhuangtaima": 200,
  "xiaoxi": "优先级更新成功",
  "shijianchuo": 1234567890,
  "shuju": {
    "yingxiang": 1
  }
}
```

## 错误响应

### 参数错误
```json
{
  "zhuangtaima": 400,
  "xiaoxi": "请求参数格式错误",
  "shijianchuo": 1234567890
}
```

### 权限不足
```json
{
  "zhuangtaima": 403,
  "xiaoxi": "权限不足，无法访问该接口",
  "shijianchuo": 1234567890
}
```

### 资源不存在
```json
{
  "zhuangtaima": 404,
  "xiaoxi": "渠道不存在",
  "shijianchuo": 1234567890
}
```

### 服务器错误
```json
{
  "zhuangtaima": 500,
  "xiaoxi": "操作失败",
  "shijianchuo": 1234567890
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| id | BIGINT | 渠道唯一标识（自增） |
| mingcheng | TEXT | 渠道名称，如"OpenAI"、"通义千问" |
| leixing | TEXT | 渠道类型，用于代码中区分调用逻辑 |
| jiekoudizhi | TEXT | API基础地址 |
| miyao | TEXT | API认证密钥 |
| moxing | TEXT | 该渠道默认使用的模型名 |
| wendu | TEXT | 生成随机性控制，0.0到2.0 |
| zhuangtai | TEXT | 启用或禁用，1启用0禁用 |
| youxianji | INTEGER | 多渠道调度顺序，数值越小优先级越高 |
| beizhu | TEXT | 补充说明（可选） |
| chuangjianshijian | TEXT | 记录创建时间 |
| gengxinshijian | TEXT | 记录最后更新时间 |

## 注意事项

1. 所有请求必须经过加密传输
2. 必须携带有效的管理员用户JWT令牌
3. 渠道名称不能重复
4. 删除操作不可恢复，请谨慎操作
5. 优先级数值越小，优先级越高
