# aiduihua 代码质量分析与重构计划

## 文件概况
- `wasm_sdk/aiduihua_jiemian.js` — 界面层，464 行，类 `Aiduihuajiemian`
- `wasm_sdk/aiduihua_luoji.js` — 逻辑层，325 行，类 `Aiduihualuoji`
- 总计 789 行

---

## 一、Bug（必须修复）

### B1: daochulishi 引用不存在的属性
`aiduihua_luoji.js:314` 使用 `this.lishijilu`，该属性从未定义。
应改为 `this.shuju`（导出全部会话数据）。

### B2: xinjianhiuhua 拼写错误
「新建会话」的拼音应为 `xinjianhuihua`，代码写成了 `xinjianhiuhua`（hiuhua 不是合法拼音）。
涉及 7 处：`aiduihua_luoji.js` 3 处、`aiduihua_jiemian.js` 3 处、`ceshi.html` 1 处。

### B3: zhongzhi 两个分支完全相同
`aiduihua_jiemian.js:262-267` 的 if/else 两个分支都调用 `this.luoji.zhongzhiliushi()`，if-else 无意义。
应移除条件判断，直接调用。

---

## 二、代码质量问题

### Q1: console.log DEBUG 残留（12 处）
- `aiduihua_jiemian.js:260-261`（zhongzhi，2 条）
- `aiduihua_luoji.js:277-280`（baocunduquqi，4 条）
- `aiduihua_luoji.js:287-289,300,303,307`（zhongzhiliushi，6 条）

全部删除。

### Q2: 缩进不一致
`aiduihua_jiemian.js:213` — `this.zhengzaifasong = true;` 缺少 4 个空格缩进。

### Q3: 架构违规 — luoji 层操作 DOM
`aiduihua_luoji.js:313-323` 的 `daochulishi()` 直接创建 `<a>` 元素触发下载。
DOM 操作应在界面层。应将导出逻辑拆为：luoji 层返回 JSON 数据，jiemian 层执行下载。

### Q4: 函数过长
- `liushihuidiao`（jiemian:297-378）81 行，超过 50 行规范
- `fasong`（jiemian:199-256）57 行，超过 50 行规范

### Q5: 魔法字符串 'yizhongzhi'
`abortController = 'yizhongzhi'` 作为哨兵值出现在 `luoji.js` 多处，应提取为常量。

---

## 三、重复代码清单

### D1: 前置检查模式（重复 2 次）
```
feiliushiduihua (luoji:171-182)
liushiduihua    (luoji:237-248)
```
两者完全相同：kehu 检查 → yidenglu 检查 → neirong 检查。
**建议**：提取 `_duihuaqianjianzha(neirong)` 私有方法，返回 false 时 early return。

### D2: 消息列表构建（重复 2 次）
```
feiliushiduihua (luoji:188-189)
liushiduihua    (luoji:254-255)
```
完全相同：`this.huoqulishi().map(x => ({ juese, neirong }))` + `JSON.stringify`。
**建议**：提取 `_goujianxiaoxijson()` 方法。

### D3: 失败回滚用户消息（重复 3 次）
```
feiliushiduihua:209-210（普通失败）
feiliushiduihua:218-219（中断失败）
feiliushiduihua:224-228（异常失败）
```
三处都做同一件事：获取当前会话 → pop 最后消息 → 保存。
**建议**：已有 `shanchuzuihouyonghuxiaoxi()` 方法（luoji:147-153），直接复用。

### D4: 中断检查模式（重复 2 次）
```
feiliushiduihua:215-221
liushiduihua:264-267
```
两处都检查 `abortController === 'yizhongzhi'` 并记录日志。
**建议**：合并到 D1 提取的 `_duihuaqianjianzha` 或在 finally 统一处理。

### D5: 气泡创建模式（重复 4 次）
```
tianjiasikaoqipao  (jiemian:270-282)
tianjiashijianqipao(jiemian:284-295)
xianshijiazai      (jiemian:174-191)
liushihuidiao 内   (jiemian:358-370)
```
每次都：获取 quyu → 创建 div → 设置 class/style → innerHTML → appendChild → scrollTop。
**建议**：提取 `_tianjialinshiqipao(id, html, classming)` 通用方法。

### D6: 刷新 UI 组合调用（重复 5 次）
```
fasong:227-228, 249-250
qingkonglishi:417-418
xinjianhiuhua:434-435
qiehuanhuihua:441-442
shanchuhuihua:449-450
```
每次都是 `xuanranhuihualiebiao()` + `xuanranduihua()` 连续调用。
**建议**：提取 `shuaxinquanbu()` 方法。

### D7: 思考气泡 HTML 结构（重复 2 次）
```
xuanranduihua:132（历史渲染时内联模板）
tianjiasikaoqipao:276-279（流式实时创建）
```
`<details>` 折叠结构、颜色、样式完全相同。
**建议**：提取 `_sikaohtml(neirong, biaoti)` 生成 HTML 字符串。

### D8: 事件气泡 HTML 结构（重复 2 次）
```
xuanranduihua:114-119（历史渲染时内联模板）
tianjiashijianqipao:290-292（流式实时创建）
```
紫色小气泡结构完全相同。
**建议**：提取 `_shijianhtml(neirong)` 生成 HTML 字符串。

### D9: 流式事件类型处理（重复 4 块）
```
liushihuidiao:312-337
```
yitu / xunhuan / gongjudiaoyong / gongjujieguo 四个 if 块结构相同：
检查 `json.shijian === 'xxx'` → 格式化字符串 → push → 添加气泡。
**建议**：用查找表替代。

---

## 四、重构优先级

### P0 — 必须修复（Bug）
1. **B1** daochulishi 属性引用错误
2. **B2** xinjianhiuhua 拼写修正（7 处 + ceshi.html）
3. **B3** zhongzhi 无意义分支

### P1 — 高优先级（代码质量）
4. **Q1** 清除 12 处 console.log DEBUG
5. **Q2** 修复缩进
6. **Q3** daochulishi DOM 操作移到界面层
7. **Q5** 'yizhongzhi' 魔法字符串提取为常量

### P2 — 中优先级（消除重复）
8. **D1+D2** 提取 `_duihuaqianjianzha` + `_goujianxiaoxijson`（合并 feiliushi/liushi 公共逻辑）
9. **D3** 失败回滚统一使用 `shanchuzuihouyonghuxiaoxi()`
10. **D6** 提取 `shuaxinquanbu()` 替代 5 处组合调用
11. **D5** 提取 `_tianjialinshiqipao()` 通用气泡创建
12. **D7+D8** 提取 `_sikaohtml()` 和 `_shijianhtml()` 消除模板重复
13. **D9** 流式事件类型用查找表替代 4 个 if 块

### P3 — 低优先级（函数拆分）
14. **Q4** 拆分 `liushihuidiao`（完成 D9 后自然缩短）
15. **Q4** 拆分 `fasong`（完成 D6 后自然缩短）

---

## 五、重构策略

### 原则
- 遵循 `ai_coding_assistant_prompt_en.xml` 规范
- 所有标识符全小写拼音，字符串内容中文
- 函数不超过 50 行
- 不写内联注释，仅函数顶部注释
- luoji 层不操作 DOM，jiemian 层不操作数据

### 预期效果
- 修复 3 个 Bug（含 1 个功能性 Bug）
- 清除 12 处 DEBUG 日志
- 消除 ~9 组重复代码
- 总行数预计从 789 行降至 ~680 行（减少约 14%）
