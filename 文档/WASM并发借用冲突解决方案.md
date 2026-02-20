# WASM 并发借用冲突解决方案

## 问题现象

在前端同时发起多个请求时（例如 AI 对话 + 日报刷新），浏览器控制台报错：

```
Uncaught (in promise) Error: recursive use of an object detected 
which would lead to unsafe aliasing in rust
```

## 根本原因

`wasm_sdk` 使用 `Rc<RefCell<Zhuangtai>>` 管理内部状态。当多个异步请求并发执行时：

1. 请求 A 调用 `borrow()` 获取状态引用
2. 请求 A 在 `.await` 处挂起（等待网络响应）
3. 请求 B 尝试调用 `borrow()` 或 `borrow_mut()`
4. **冲突**：`RefCell` 检测到重复借用，触发运行时 panic

虽然 WASM 是单线程，但多个 async 任务会交错执行，导致借用冲突。

## 错误代码示例

```rust
// ❌ 错误：借用跨越 async 边界
pub fn huoqujiamixinxi(&self) -> Result<Jiamixinxi, JsValue> {
    let zhuangtai = self.zhuangtai.borrow();  // 借用持续到函数结束
    Ok(Jiamixinxi {
        miyao: zhuangtai.miyao.as_ref()?.clone(),
        huihuaid: zhuangtai.huihuaid.as_ref()?.clone(),
        kehugongyao: zhuangtai.kehugongyao_b64.as_ref()?.clone(),
    })
}
// 如果调用方在 await 期间持有返回值，其他请求无法借用
```

## 正确解决方案

**核心原则**：在 `.await` 之前释放所有 `RefCell` 借用

```rust
// ✅ 正确：显式限制借用作用域
pub fn huoqujiamixinxi(&self) -> Result<Jiamixinxi, JsValue> {
    let (miyao, huihuaid, kehugongyao) = {
        let zhuangtai = self.zhuangtai.borrow();  // 借用在块内
        (
            zhuangtai.miyao.as_ref()?.clone(),
            zhuangtai.huihuaid.as_ref()?.clone(),
            zhuangtai.kehugongyao_b64.as_ref()?.clone(),
        )
    };  // 借用在这里立即释放
    Ok(Jiamixinxi { miyao, huihuaid, kehugongyao })
}
```

**关键技巧**：
- 用 `{}` 块包裹 `borrow()` 调用
- 在块内提取需要的数据（clone）
- 块结束时自动释放借用
- 后续代码使用已拷贝的数据

## 标准请求模板

所有异步请求函数应遵循此模板：

```rust
pub async fn xxxqingqiu(&self, canshu: &str) -> Result<String, JsValue> {
    // 1. 快速读取状态副本
    let (lingpai, fuwuqidizhi) = {
        let zhuangtai = self.zhuangtai.borrow();
        (
            zhuangtai.lingpai.clone()?,
            zhuangtai.fuwuqidizhi.clone(),
        )
    };  // 立即释放借用
    
    // 2. 执行网络请求（可以安全 await）
    let url = format!("{}{}", fuwuqidizhi, lujing);
    let xiangying = putongqingqiu(&url, canshu).await?;
    
    // 3. 短暂写回状态（如需要）
    if xuyaogengxin {
        self.zhuangtai.borrow_mut().xxx = xinzhi;
    }
    
    Ok(xiangying)
}
```

## 后续扩展 API 的 5 条铁律

### 1. 禁止借用跨 `.await`
```rust
// ❌ 错误
let zhuangtai = self.zhuangtai.borrow();
let jieguo = wangluoqingqiu().await;  // 借用仍在持有
shiyong(&zhuangtai);

// ✅ 正确
let shuju = { self.zhuangtai.borrow().shuju.clone() };
let jieguo = wangluoqingqiu().await;
```

### 2. 只返回 Owned 数据
```rust
// ❌ 错误：返回引用
pub fn huoqulingpai(&self) -> Option<&str> {
    self.zhuangtai.borrow().lingpai.as_deref()
}

// ✅ 正确：返回拷贝
pub fn huoqulingpai(&self) -> Option<String> {
    self.zhuangtai.borrow().lingpai.clone()
}
```

### 3. 回调中避免重入
```rust
// ❌ 错误：SSE 回调里直接读写状态
pub async fn ssejiekou(&self, huidiao: &Function) {
    let xinxi = self.huoqujiamixinxi()?;  // 可能在回调重入时冲突
    duquliushi(huidiao).await;
}

// ✅ 正确：提前提取数据
pub async fn ssejiekou(&self, huidiao: &Function) {
    let xinxi = self.huoqujiamixinxi()?;  // 提前提取
    duquliushi_with_data(huidiao, xinxi).await;  // 传递副本
}
```

### 4. 密钥协商做单飞控制
```rust
// 避免并发重复协商
pub async fn quebaoxieshang(&self) -> Result<(), JsValue> {
    if self.yixieshang() {
        return Ok(());
    }
    // TODO: 加锁或标志位防止并发协商
    self.xieshangmiyao().await
}
```

### 5. 集中状态访问
```rust
// ✅ 好：统一的状态访问方法
impl Kehuduanjiami {
    fn duquzhuangtai<T>(&self, f: impl FnOnce(&Zhuangtai) -> T) -> T {
        f(&self.zhuangtai.borrow())
    }
    
    fn xiuzhuangtai(&self, f: impl FnOnce(&mut Zhuangtai)) {
        f(&mut self.zhuangtai.borrow_mut())
    }
}

// 使用时
let lingpai = self.duquzhuangtai(|z| z.lingpai.clone());
```

## 验证方法

1. 编译 WASM：`wasm-pack build --target web`
2. 刷新浏览器页面
3. 发起 AI 对话（不等待完成）
4. 立即刷新日报列表
5. 检查控制台无 "recursive use" 错误

## 相关文件

- `wasm_sdk/src/kehuduanjiami_neibu.rs` - 状态管理核心
- `wasm_sdk/src/lib.rs` - 公共 API 层

## 总结

- **问题**：`Rc<RefCell<>>` 在异步环境中的借用冲突
- **原因**：借用跨越 `.await` 点，多个任务交错执行
- **方案**：显式限制借用作用域，提前提取数据副本
- **预防**：遵循 5 条铁律，所有新 API 按标准模板编写
