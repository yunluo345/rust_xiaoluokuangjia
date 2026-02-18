// AI对话管理 - 界面层
export class Aiduihuajiemian {
    constructor(luoji, rongqiid) {
        this.luoji = luoji;
        this.rongqi = document.getElementById(rongqiid);
        this.liushihuifu = ''; // 流式回复缓存
        this.zhengzaifasong = false;
    }

    xuanran() {
        this.rongqi.innerHTML = '';
        
        // 标题栏
        const tou = document.createElement('div');
        tou.style.cssText = 'display:flex;justify-content:space-between;align-items:center;margin-bottom:12px';
        tou.innerHTML = `
            <h2 style="font-size:15px;color:#475569;margin:0">AI对话</h2>
            <div>
                <button class="aq-btn aq-btn-xiao" onclick="aiduihua_xinjianhiuhua()" style="margin:0 4px 0 0">新建对话</button>
                <button class="aq-btn aq-btn-xiao" onclick="aiduihua_qingkonglishi()" style="margin:0 4px 0 0">清空历史</button>
                <button class="aq-btn aq-btn-xiao aq-btn-huang" onclick="aiduihua_daochulishi()" style="margin:0">导出历史</button>
            </div>
        `;
        this.rongqi.appendChild(tou);

        // 会话列表栏
        const huihualan = document.createElement('div');
        huihualan.id = 'aiduihua_huihualan';
        huihualan.style.cssText = 'margin-bottom:12px;display:flex;gap:6px;overflow-x:auto;padding-bottom:4px';
        this.rongqi.appendChild(huihualan);
        this.xuanranhuihualiebiao();

        // 模式选择
        const moshilan = document.createElement('div');
        moshilan.style.cssText = 'margin-bottom:12px;display:flex;gap:8px;align-items:center';
        moshilan.innerHTML = `
            <span style="font-size:13px;color:#475569">模式:</span>
            <label style="display:flex;align-items:center;gap:4px;cursor:pointer">
                <input type="radio" name="duihua_moshi" value="feiliushi" checked onchange="aiduihua_qiehuanmoshi('feiliushi')">
                <span style="font-size:13px">非流式</span>
            </label>
            <label style="display:flex;align-items:center;gap:4px;cursor:pointer">
                <input type="radio" name="duihua_moshi" value="liushi" onchange="aiduihua_qiehuanmoshi('liushi')">
                <span style="font-size:13px">流式</span>
            </label>
        `;
        this.rongqi.appendChild(moshilan);

        // 对话区域
        const duihuaqu = document.createElement('div');
        duihuaqu.id = 'aiduihua_quyu';
        duihuaqu.style.cssText = 'background:#F8FAFC;border:1px solid #E2E8F0;border-radius:8px;padding:12px;max-height:400px;overflow-y:auto;margin-bottom:12px;min-height:200px;display:flex;flex-direction:column;justify-content:center';
        this.rongqi.appendChild(duihuaqu);

        // 输入区域
        const shuruqu = document.createElement('div');
        shuruqu.style.cssText = 'display:flex;gap:8px;align-items:stretch';
        shuruqu.innerHTML = `
            <textarea id="aiduihua_shuru" placeholder="输入消息..." style="flex:1;border:1px solid #E2E8F0;border-radius:8px;padding:10px;font-size:14px;resize:vertical;min-height:60px;outline:none;font-family:inherit"></textarea>
            <button id="aiduihua_fasong_btn" class="aq-btn aq-btn-lv" onclick="aiduihua_fasong()" style="margin:0">发送</button>
            <button id="aiduihua_zhongzhi_btn" class="aq-btn aq-btn-hong" onclick="aiduihua_zhongzhi()" style="margin:0;display:none">终止</button>
        `;
        this.rongqi.appendChild(shuruqu);

        // 渲染历史记录
        this.xuanranduihua();
    }

    xuanranhuihualiebiao() {
        const lan = document.getElementById('aiduihua_huihualan');
        if (!lan) return;

        const liebiao = this.luoji.huoquhuihualiebiao();
        const dangqianid = this.luoji.huoqudangqianid();

        let html = '';
        liebiao.forEach(h => {
            const xuanzhong = h.id === dangqianid;
            const bg = xuanzhong ? '#3B82F6' : '#E2E8F0';
            const color = xuanzhong ? '#fff' : '#475569';
            html += `
                <div style="display:flex;align-items:center;gap:4px;flex-shrink:0">
                    <button onclick="aiduihua_qiehuanhuihua('${h.id}')" style="background:${bg};color:${color};border:none;border-radius:6px;padding:6px 12px;font-size:13px;cursor:pointer;white-space:nowrap;max-width:140px;overflow:hidden;text-overflow:ellipsis;margin:0;min-height:32px" title="${this.zhuanyihtml(h.mingcheng)}">${this.zhuanyihtml(h.mingcheng)}</button>
                    <button onclick="aiduihua_chongmingming('${h.id}')" style="background:#F1F5F9;border:1px solid #CBD5E1;border-radius:4px;cursor:pointer;font-size:13px;padding:4px 6px;margin:0;color:#64748B;min-height:28px" title="重命名">✏</button>
                    <button onclick="aiduihua_shanchuhuihua('${h.id}')" style="background:#FEF2F2;border:1px solid #FECACA;border-radius:4px;cursor:pointer;font-size:13px;padding:4px 6px;margin:0;color:#EF4444;min-height:28px" title="删除">✕</button>
                </div>
            `;
        });
        lan.innerHTML = html;
    }

    xuanranduihua() {
        const quyu = document.getElementById('aiduihua_quyu');
        if (!quyu) return;

        const lishi = this.luoji.huoqulishi();
        if (lishi.length === 0) {
            quyu.innerHTML = '<p style="color:#94A3B8;text-align:center;margin:20px 0">暂无对话记录</p>';
            return;
        }

        let html = '';
        lishi.forEach((xiaoxi, idx) => {
            const shiuser = xiaoxi.juese === 'user';
            const yanse = shiuser ? '#3B82F6' : '#10B981';
            const beijing = shiuser ? '#EFF6FF' : '#F0FDF4';
            const duiqi = shiuser ? 'flex-end' : 'flex-start';
            const juese_text = shiuser ? '我' : 'AI';
            
            html += `
                <div style="display:flex;justify-content:${duiqi};margin-bottom:12px">
                    <div style="max-width:80%;background:${beijing};border-radius:8px;padding:10px;position:relative">
                        <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:4px">
                            <span style="font-size:12px;font-weight:600;color:${yanse}">${juese_text}</span>
                            <button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="aiduihua_shanchuxiaoxi(${idx})" style="padding:2px 6px;font-size:11px;min-height:20px">删除</button>
                        </div>
                        <div style="font-size:13px;color:#1E293B;white-space:pre-wrap;word-break:break-word">${this.zhuanyihtml(xiaoxi.neirong)}</div>
                    </div>
                </div>
            `;
        });

        quyu.innerHTML = html;
        // 滚动到底部
        quyu.scrollTop = quyu.scrollHeight;
    }

    zhuanyihtml(wenben) {
        return wenben
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
            .replace(/"/g, '&quot;')
            .replace(/'/g, '&#039;');
    }

    qiehuanmoshi(moshi) {
        this.luoji.shezhimoshi(moshi);
    }

    // 显示/隐藏请求中状态
    shezhibtnzhuangtai(zhengzai) {
        const btn = document.getElementById('aiduihua_fasong_btn');
        const zhongzhiBtn = document.getElementById('aiduihua_zhongzhi_btn');
        if (zhengzai) {
            btn.disabled = true;
            btn.textContent = '请求中...';
            btn.style.display = 'none';
            if (zhongzhiBtn) zhongzhiBtn.style.display = '';
        } else {
            btn.disabled = false;
            btn.textContent = '发送';
            btn.style.display = '';
            if (zhongzhiBtn) zhongzhiBtn.style.display = 'none';
        }
    }

    // 在对话区显示"正在请求..."加载提示
    xianshijiazai() {
        const quyu = document.getElementById('aiduihua_quyu');
        if (!quyu) return;
        let jiazaiqu = document.getElementById('aiduihua_jiazai_linshi');
        if (!jiazaiqu) {
            jiazaiqu = document.createElement('div');
            jiazaiqu.id = 'aiduihua_jiazai_linshi';
            jiazaiqu.style.cssText = 'display:flex;justify-content:flex-start;margin-bottom:12px';
            jiazaiqu.innerHTML = `
                <div style="max-width:80%;background:#F0FDF4;border-radius:8px;padding:10px">
                    <div style="font-size:12px;font-weight:600;color:#10B981;margin-bottom:4px">AI</div>
                    <div style="font-size:13px;color:#94A3B8">正在请求...</div>
                </div>
            `;
            quyu.appendChild(jiazaiqu);
            quyu.scrollTop = quyu.scrollHeight;
        }
    }

    // 移除加载提示
    yichujiazai() {
        const jiazaiqu = document.getElementById('aiduihua_jiazai_linshi');
        if (jiazaiqu) jiazaiqu.remove();
    }

    async fasong() {
        if (this.zhengzaifasong) {
            this.luoji.rizhi('正在发送中，请稍候', 'warn');
            return;
        }

        const shuru = document.getElementById('aiduihua_shuru');
        const neirong = shuru.value.trim();

        if (!neirong) {
            this.luoji.rizhi('消息内容不能为空', 'warn');
            return;
        }

        this.zhengzaifasong = true;
        this.shezhibtnzhuangtai(true);
        this.xianshijiazai();

        try {
            if (this.luoji.dangqianmoshi === 'feiliushi') {
                // 非流式
                const huifu = await this.luoji.feiliushiduihua(neirong);
                if (huifu) {
                    shuru.value = '';
                    this.xuanranhuihualiebiao();
                    this.xuanranduihua();
                }
            } else {
                // 流式
                this.liushihuifu = '';
                await this.luoji.liushiduihua(neirong, 'aiduihua_liushi_huidiao', 'aiduihua_duquqi_huidiao');
                shuru.value = '';
            }
        } finally {
            // 流式模式：无论成功失败，保存已收到的内容并清理临时DOM
            if (this.luoji.dangqianmoshi === 'liushi') {
                if (this.liushihuifu) {
                    this.luoji.tianjiaxiaoxi('assistant', this.liushihuifu);
                } else {
                    // 没有收到任何AI回复，移除用户消息
                    this.luoji.shanchuzuihouyonghuxiaoxi();
                }
                this.qingchulishilinshi();
                this.xuanranhuihualiebiao();
                this.xuanranduihua();
            }
            this.yichujiazai();
            this.zhengzaifasong = false;
            this.shezhibtnzhuangtai(false);
        }
    }

    // 终止请求
    async zhongzhi() {
        console.log('[DEBUG] jiemian.zhongzhi 被调用');
        console.log('[DEBUG] dangqianmoshi:', this.luoji.dangqianmoshi);
        if (this.luoji.dangqianmoshi === 'liushi') {
            await this.luoji.zhongzhiliushi();
        } else {
            // 非流式也支持终止
            await this.luoji.zhongzhiliushi();
        }
    }

    liushihuidiao(shuju) {
        // 收到数据后移除加载提示
        this.yichujiazai();
        // 解析 SSE 格式: data: {"neirong":"xxx"}\n\n
        try {
            const lines = shuju.split('\n');
            for (const line of lines) {
                if (line.startsWith('data: ')) {
                    const jsonStr = line.substring(6).trim();
                    if (!jsonStr) continue;
                    
                    const json = JSON.parse(jsonStr);
                    
                    // 检查错误
                    if (json.cuowu) {
                        this.luoji.rizhi('流式错误: ' + json.cuowu, 'err');
                        continue;
                    }
                    
                    // 累积内容
                    if (json.neirong) {
                        this.liushihuifu += json.neirong;
                    }
                }
            }
        } catch (e) {
            this.luoji.rizhi('解析流式数据失败: ' + e, 'warn');
            return;
        }
        
        // 实时显示（在对话区域底部追加）
        const quyu = document.getElementById('aiduihua_quyu');
        if (!quyu) return;

        // 查找或创建流式显示区域
        let liushiqu = document.getElementById('aiduihua_liushi_linshi');
        if (!liushiqu) {
            liushiqu = document.createElement('div');
            liushiqu.id = 'aiduihua_liushi_linshi';
            liushiqu.style.cssText = 'display:flex;justify-content:flex-start;margin-bottom:12px';
            liushiqu.innerHTML = `
                <div style="max-width:80%;background:#F0FDF4;border-radius:8px;padding:10px">
                    <div style="font-size:12px;font-weight:600;color:#10B981;margin-bottom:4px">AI</div>
                    <div id="aiduihua_liushi_neirong" style="font-size:13px;color:#1E293B;white-space:pre-wrap;word-break:break-word"></div>
                </div>
            `;
            quyu.appendChild(liushiqu);
        }

        const neirongqu = document.getElementById('aiduihua_liushi_neirong');
        if (neirongqu) {
            neirongqu.textContent = this.liushihuifu;
        }

        // 滚动到底部
        quyu.scrollTop = quyu.scrollHeight;
    }

    qingchulishilinshi() {
        const linshi = document.getElementById('aiduihua_liushi_linshi');
        if (linshi) {
            linshi.remove();
        }
        this.liushihuifu = '';
    }

    qingkonglishi() {
        if (confirm('确定要清空当前对话历史吗？')) {
            this.luoji.qingkonglishi();
            this.xuanranhuihualiebiao();
            this.xuanranduihua();
        }
    }

    shanchuxiaoxi(suoyin) {
        if (confirm('确定要删除这条消息吗？')) {
            this.luoji.shanchuxiaoxi(suoyin);
            this.xuanranduihua();
        }
    }

    daochulishi() {
        this.luoji.daochulishi();
    }

    // 新建会话
    xinjianhiuhua() {
        this.luoji.xinjianhiuhua();
        this.xuanranhuihualiebiao();
        this.xuanranduihua();
    }

    // 切换会话
    qiehuanhuihua(id) {
        this.luoji.qiehuanhuihua(id);
        this.xuanranhuihualiebiao();
        this.xuanranduihua();
    }

    // 删除会话
    shanchuhuihua(id) {
        if (confirm('确定要删除这个对话吗？')) {
            this.luoji.shanchuhuihua(id);
            this.xuanranhuihualiebiao();
            this.xuanranduihua();
        }
    }

    // 重命名会话
    chongmingming(id) {
        const liebiao = this.luoji.huoquhuihualiebiao();
        const huihua = liebiao.find(h => h.id === id);
        if (!huihua) return;
        const xinming = prompt('请输入新名称:', huihua.mingcheng);
        if (xinming && xinming.trim()) {
            this.luoji.chongmingminghuihua(id, xinming.trim());
            this.xuanranhuihualiebiao();
        }
    }
}
