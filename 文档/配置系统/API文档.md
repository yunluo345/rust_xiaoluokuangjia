# 配置系统 API 文档

## 模块：peizhixitongzhuti

配置系统主体模块，提供配置的初始化、读取、缓存管理功能。

### 公共方法

#### `chushihua() -> bool`

初始化配置系统：同步所有配置文件并加载到内存。

**返回值：**
- `true` — 初始化成功
- `false` — 初始化失败

**功能：**
1. 检查并创建 `peizhi/` 目录
2. 同步所有注册的配置文件（补充缺失字段）
3. 将所有配置文件加载到内存缓存

**使用示例：**
```rust
if !peizhixitongzhuti::chushihua() {
    eprintln!("配置系统初始化失败");
    std::process::exit(1);
}
```

**注意事项：**
- 必须在程序启动时调用
- 只需调用一次
- 失败时应终止程序

---

#### `duqupeizhi<T>(wenjianming: &str) -> Option<T>`

读取配置文件内容（优先从内存缓存读取，回退到磁盘 IO）。

**泛型参数：**
- `T` — 配置结构体类型，必须实现 `serde::de::DeserializeOwned`

**参数：**
- `wenjianming` — 配置文件名（不含 `.json` 后缀）

**返回值：**
- `Some(T)` — 读取成功，返回配置对象
- `None` — 读取失败（文件不存在或反序列化失败）

**读取优先级：**
1. 内存缓存（零 IO）
2. 磁盘文件（回退方案）

**使用示例：**
```rust
use peizhixt::peizhi_nr::peizhi_zongpeizhi::Zongpeizhi;

if let Some(peizhi) = peizhixitongzhuti::duqupeizhi::<Zongpeizhi>(
    Zongpeizhi::wenjianming()
) {
    println!("网站名称: {}", peizhi.wangzhanmingcheng);
    println!("后端端口: {}", peizhi.houduanyunxingduankou);
} else {
    eprintln!("读取配置失败");
}
```

**性能：**
- 初始化后从内存读取，时间复杂度 O(1)
- 反序列化开销取决于配置大小

---

#### `jiazaidaohuancun(wenjianming: &str) -> bool`

将指定配置文件加载到内存缓存。

**参数：**
- `wenjianming` — 配置文件名（不含 `.json` 后缀）

**返回值：**
- `true` — 加载成功
- `false` — 加载失败（文件不存在或读取失败）

**使用场景：**
- 手动加载新增的配置文件
- 重新加载已修改的配置文件

**使用示例：**
```rust
if peizhixitongzhuti::jiazaidaohuancun("zongpeizhi") {
    println!("配置已加载到内存");
}
```

---

#### `jiazaisuoyoupeizhi() -> bool`

将 `peizhi/` 文件夹内所有配置文件加载到内存。

**返回值：**
- `true` — 所有文件加载成功
- `false` — 至少有一个文件加载失败

**功能：**
- 自动扫描 `peizhi/` 目录
- 批量加载所有 `.json` 文件到内存

**使用场景：**
- 初始化时自动调用
- 手动刷新所有配置缓存

**使用示例：**
```rust
if peizhixitongzhuti::jiazaisuoyoupeizhi() {
    println!("所有配置已加载到内存");
}
```

---

## 模块：neicungongju

全局内存缓存工具，提供键值对存储功能。

### 公共方法

#### `jiazaiwenjian(lujing: &str) -> bool`

将指定文件加载到内存缓存，键为文件路径。

**参数：**
- `lujing` — 文件完整路径（如 `"peizhi/zongpeizhi.json"`）

**返回值：**
- `true` — 加载成功
- `false` — 加载失败

**使用示例：**
```rust
use gongju::neicungongju;

if neicungongju::jiazaiwenjian("peizhi/zongpeizhi.json") {
    println!("文件已加载到内存");
}
```

---

#### `piliangjiaizai(lujinglie: &[&str]) -> bool`

批量加载多个文件到内存缓存。

**参数：**
- `lujinglie` — 文件路径数组

**返回值：**
- `true` — 所有文件加载成功
- `false` — 至少有一个文件加载失败

**使用示例：**
```rust
let wenjianlie = ["peizhi/zongpeizhi.json", "peizhi/shujuku.json"];
if neicungongju::piliangjiaizai(&wenjianlie) {
    println!("批量加载成功");
}
```

---

#### `duqu(jian: &str) -> Option<String>`

从内存缓存读取内容。

**参数：**
- `jian` — 缓存键（通常是文件路径）

**返回值：**
- `Some(String)` — 读取成功，返回内容
- `None` — 键不存在

**使用示例：**
```rust
if let Some(neirong) = neicungongju::duqu("peizhi/zongpeizhi.json") {
    println!("配置内容: {}", neirong);
}
```

---

#### `xieru(jian: &str, zhi: &str) -> bool`

写入或更新缓存内容。

**参数：**
- `jian` — 缓存键
- `zhi` — 缓存值

**返回值：**
- `true` — 写入成功
- `false` — 写入失败

**使用示例：**
```rust
neicungongju::xieru("peizhi/zongpeizhi.json", "{...}");
```

---

#### `yichu(jian: &str) -> bool`

移除缓存中的指定键。

**参数：**
- `jian` — 缓存键

**返回值：**
- `true` — 移除成功（键存在）
- `false` — 移除失败（键不存在）

**使用示例：**
```rust
if neicungongju::yichu("peizhi/zongpeizhi.json") {
    println!("缓存已移除");
}
```

---

#### `qingkong() -> bool`

清空全部缓存。

**返回值：**
- `true` — 清空成功
- `false` — 清空失败

**使用示例：**
```rust
neicungongju::qingkong();
```

---

#### `cunzai(jian: &str) -> bool`

检查缓存中是否存在指定键。

**参数：**
- `jian` — 缓存键

**返回值：**
- `true` — 键存在
- `false` — 键不存在

**使用示例：**
```rust
if neicungongju::cunzai("peizhi/zongpeizhi.json") {
    println!("配置已缓存");
}
```

---

#### `regengxin() -> bool`

热更新：重新从磁盘加载所有已缓存的文件。

**返回值：**
- `true` — 所有文件重新加载成功
- `false` — 至少有一个文件重新加载失败

**使用场景：**
- 手动修改配置文件后刷新内存缓存
- 不重启程序更新配置

**使用示例：**
```rust
if neicungongju::regengxin() {
    println!("所有缓存已热更新");
}
```

---

## 模块：wenjiancaozuo

文件操作工具，封装所有文件 IO 操作。

### 公共方法

#### `wenjiancunzai(lujing: &str) -> bool`

检查文件是否存在。

**参数：**
- `lujing` — 文件路径

**返回值：**
- `true` — 文件存在
- `false` — 文件不存在或路径指向目录

**使用示例：**
```rust
use gongju::wenjiancaozuo;

if wenjiancaozuo::wenjiancunzai("peizhi/zongpeizhi.json") {
    println!("配置文件存在");
}
```

---

#### `duquwenjian(lujing: &str) -> Option<String>`

以文本形式读取任意文件。

**参数：**
- `lujing` — 文件路径

**返回值：**
- `Some(String)` — 读取成功，返回文件内容
- `None` — 读取失败

**使用示例：**
```rust
if let Some(neirong) = wenjiancaozuo::duquwenjian("peizhi/zongpeizhi.json") {
    println!("文件内容: {}", neirong);
}
```

---

#### `xieruwenjian(lujing: &str, neirong: &str) -> bool`

将文本内容写入任意文件，沿途文件夹不存在则自动创建。

**参数：**
- `lujing` — 文件路径
- `neirong` — 文件内容

**返回值：**
- `true` — 写入成功
- `false` — 写入失败

**使用示例：**
```rust
let neirong = r#"{"wangzhanmingcheng": "测试"}"#;
if wenjiancaozuo::xieruwenjian("peizhi/test.json", neirong) {
    println!("文件写入成功");
}
```

---

#### `zhuijianeirong(lujing: &str, neirong: &str) -> bool`

追加内容到文件末尾，文件不存在则自动创建。

**参数：**
- `lujing` — 文件路径
- `neirong` — 追加内容

**返回值：**
- `true` — 追加成功
- `false` — 追加失败

**使用示例：**
```rust
wenjiancaozuo::zhuijianeirong("log.txt", "新日志\n");
```

---

#### `shanchuwenjian(lujing: &str) -> bool`

删除文件。

**参数：**
- `lujing` — 文件路径

**返回值：**
- `true` — 删除成功
- `false` — 删除失败

**使用示例：**
```rust
wenjiancaozuo::shanchuwenjian("peizhi/old.json");
```

---

#### `fuzhiwenjian(yuan: &str, mubiao: &str) -> bool`

复制文件到目标路径，目标父目录不存在则自动创建。

**参数：**
- `yuan` — 源文件路径
- `mubiao` — 目标文件路径

**返回值：**
- `true` — 复制成功
- `false` — 复制失败

**使用示例：**
```rust
wenjiancaozuo::fuzhiwenjian("peizhi/zongpeizhi.json", "backup/zongpeizhi.json");
```

---

#### `yidongwenjian(yuan: &str, mubiao: &str) -> bool`

移动文件到目标路径，优先原子重命名，跨设备时回退为复制后删除。

**参数：**
- `yuan` — 源文件路径
- `mubiao` — 目标文件路径

**返回值：**
- `true` — 移动成功
- `false` — 移动失败

**使用示例：**
```rust
wenjiancaozuo::yidongwenjian("peizhi/old.json", "peizhi/new.json");
```

---

#### `liebiaowenjian(mulu: &str) -> Option<Vec<PathBuf>>`

列出目录下所有文件路径（不包含子目录）。

**参数：**
- `mulu` — 目录路径

**返回值：**
- `Some(Vec<PathBuf>)` — 文件路径列表
- `None` — 读取失败

**使用示例：**
```rust
if let Some(wenjianlie) = wenjiancaozuo::liebiaowenjian("peizhi") {
    for wenjian in wenjianlie {
        println!("文件: {:?}", wenjian);
    }
}
```

---

## 配置结构体规范

所有配置结构体必须遵守以下规范：

### 必需的 Trait

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YourConfig {
    // 字段定义
}
```

### 必需的 Default 实现

```rust
impl Default for YourConfig {
    fn default() -> Self {
        Self {
            // 提供合理的默认值
        }
    }
}
```

### 必需的 wenjianming 方法

```rust
impl YourConfig {
    pub fn wenjianming() -> &'static str {
        "yourconfig"  // 不含 .json 后缀
    }
}
```

### 完整示例

```rust
#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shujuku {
    pub zhiji: String,
    pub duankou: u16,
    pub yonghuming: String,
    pub mima: String,
}

impl Default for Shujuku {
    fn default() -> Self {
        Self {
            zhiji: "localhost".to_string(),
            duankou: 5432,
            yonghuming: "postgres".to_string(),
            mima: "".to_string(),
        }
    }
}

impl Shujuku {
    pub fn wenjianming() -> &'static str {
        "shujuku"
    }
}
```

---

## 错误处理

配置系统采用 `Option` 和 `bool` 返回值，不抛出异常：

- `Option<T>` — 可能失败的操作，`None` 表示失败
- `bool` — 成功/失败标志，`false` 表示失败

**推荐的错误处理模式：**

```rust
// 模式 1：使用 if let
if let Some(peizhi) = peizhixitongzhuti::duqupeizhi::<Zongpeizhi>(
    Zongpeizhi::wenjianming()
) {
    // 使用配置
} else {
    eprintln!("读取配置失败");
}

// 模式 2：使用 expect（确定配置必须存在）
let peizhi = peizhixitongzhuti::duqupeizhi::<Zongpeizhi>(
    Zongpeizhi::wenjianming()
).expect("读取配置失败");

// 模式 3：使用 unwrap_or_default（提供回退值）
let peizhi = peizhixitongzhuti::duqupeizhi::<Zongpeizhi>(
    Zongpeizhi::wenjianming()
).unwrap_or_default();
```

---

## 线程安全

所有公共 API 都是线程安全的：

- `neicungongju` 使用 `RwLock` 保护全局缓存
- 多线程并发读取配置不会阻塞
- 写操作（初始化、热更新）会加写锁

**并发读取示例：**

```rust
use std::thread;

let handles: Vec<_> = (0..10).map(|i| {
    thread::spawn(move || {
        if let Some(peizhi) = peizhixitongzhuti::duqupeizhi::<Zongpeizhi>(
            Zongpeizhi::wenjianming()
        ) {
            println!("线程 {} 读取成功", i);
        }
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}
```

---

## 性能建议

1. **初始化时机**：在程序启动时调用 `chushihua()`，只调用一次
2. **读取频率**：配置读取非常快（内存查找 + 反序列化），可以频繁调用
3. **热更新**：`regengxin()` 会重新读取所有文件，不要频繁调用
4. **缓存大小**：配置文件应保持小巧（< 10KB），避免占用过多内存

---

## 相关文档

- [使用指南](./使用指南.md)
- [实现原理](./实现原理.md)
