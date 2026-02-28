// AI对话管理 - 界面层
export class Aiduihuajiemian {
    constructor(luoji, rongqiid) {
        this.luoji = luoji;
        this.rongqi = document.getElementById(rongqiid);
        this.liushihuifu = ''; // 流式回复缓存
        this.liushisikao = ''; // 流式思考内容缓存
        this.liushishijianlie = []; // 流式事件消息列表
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
                <button class="aq-btn aq-btn-xiao" onclick="aiduihua_xinjianhuihua()" style="margin:0 4px 0 0">新建对话</button>
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
        duihuaqu.style.cssText = 'background:#F8FAFC;border-radius:8px;padding:12px;max-height:400px;overflow-y:auto;margin-bottom:12px;min-height:200px';
        this.rongqi.appendChild(duihuaqu);

        // 输入区域
        const shuruqu = document.createElement('div');
        shuruqu.style.cssText = 'display:flex;gap:8px;align-items:stretch';
        shuruqu.innerHTML = `
            <textarea id="aiduihua_shuru" placeholder="输入消息..." style="flex:1;border:none;border-radius:8px;padding:10px;font-size:14px;resize:vertical;min-height:60px;outline:none;font-family:inherit;background:#F8FAFC;color:#1E293B"></textarea>
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
            const zhongliang = xuanzhong ? '600' : '500';
            html += `
                <div style="display:flex;align-items:center;gap:4px;flex-shrink:0">
                    <button class="aq-btn aq-btn-xiao" onclick="aiduihua_qiehuanhuihua('${h.id}')" style="font-weight:${zhongliang};white-space:nowrap;max-width:140px;overflow:hidden;text-overflow:ellipsis;margin:0;min-height:32px" title="${this.zhuanyihtml(h.mingcheng)}">${this.zhuanyihtml(h.mingcheng)}</button>
                    <button class="aq-btn aq-btn-xiao" onclick="aiduihua_chongmingming('${h.id}')" style="font-size:13px;padding:4px 6px;margin:0;min-height:28px" title="重命名">✏</button>
                    <button class="aq-btn aq-btn-xiao" onclick="aiduihua_shanchuhuihua('${h.id}')" style="font-size:13px;padding:4px 6px;margin:0;min-height:28px" title="删除">✕</button>
                </div>
            `;
        });
        lan.innerHTML = html;
    }

    shifoushijian(neirong) {
        return /^\[(\u610f\u56fe|\u8fdb\u5ea6|\u5de5\u5177\u8c03\u7528|\u5de5\u5177\u7ed3\u679c)\]/.test(neirong);
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
            const shiShijian = !shiuser && this.shifoushijian(xiaoxi.neirong);

            if (shiShijian) {
                html += this.shengchengshijianhtml(xiaoxi.neirong);
            } else {
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
                            ${xiaoxi.sikao ? this.shengchengsikaohtml(xiaoxi.sikao, '思考过程') : ''}
                            <div style="font-size:13px;color:#1E293B;white-space:pre-wrap;word-break:break-word">${this.zhuanyihtml(xiaoxi.neirong)}</div>
                        </div>
                    </div>`;
            }
        });

        quyu.innerHTML = html;
        quyu.scrollTop = quyu.scrollHeight;
    }
    shuaxinquanbu() {
        this.xuanranhuihualiebiao();
        this.xuanranduihua();
    }

    shengchengsikaohtml(neirong, biaoti = '思考过程') {
        return `<details style="margin-bottom:6px;border:none;border-radius:6px;padding:4px 8px;background:#FAF5FF"><summary style="cursor:pointer;font-size:12px;color:#7C3AED;user-select:none">💭 ${this.zhuanyihtml(biaoti)}</summary><div style="font-size:12px;color:#6B21A8;white-space:pre-wrap;word-break:break-word;margin-top:4px">${this.zhuanyihtml(neirong)}</div></details>`;
    }

    shengchengshijianhtml(neirong) {
        return `<div style="display:flex;justify-content:flex-start;margin-bottom:6px"><div style="background:#F5F3FF;border:none;border-radius:16px;padding:5px 12px"><span style="font-size:12px;color:#7C3AED">${this.zhuanyihtml(neirong)}</span></div></div>`;
    }

    tianjialinshiqipao(html, id = '', classname = 'aiduihua_shijian_linshi') {
        const quyu = document.getElementById('aiduihua_quyu');
        if (!quyu) return null;
        const qipao = document.createElement('div');
        if (id) qipao.id = id;
        if (classname) qipao.className = classname;
        qipao.style.cssText = 'display:flex;justify-content:flex-start;margin-bottom:6px';
        qipao.innerHTML = html;
        quyu.appendChild(qipao);
        quyu.scrollTop = quyu.scrollHeight;
        return qipao;
    }

    zhuanyihtml(wenben) {
        return String(wenben ?? '')
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
        if (document.getElementById('aiduihua_jiazai_linshi')) return;
        this.tianjialinshiqipao(`<div style="max-width:80%;background:#F0FDF4;border-radius:8px;padding:10px"><div style="font-size:12px;font-weight:600;color:#10B981;margin-bottom:4px">AI</div><div style="font-size:13px;color:#94A3B8">正在请求...</div></div>`, 'aiduihua_jiazai_linshi', '');
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

        // 立即显示用户消息并清空输入框
        this.luoji.tianjiaxiaoxi('user', neirong);
        shuru.value = '';
        this.shuaxinquanbu();
        this.xianshijiazai();

        try {
            if (this.luoji.dangqianmoshi === 'feiliushi') {
                await this.luoji.feiliushiduihua(neirong);
                this.shuaxinquanbu();
            } else {
                this.liushihuifu = '';
                this.liushisikao = '';
                await this.luoji.liushiduihua(neirong, 'aiduihua_liushi_huidiao', 'aiduihua_duquqi_huidiao');
            }
        } finally {
            if (this.luoji.dangqianmoshi === 'liushi') {
                for (const sj of this.liushishijianlie) {
                    this.luoji.tianjiaxiaoxi('assistant', sj);
                }
                if (this.liushihuifu) {
                    this.luoji.tianjiaxiaoxi('assistant', this.liushihuifu, this.liushisikao || null);
                } else if (this.liushishijianlie.length === 0) {
                    this.luoji.shanchuzuihouyonghuxiaoxi();
                }
                this.qingchulishilinshi();
                this.shuaxinquanbu();
            }
            this.yichujiazai();
            this.zhengzaifasong = false;
            this.shezhibtnzhuangtai(false);
        }
    }

    // 终止请求
    async zhongzhi() {
        await this.luoji.zhongzhiliushi();
    }

    tianjiasikaoqipao(neirong, biaoti) {
        this.tianjialinshiqipao(this.shengchengsikaohtml(neirong, biaoti || '思考过程'));
    }

    tianjiashijianqipao(neirong) {
        this.tianjialinshiqipao(this.shengchengshijianhtml(neirong));
    }

    chuliliushishijian(json) {
        const shijianpeizhi = {
            yitu: { qianzhui: '[意图] ', ziduan: 'yitu', sikaobiaoti: '意图分析思考' },
            xunhuan: { qianzhui: '[进度] ', ziduan: 'neirong' },
            gongjudiaoyong: { qianzhui: '[工具调用] ', ziduan: 'neirong' },
            gongjujieguo: { qianzhui: '[工具结果] ', ziduan: 'neirong' },
        };
        const peizhi = shijianpeizhi[json.shijian];
        if (!peizhi) return false;
        const yuanwen = json[peizhi.ziduan];
        if (!yuanwen) return true;
        const sj = peizhi.qianzhui + yuanwen;
        this.liushishijianlie.push(sj);
        this.tianjiashijianqipao(sj);
        if (peizhi.sikaobiaoti && json.sikao) {
            this.tianjiasikaoqipao(json.sikao, peizhi.sikaobiaoti);
        }
        return true;
    }

    liushihuidiao(shuju) {
        this.yichujiazai();
        try {
            const lines = shuju.split('\n');
            for (const line of lines) {
                if (!line.startsWith('data: ')) continue;
                const jsonStr = line.substring(6).trim();
                if (!jsonStr) continue;

                const json = JSON.parse(jsonStr);
                if (json.cuowu) {
                    this.luoji.rizhi('流式错误: ' + json.cuowu, 'err');
                    continue;
                }
                if (this.chuliliushishijian(json)) continue;
                if (json.shijian === 'sikao' && json.neirong) {
                    this.liushisikao += json.neirong;
                    this.gengxinliushisikao();
                    continue;
                }
                if (json.neirong) {
                    this.liushihuifu += json.neirong;
                }
            }
        } catch (e) {
            this.luoji.rizhi('解析流式数据失败: ' + e, 'warn');
            return;
        }

        // 只有有文字内容时才显示流式文字气泡
        if (this.liushihuifu) {
            const quyu = document.getElementById('aiduihua_quyu');
            if (!quyu) return;

            let liushiqu = document.getElementById('aiduihua_liushi_linshi');
            if (!liushiqu) {
                liushiqu = this.tianjialinshiqipao(`<div style="max-width:80%;background:#F0FDF4;border-radius:8px;padding:10px"><div style="font-size:12px;font-weight:600;color:#10B981;margin-bottom:4px">AI</div><div id="aiduihua_liushi_neirong" style="font-size:13px;color:#1E293B;white-space:pre-wrap;word-break:break-word"></div></div>`, 'aiduihua_liushi_linshi', '');
            }

            const neirongqu = document.getElementById('aiduihua_liushi_neirong');
            if (neirongqu) {
                neirongqu.textContent = this.liushihuifu;
            }
            quyu.scrollTop = quyu.scrollHeight;
        }
    }

    // 更新流式思考内容显示
    gengxinliushisikao() {
        const quyu = document.getElementById('aiduihua_quyu');
        if (!quyu || !this.liushisikao) return;

        let sikaoqu = document.getElementById('aiduihua_liushi_sikao');
        if (!sikaoqu) {
            sikaoqu = document.createElement('div');
            sikaoqu.id = 'aiduihua_liushi_sikao';
            sikaoqu.style.cssText = 'display:flex;justify-content:flex-start;margin-bottom:6px';
            sikaoqu.innerHTML = `
                <details open style="max-width:80%;border:none;border-radius:6px;padding:6px 10px;background:#FAF5FF">
                    <summary style="cursor:pointer;font-size:12px;color:#7C3AED;user-select:none">💭 思考中...</summary>
                    <div id="aiduihua_liushi_sikao_neirong" style="font-size:12px;color:#6B21A8;white-space:pre-wrap;word-break:break-word;margin-top:4px"></div>
                </details>
            `;
            quyu.appendChild(sikaoqu);
        }
        const nr = document.getElementById('aiduihua_liushi_sikao_neirong');
        if (nr) nr.textContent = this.liushisikao;
        quyu.scrollTop = quyu.scrollHeight;
    }

    qingchulishilinshi() {
        const linshi = document.getElementById('aiduihua_liushi_linshi');
        if (linshi) linshi.remove();
        const sikaolinshi = document.getElementById('aiduihua_liushi_sikao');
        if (sikaolinshi) sikaolinshi.remove();
        document.querySelectorAll('.aiduihua_shijian_linshi').forEach(el => el.remove());
        this.liushihuifu = '';
        this.liushisikao = '';
        this.liushishijianlie = [];
    }

    async qingkonglishi() {
        if (!await aqqueren('清空对话', '确定要清空当前对话历史吗？')) return;
        this.luoji.qingkonglishi();
        this.shuaxinquanbu();
    }

    async shanchuxiaoxi(suoyin) {
        if (!await aqqueren('删除消息', '确定要删除这条消息吗？')) return;
        this.luoji.shanchuxiaoxi(suoyin);
        this.xuanranduihua();
    }

    daochulishi() {
        const json = this.luoji.daochulishi();
        const blob = new Blob([json], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'ai_duihua_lishi_' + new Date().getTime() + '.json';
        a.click();
        URL.revokeObjectURL(url);
        this.luoji.rizhi('历史记录已导出', 'ok');
    }

    // 新建会话
    xinjianhuihua() {
        this.luoji.xinjianhuihua();
        this.shuaxinquanbu();
    }

    // 切换会话
    qiehuanhuihua(id) {
        this.luoji.qiehuanhuihua(id);
        this.shuaxinquanbu();
    }

    // 删除会话
    async shanchuhuihua(id) {
        if (!await aqqueren('删除对话', '确定要删除这个对话吗？')) return;
        this.luoji.shanchuhuihua(id);
        this.shuaxinquanbu();
    }

    async chongmingming(id) {
        const liebiao = this.luoji.huoquhuihualiebiao();
        const huihua = liebiao.find(h => h.id === id);
        if (!huihua) return;
        const xinming = await aqshuru('重命名对话', '请输入新名称', huihua.mingcheng, '输入对话名称');
        if (xinming && xinming.trim()) {
            this.luoji.chongmingminghuihua(id, xinming.trim());
            this.xuanranhuihualiebiao();
        }
    }
}
