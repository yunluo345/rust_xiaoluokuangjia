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
                <button class="aq-btn aq-btn-xiao" onclick="aiduihua_qingkonglishi()">清空历史</button>
                <button class="aq-btn aq-btn-xiao aq-btn-huang" onclick="aiduihua_daochulishi()">导出历史</button>
            </div>
        `;
        this.rongqi.appendChild(tou);

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
        duihuaqu.style.cssText = 'background:#F8FAFC;border:1px solid #E2E8F0;border-radius:8px;padding:12px;max-height:400px;overflow-y:auto;margin-bottom:12px;min-height:200px';
        this.rongqi.appendChild(duihuaqu);

        // 输入区域
        const shuruqu = document.createElement('div');
        shuruqu.style.cssText = 'display:flex;gap:8px;align-items:flex-end';
        shuruqu.innerHTML = `
            <textarea id="aiduihua_shuru" placeholder="输入消息..." style="flex:1;border:1px solid #E2E8F0;border-radius:8px;padding:10px;font-size:14px;resize:vertical;min-height:60px;outline:none;font-family:inherit"></textarea>
            <button id="aiduihua_fasong_btn" class="aq-btn aq-btn-lv" onclick="aiduihua_fasong()" style="min-height:60px">发送</button>
        `;
        this.rongqi.appendChild(shuruqu);

        // 渲染历史记录
        this.xuanranduihua();
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

    async fasong() {
        if (this.zhengzaifasong) {
            this.luoji.rizhi('正在发送中，请稍候', 'warn');
            return;
        }

        const shuru = document.getElementById('aiduihua_shuru');
        const btn = document.getElementById('aiduihua_fasong_btn');
        const neirong = shuru.value.trim();

        if (!neirong) {
            this.luoji.rizhi('消息内容不能为空', 'warn');
            return;
        }

        this.zhengzaifasong = true;
        btn.disabled = true;
        btn.textContent = '发送中...';

        try {
            if (this.luoji.dangqianmoshi === 'feiliushi') {
                // 非流式
                const huifu = await this.luoji.feiliushiduihua(neirong);
                if (huifu) {
                    shuru.value = '';
                    this.xuanranduihua();
                }
            } else {
                // 流式
                this.liushihuifu = '';
                const chenggong = await this.luoji.liushiduihua(neirong, 'aiduihua_liushi_huidiao');
                if (chenggong) {
                    shuru.value = '';
                    // 流式完成后添加完整回复到历史
                    if (this.liushihuifu) {
                        this.luoji.tianjiaxiaoxi('assistant', this.liushihuifu);
                    }
                    this.xuanranduihua();
                }
            }
        } finally {
            this.zhengzaifasong = false;
            btn.disabled = false;
            btn.textContent = '发送';
        }
    }

    liushihuidiao(shuju) {
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
        if (confirm('确定要清空所有对话历史吗？')) {
            this.luoji.qingkonglishi();
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
}
