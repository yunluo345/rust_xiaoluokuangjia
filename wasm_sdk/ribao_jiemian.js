// 日报管理 - 界面层
export class Ribaojiemian {
    constructor(luoji, rongqiid) {
        this.luoji = luoji;
        this.rongqi = document.getElementById(rongqiid);
        this.dangqianshitu = 'ribao';
        this.xuanzhongid = null;
        this.dangqianyeshu = 1;
        this.meiyetiaoshu = 10;
        this.shifouquanxian = !!(luoji && luoji.shifouquanxian);
        this.chakanquanbu = false;
        this.sousuobiaoqianid = null;
        this.sousuoleixing = null;
        this.sousuoguanjiancizhi = null;
        this.sousuoyonghuid = null;
    }

    async xuanran() {
        this.rongqi.innerHTML = '';
        const tou = document.createElement('div');
        tou.style.cssText = 'display:flex;justify-content:space-between;align-items:center;margin-bottom:12px';
        const anniulie = this.shifouquanxian
            ? `<button class="aq-btn aq-btn-zhu" onclick="ribao_qiehuanshitu('ribao')">日报</button>
               <button class="aq-btn aq-btn-zhu" onclick="ribao_qiehuanshitu('biaoqian')">标签</button>
               <button class="aq-btn aq-btn-zhu" onclick="ribao_qiehuanshitu('leixing')">类型</button>
               <button class="aq-btn aq-btn-zhu" onclick="ribao_qiehuanshitu('renwu')">任务</button>
               <button class="aq-btn aq-btn-lv" onclick="ribao_shuaxin()">刷新数据</button>`
            : `<button class="aq-btn ${this.dangqianshitu === 'ribao' ? 'aq-btn-lv' : 'aq-btn-zhu'}" onclick="ribao_qiehuanshitu('ribao')">我的日报</button>
               <button class="aq-btn ${this.dangqianshitu === 'quanburibao' ? 'aq-btn-lv' : 'aq-btn-zhu'}" onclick="ribao_qiehuanshitu('quanburibao')">全部日报</button>
               <button class="aq-btn ${this.dangqianshitu === 'renwu' ? 'aq-btn-lv' : 'aq-btn-zhu'}" onclick="ribao_qiehuanshitu('renwu')">任务</button>
               <button class="aq-btn aq-btn-lv" onclick="ribao_shuaxin()">刷新数据</button>`;
        tou.innerHTML = `<h2 style="font-size:15px;color:#475569;margin:0">日报管理</h2><div>${anniulie}</div>`;
        this.rongqi.appendChild(tou);
        const neirong = document.createElement('div');
        neirong.id = 'ribao_neirong';
        neirong.innerHTML = '<p style="color:#94A3B8;font-size:14px">点击「刷新数据」加载日报信息</p>';
        this.rongqi.appendChild(neirong);
    }

    async qiehuanshitu(shitu) {
        const yunxushitu = this.shifouquanxian
            ? ['ribao', 'biaoqian', 'leixing', 'renwu']
            : ['ribao', 'quanburibao', 'renwu'];
        this.dangqianshitu = yunxushitu.includes(shitu) ? shitu : 'ribao';
        this.chakanquanbu = this.dangqianshitu === 'quanburibao';
        await this.xuanran();
        await this.shuaxindangqianshitu();
    }

    async shuaxindangqianshitu() {
        const shitumap = {
            'ribao': () => this.shuaxinribaoliebiao(),
            'quanburibao': () => this.shuaxinribaoliebiao(),
            'biaoqian': () => this.shuaxinbiaoqianliebiao(),
            'leixing': () => this.shuaxinleixingliebiao(),
            'renwu': () => this.shuaxinrenwuliebiao()
        };
        const fn = shitumap[this.dangqianshitu];
        if (fn) await fn();
    }

    async shuaxinribaoliebiao() {
        const nr = document.getElementById('ribao_neirong');
        let jg, liebiao, zongshu;
        if (this.sousuobiaoqianid) {
            jg = await this.luoji.guanlian_chaxun_biaoqianid(this.sousuobiaoqianid);
            liebiao = jg?.zhuangtaima === 200 ? jg.shuju || [] : [];
            zongshu = liebiao.length;
        } else if (this.sousuoleixing) {
            jg = await this.luoji.guanlian_chaxun_leixingmingcheng_zhi(this.sousuoleixing.mc, this.sousuoleixing.z);
            liebiao = jg?.zhuangtaima === 200 ? jg.shuju || [] : [];
            zongshu = liebiao.length;
        } else if (this.sousuoguanjiancizhi) {
            jg = await this.luoji.guanjiancichaxunfenye_shipei(this.sousuoguanjiancizhi, this.dangqianyeshu, this.meiyetiaoshu);
            liebiao = jg?.zhuangtaima === 200 ? jg.shuju?.liebiao || [] : [];
            zongshu = jg?.zhuangtaima === 200 ? jg.shuju?.zongshu || 0 : 0;
        } else if (this.sousuoyonghuid) {
            jg = await this.luoji.ribao_chaxun_yonghuid_fenye(this.sousuoyonghuid, this.dangqianyeshu, this.meiyetiaoshu);
            liebiao = jg?.zhuangtaima === 200 ? jg.shuju?.liebiao || [] : [];
            zongshu = jg?.zhuangtaima === 200 ? jg.shuju?.zongshu || 0 : 0;
        } else {
            jg = await this.luoji.chaxunfenye_shipei(this.dangqianyeshu, this.meiyetiaoshu, this.chakanquanbu);
            liebiao = jg?.zhuangtaima === 200 ? jg.shuju?.liebiao || [] : [];
            zongshu = jg?.zhuangtaima === 200 ? jg.shuju?.zongshu || 0 : 0;
        }
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = `<p style="color:#EF4444">加载失败: ${jg ? jg.xiaoxi : '请求错误'}</p>`;
            return;
        }

        const biaoqianjg = await this.luoji.biaoqian_chaxun_quanbu();
        const suoyoubiaoqian = biaoqianjg?.zhuangtaima === 200 ? biaoqianjg.shuju || [] : [];

        let html = '<div style="margin-bottom:16px">';
        html += '<div style="display:flex;gap:10px;align-items:center;flex-wrap:wrap">';
        html += '<button class="aq-btn aq-btn-lv" onclick="ribao_xinzengshitu()" style="height:36px">新增日报</button>';
        html += '<div style="height:20px;width:1px;background:#E2E8F0"></div>';
        html += '<input id="rb_gjc" type="text" value="' + (this.sousuoguanjiancizhi || '') + '" onkeydown="ribao_sousuoshuru_huiche(event)" placeholder="搜索内容关键词" style="height:36px;padding:0 12px;border:1px solid #E2E8F0;border-radius:6px;width:160px;font-size:13px;box-sizing:border-box">';
        html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_sousuoguanjianci()" style="height:36px">搜索</button>';
        html += '<div style="height:20px;width:1px;background:#E2E8F0"></div>';
        html += '<input id="rb_yhid" type="text" placeholder="用户ID" style="height:36px;padding:0 12px;border:1px solid #E2E8F0;border-radius:6px;width:120px;font-size:13px;box-sizing:border-box">';
        html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_sousuoyonghuid_xuanze()" style="height:36px">查询</button>';

        html += '<div style="height:20px;width:1px;background:#E2E8F0"></div>';
        html += '<input id="rb_bqxz" type="text" placeholder="标签关键词筛选" style="height:36px;padding:0 12px;border:1px solid #E2E8F0;border-radius:6px;width:160px;font-size:13px;box-sizing:border-box">';
        html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_sousuobiaoqian_xuanze()" style="height:36px">筛选</button>';

        if (this.sousuobiaoqianid || this.sousuoleixing || this.sousuoguanjiancizhi || this.sousuoyonghuid) {
            html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_qingchusousuo()" style="height:36px">清除筛选</button>';
        }
        html += '</div></div>';

        if (jg?.zhuangtaima === 200 && this.sousuoguanjiancizhi && !this.shifouquanxian) {
            this.luoji.rizhi('普通用户关键词搜索按当前用户分页展示', 'info');
        }

        if (liebiao.length === 0) {
            nr.innerHTML = html + '<p style="color:#64748B">暂无日报数据</p>';
            return;
        }

        html += '<div style="display:flex;flex-direction:column;gap:12px">';
        nr.innerHTML = html + '<p style="color:#64748B">加载中...</p></div>';

        const ribaobiaoqianmap = {};
        for (const rb of liebiao) {
            const gljg = await this.luoji.guanlian_chaxun_ribaoid_daixinxi(rb.id);
            ribaobiaoqianmap[rb.id] = gljg?.zhuangtaima === 200 ? gljg.shuju || [] : [];
        }

        html = '<div style="margin-bottom:16px">';
        html += '<div style="display:flex;gap:10px;align-items:center;flex-wrap:wrap">';
        html += '<button class="aq-btn aq-btn-lv" onclick="ribao_xinzengshitu()" style="height:36px">新增日报</button>';
        html += '<div style="height:20px;width:1px;background:#E2E8F0"></div>';
        html += '<input id="rb_gjc" type="text" placeholder="搜索内容关键词" style="height:36px;padding:0 12px;border:1px solid #E2E8F0;border-radius:6px;width:160px;font-size:13px;box-sizing:border-box">';
        html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_sousuoguanjianci()" style="height:36px">搜索</button>';
        html += '<div style="height:20px;width:1px;background:#E2E8F0"></div>';
        html += '<input id="rb_yhid" type="text" placeholder="用户ID" style="height:36px;padding:0 12px;border:1px solid #E2E8F0;border-radius:6px;width:120px;font-size:13px;box-sizing:border-box">';
        html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_sousuoyonghuid_xuanze()" style="height:36px">查询</button>';

        html += '<div style="height:20px;width:1px;background:#E2E8F0"></div>';
        html += '<input id="rb_bqxz" type="text" placeholder="标签关键词筛选" style="height:36px;padding:0 12px;border:1px solid #E2E8F0;border-radius:6px;width:160px;font-size:13px;box-sizing:border-box">';
        html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_sousuobiaoqian_xuanze()" style="height:36px">筛选</button>';

        if (this.sousuobiaoqianid || this.sousuoleixing || this.sousuoguanjiancizhi || this.sousuoyonghuid) {
            html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_qingchusousuo()" style="height:36px">清除筛选</button>';
        }
        html += '</div></div>';

        this._ribaoshujuhuancun = {};
        for (const rb of liebiao) this._ribaoshujuhuancun[rb.id] = rb;

        html += '<div style="display:flex;flex-direction:column;gap:12px">';
        for (const rb of liebiao) {
            const shifouchaochu = rb.neirong.length > 100;
            const jiedneirong = shifouchaochu ? rb.neirong.substring(0, 100) + '...' : rb.neirong;
            const biaoqianlie = ribaobiaoqianmap[rb.id] || [];

            let neironghtml = `<div id="rb_nr_${rb.id}" style="font-size:14px;color:#1E293B;line-height:1.6;white-space:pre-wrap">${jiedneirong}</div>`;
            if (shifouchaochu) {
                neironghtml += `<span id="rb_zk_${rb.id}" onclick="ribao_qiehuanneirong('${rb.id}')" style="color:#3B82F6;font-size:12px;cursor:pointer;user-select:none;display:inline-block;margin-top:4px" onmouseover="this.style.color='#2563EB'" onmouseout="this.style.color='#3B82F6'">查看完整内容 ▼</span>`;
            }

            html += `<div style="background:#FFFFFF;border:1px solid #E2E8F0;border-radius:8px;padding:14px">
                <div style="display:flex;justify-content:space-between;align-items:start;margin-bottom:10px">
                    <div style="flex:1">
                        <div style="font-size:12px;color:#64748B;margin-bottom:4px">ID: ${rb.id} | 发布者: ${rb.fabuzhemingcheng || rb.fabuzhezhanghao || rb.yonghuid}${rb.fabuzhezhanghao ? '（' + rb.fabuzhezhanghao + '）' : ''} | ${rb.fabushijian}</div>
                        ${neironghtml}
                    </div>
                    <div style="display:flex;gap:6px;margin-left:12px">
                        <button class="aq-btn aq-btn-xiao aq-btn-huang" onclick="ribao_bianji('${rb.id}')">编辑</button>
                        <button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_shanchu('${rb.id}')">删除</button>
                        <button class="aq-btn aq-btn-xiao" onclick="ribao_guanlianguanlian('${rb.id}')">管理标签</button>
                    </div>
                </div>`;

            if (biaoqianlie.length > 0) {
                html += '<div style="display:flex;flex-wrap:wrap;gap:6px;margin-top:8px">';
                for (const bq of biaoqianlie) {
                    const leixing = bq.leixingmingcheng || '未知';
                    const zhi = bq.zhi || '';
                    html += `<span onclick="ribao_dianjibibaoqian('${leixing}','${zhi}')" style="display:inline-flex;align-items:center;gap:4px;padding:4px 10px;background:#EFF6FF;color:#1E40AF;border-radius:16px;font-size:12px;cursor:pointer;transition:background 200ms" onmouseover="this.style.background='#DBEAFE'" onmouseout="this.style.background='#EFF6FF'">
                        <span style="color:#64748B">${leixing}:</span>${zhi}
                    </span>`;
                }
                html += '</div>';
            } else {
                html += '<div style="font-size:12px;color:#94A3B8;margin-top:8px">暂无标签</div>';
            }

            html += '</div>';
        }
        html += '</div>';

        const zongyeshu = Math.max(1, Math.ceil(zongshu / this.meiyetiaoshu));
        html += `<div style="margin-top:12px;display:flex;gap:8px;align-items:center">
            <button class="aq-btn aq-btn-xiao" onclick="ribao_shangyiye()" ${this.dangqianyeshu <= 1 ? 'disabled' : ''}>上一页</button>
            <span style="color:#64748B">第 ${this.dangqianyeshu} / ${zongyeshu} 页</span>
            <button class="aq-btn aq-btn-xiao" onclick="ribao_xiayiye()" ${this.dangqianyeshu >= zongyeshu ? 'disabled' : ''}>下一页</button>
        </div>`;
        nr.innerHTML = html;
    }

    async shuaxinbiaoqianliebiao() {
        const jg = await this.luoji.biaoqian_chaxun_quanbu();
        const nr = document.getElementById('ribao_neirong');
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = `<p style="color:#EF4444">加载失败: ${jg ? jg.xiaoxi : '请求错误'}</p>`;
            return;
        }
        const liebiao = jg.shuju || [];
        let html = '<div style="margin-bottom:12px"><button class="aq-btn aq-btn-lv" onclick="ribao_xinzengbiaoqian()">新增标签</button></div>';
        if (liebiao.length === 0) {
            nr.innerHTML = html + '<p style="color:#64748B">暂无标签数据</p>';
            return;
        }
        html += '<div style="overflow-x:auto"><table class="aq-biao"><thead><tr>' +
            '<th>ID</th><th>类型ID</th><th>值</th><th>操作</th>' +
            '</tr></thead><tbody>';
        for (const bq of liebiao) {
            html += `<tr>
                <td>${bq.id}</td><td>${bq.leixingid}</td><td>${bq.zhi}</td>
                <td style="white-space:nowrap">
                    <button class="aq-btn aq-btn-xiao aq-btn-huang" onclick="ribao_bianjibiaoqian('${bq.id}')">编辑</button>
                    <button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_shanchubiaoqian('${bq.id}')">删除</button>
                </td></tr>`;
        }
        html += '</tbody></table></div>';
        nr.innerHTML = html;
    }

    async shuaxinleixingliebiao() {
        const jg = await this.luoji.leixing_chaxun_quanbu();
        const nr = document.getElementById('ribao_neirong');
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = `<p style="color:#EF4444">加载失败: ${jg ? jg.xiaoxi : '请求错误'}</p>`;
            return;
        }
        const liebiao = jg.shuju || [];
        let html = '<div style="margin-bottom:12px"><button class="aq-btn aq-btn-lv" onclick="ribao_xinzengleixing()">新增类型</button></div>';
        if (liebiao.length === 0) {
            nr.innerHTML = html + '<p style="color:#64748B">暂无类型数据</p>';
            return;
        }
        html += '<div style="overflow-x:auto"><table class="aq-biao"><thead><tr>' +
            '<th>ID</th><th>名称</th><th>操作</th>' +
            '</tr></thead><tbody>';
        for (const lx of liebiao) {
            html += `<tr>
                <td>${lx.id}</td><td>${lx.mingcheng}</td>
                <td style="white-space:nowrap">
                    <button class="aq-btn aq-btn-xiao aq-btn-huang" onclick="ribao_bianjileixing('${lx.id}')">编辑</button>
                    <button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_shanchuleixing('${lx.id}')">删除</button>
                </td></tr>`;
        }
        html += '</tbody></table></div>';
        nr.innerHTML = html;
    }

    xuanranxinzengribao() {
        const nr = document.getElementById('ribao_neirong');
        const yonghuru = this.shifouquanxian
            ? '<div class="aq-hang"><label>用户ID</label><input id="rb_yonghuid" type="text" placeholder="用户ID"></div>'
            : '';
        nr.innerHTML = `<div class="aq-biaodan">
            ${yonghuru}
            <div class="aq-hang"><label>内容</label><textarea id="rb_neirong" rows="5" placeholder="日报内容" style="border:1px solid #E2E8F0;border-radius:8px;padding:8px 12px;font-size:14px;outline:none;color:#1E293B;background:#fff;width:100%;resize:vertical"></textarea></div>
            <div class="aq-hang"><label>发布时间</label><input id="rb_fabushijian" type="datetime-local"></div>
            <div style="margin-top:12px">
                <button class="aq-btn aq-btn-lv" onclick="ribao_tijiaoxinzeng()">提交</button>
                <button class="aq-btn" onclick="ribao_quxiao()">取消</button>
            </div></div>`;
    }

    async tijiaoxinzeng() {
        const hq = id => document.getElementById(id)?.value?.trim() || '';
        const shuju = {
            yonghuid: this.shifouquanxian ? hq('rb_yonghuid') : '',
            neirong: hq('rb_neirong'),
            fabushijian: hq('rb_fabushijian')
        };
        if ((!this.shifouquanxian ? false : !shuju.yonghuid) || !shuju.neirong || !shuju.fabushijian) {
            this.luoji.rizhi('请填写所有必填字段', 'warn');
            return;
        }
        const jg = await this.luoji.ribao_xinzeng(shuju.yonghuid, shuju.neirong, shuju.fabushijian);
        if (jg && jg.zhuangtaima === 200) this.shuaxinribaoliebiao();
    }

    async shanchuribao(id) {
        if (!await aqqueren('删除日报', '确认删除此日报？')) return;
        const jg = await this.luoji.ribao_shanchu(id);
        if (jg && jg.zhuangtaima === 200) {
            this.shuaxinribaoliebiao();
        } else if (jg && jg.zhuangtaima === 403) {
            this.luoji.rizhi('权限不足：' + jg.xiaoxi, 'warn');
        }
    }

    xuanranxinzengbiaoqian() {
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = `<div class="aq-biaodan">
            <div class="aq-hang"><label>类型ID</label><input id="bq_leixingid" type="text" placeholder="类型ID"></div>
            <div class="aq-hang"><label>值</label><input id="bq_zhi" type="text" placeholder="标签值"></div>
            <div style="margin-top:12px">
                <button class="aq-btn aq-btn-lv" onclick="ribao_tijiaoxinzengbiaoqian()">提交</button>
                <button class="aq-btn" onclick="ribao_quxiao()">取消</button>
            </div></div>`;
    }

    async tijiaoxinzengbiaoqian() {
        const hq = id => document.getElementById(id)?.value?.trim() || '';
        const leixingid = hq('bq_leixingid');
        const zhi = hq('bq_zhi');
        if (!leixingid || !zhi) {
            this.luoji.rizhi('请填写所有必填字段', 'warn');
            return;
        }
        const jg = await this.luoji.biaoqian_xinzeng(leixingid, zhi);
        if (jg && jg.zhuangtaima === 200) {
            this.shuaxinbiaoqianliebiao();
        } else if (jg && jg.zhuangtaima === 403) {
            this.luoji.rizhi('权限不足：' + jg.xiaoxi, 'warn');
        }
    }

    async shanchubiaoqian(id) {
        if (!await aqqueren('删除标签', '确认删除此标签？')) return;
        const jg = await this.luoji.biaoqian_shanchu(id);
        if (jg && jg.zhuangtaima === 200) {
            this.shuaxinbiaoqianliebiao();
        } else if (jg && jg.zhuangtaima === 403) {
            this.luoji.rizhi('权限不足：' + jg.xiaoxi, 'warn');
        }
    }

    xuanranxinzengleixing() {
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = `<div class="aq-biaodan">
            <div class="aq-hang"><label>名称</label><input id="lx_mingcheng" type="text" placeholder="类型名称"></div>
            <div style="margin-top:12px">
                <button class="aq-btn aq-btn-lv" onclick="ribao_tijiaoxinzengleixing()">提交</button>
                <button class="aq-btn" onclick="ribao_quxiao()">取消</button>
            </div></div>`;
    }

    async tijiaoxinzengleixing() {
        const mingcheng = document.getElementById('lx_mingcheng')?.value?.trim() || '';
        if (!mingcheng) {
            this.luoji.rizhi('请填写类型名称', 'warn');
            return;
        }
        const jg = await this.luoji.leixing_xinzeng(mingcheng);
        if (jg && jg.zhuangtaima === 200) {
            this.shuaxinleixingliebiao();
        } else if (jg && jg.zhuangtaima === 403) {
            this.luoji.rizhi('权限不足：' + jg.xiaoxi, 'warn');
        }
    }

    async shanchuleixing(id) {
        if (!await aqqueren('删除类型', '确认删除此类型？')) return;
        const jg = await this.luoji.leixing_shanchu(id);
        if (jg && jg.zhuangtaima === 200) {
            this.shuaxinleixingliebiao();
        } else if (jg && jg.zhuangtaima === 403) {
            this.luoji.rizhi('权限不足：' + jg.xiaoxi, 'warn');
        }
    }

    async bianji(id) {
        const jg = await this.luoji.ribao_chaxun_id(id);
        if (!jg || jg.zhuangtaima !== 200) {
            this.luoji.rizhi('查询日报失败: ' + (jg ? jg.xiaoxi : '请求错误'), 'err');
            return;
        }
        const rb = jg.shuju;
        this.xuanzhongid = id;
        const nr = document.getElementById('ribao_neirong');
        const yonghuru = this.shifouquanxian
            ? `<div class="aq-hang"><label>用户ID</label><input id="rb_yonghuid" type="text" value="${rb.yonghuid}" readonly style="background:#F1F5F9"></div>`
            : '';
        nr.innerHTML = `<div class="aq-biaodan">
            ${yonghuru}
            <div class="aq-hang"><label>内容</label><textarea id="rb_neirong" rows="5" style="border:1px solid #E2E8F0;border-radius:8px;padding:8px 12px;font-size:14px;outline:none;color:#1E293B;background:#fff;width:100%;resize:vertical">${rb.neirong}</textarea></div>
            <div class="aq-hang"><label>发布时间</label><input id="rb_fabushijian" type="datetime-local" value="${rb.fabushijian.replace(' ', 'T').substring(0, 16)}"></div>
            <div style="margin-top:12px">
                <button class="aq-btn aq-btn-lv" onclick="ribao_tijiaobianji()">保存</button>
                <button class="aq-btn" onclick="ribao_quxiao()">取消</button>
            </div></div>`;
    }

    async tijiaobianji() {
        if (!this.xuanzhongid) {
            this.luoji.rizhi('未选中日报', 'warn');
            return;
        }
        const hq = id => document.getElementById(id)?.value?.trim() || '';
        const neirong = hq('rb_neirong');
        const fabushijian = hq('rb_fabushijian');
        if (!neirong || !fabushijian) {
            this.luoji.rizhi('请填写所有必填字段', 'warn');
            return;
        }
        const ziduanlie = [['neirong', neirong], ['fabushijian', fabushijian]];
        const jg = await this.luoji.ribao_gengxin(this.xuanzhongid, ziduanlie);
        if (jg && jg.zhuangtaima === 200) {
            this.xuanzhongid = null;
            this.shuaxinribaoliebiao();
        } else if (jg && jg.zhuangtaima === 403) {
            this.luoji.rizhi('权限不足：' + jg.xiaoxi, 'warn');
        }
    }

    async bianjibiaoqian(id) {
        this.luoji.rizhi('编辑标签功能暂未实现', 'info');
    }

    async bianjileixing(id) {
        this.luoji.rizhi('编辑类型功能暂未实现', 'info');
    }

    async guanlianguanlian(ribaoid) {
        this.xuanzhongid = ribaoid;
        this.xuanranbiaoqianguanlian(ribaoid);
    }

    async xuanranbiaoqianguanlian(ribaoid) {
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = '<p style="color:#64748B">加载中...</p>';
        const guanlianjg = await this.luoji.guanlian_chaxun_ribaoid_daixinxi(ribaoid);
        if (!guanlianjg || guanlianjg.zhuangtaima !== 200) {
            const cuowu = !guanlianjg ? '关联查询失败' : `关联查询错误: ${guanlianjg.xiaoxi}`;
            nr.innerHTML = `<p style="color:#EF4444">${cuowu}</p><button class="aq-btn" onclick="ribao_quxiao()">返回</button>`;
            return;
        }
        const biaoqianjg = await this.luoji.biaoqian_chaxun_quanbu();
        if (!biaoqianjg || biaoqianjg.zhuangtaima !== 200) {
            const cuowu = !biaoqianjg ? '标签查询失败' : `标签查询错误: ${biaoqianjg.xiaoxi}`;
            nr.innerHTML = `<p style="color:#EF4444">${cuowu}</p><button class="aq-btn" onclick="ribao_quxiao()">返回</button>`;
            return;
        }
        const yiguanlian = guanlianjg.shuju || [];
        const suoyoubiaoqian = biaoqianjg.shuju || [];
        const yiguanlianid = new Set(yiguanlian.map(g => g.biaoqianid));
        const weiguanlian = suoyoubiaoqian.filter(bq => !yiguanlianid.has(bq.id));

        let html = `<div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:16px">
            <h3 style="font-size:15px;color:#1E293B;margin:0">日报 #${ribaoid} 的标签</h3>
            <button class="aq-btn" onclick="ribao_quxiao()">返回列表</button>
        </div>`;

        // 已关联标签
        html += `<div style="background:#F8FAFC;border:1px solid #E2E8F0;border-radius:8px;padding:14px;margin-bottom:14px">
            <div style="font-size:13px;font-weight:600;color:#475569;margin-bottom:10px">已关联标签</div>`;
        if (yiguanlian.length === 0) {
            html += '<p style="color:#94A3B8;font-size:13px;margin:0">暂无关联标签</p>';
        } else {
            html += '<div style="display:flex;flex-direction:column;gap:10px">';
            for (const gl of yiguanlian) {
                const bq = suoyoubiaoqian.find(b => b.id === gl.biaoqianid);
                const leixing = gl.leixingmingcheng || '未知类型';
                const zhi = gl.zhi || (bq ? bq.zhi : gl.biaoqianid);
                html += `<div style="display:flex;align-items:center;gap:8px">
                    <span style="display:inline-flex;align-items:center;gap:6px;padding:6px 12px;background:#EFF6FF;color:#1E40AF;border-radius:20px;font-size:13px;line-height:1.2">
                        <span style="color:#64748B">${leixing}:</span> ${zhi}
                        <button onclick="ribao_shanchuguanlian('${ribaoid}','${gl.biaoqianid}')" style="background:none;border:none;color:#DC2626;cursor:pointer;padding:0;font-size:14px;line-height:1;display:flex;align-items:center">×</button>
                    </span>
                    <button class="aq-btn aq-btn-xiao" onclick="ribao_chakanguanlian('${gl.biaoqianid}','${leixing}')">相关标签</button>
                    <button class="aq-btn aq-btn-xiao" onclick="ribao_chakanleixingribao('${leixing}','${zhi}')">同类日报</button>
                </div>`;
            }
            html += '</div>';
        }
        html += '</div>';

        // 可添加标签
        html += `<div style="background:#F8FAFC;border:1px solid #E2E8F0;border-radius:8px;padding:14px">
            <div style="font-size:13px;font-weight:600;color:#475569;margin-bottom:10px">可添加标签</div>`;
        if (weiguanlian.length === 0) {
            html += '<p style="color:#94A3B8;font-size:13px;margin:0">所有标签已关联</p>';
        } else {
            html += '<div style="display:flex;flex-wrap:wrap;gap:8px">';
            for (const bq of weiguanlian) {
                html += `<button onclick="ribao_xinzengguanlian('${ribaoid}','${bq.id}')" style="padding:6px 12px;font-size:13px;background:#E0F2FE;color:#0369A1;border:1px solid #BAE6FD;border-radius:20px;cursor:pointer;line-height:1.2;transition:background 200ms" onmouseover="this.style.background='#BAE6FD'" onmouseout="this.style.background='#E0F2FE'">${bq.zhi}</button>`;
            }
            html += '</div>';
        }
        html += '</div>';

        nr.innerHTML = html;
    }

    async xinzengguanlian(ribaoid, biaoqianid) {
        const jg = await this.luoji.guanlian_xinzeng(ribaoid, biaoqianid);
        if (jg && jg.zhuangtaima === 200) {
            this.guanlianguanlian(ribaoid);
        }
    }

    async shanchuguanlian(ribaoid, biaoqianid) {
        const jg = await this.luoji.guanlian_shanchu(ribaoid, biaoqianid);
        if (jg && jg.zhuangtaima === 200) {
            this.guanlianguanlian(ribaoid);
        }
    }

    async chakanguanlian(biaoqianid, leixing) {
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = '<p style="color:#64748B">加载中...</p>';
        const jg = await this.luoji.guanlian_chaxun_xiangguanbiaoqian(biaoqianid, leixing);
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = `<p style="color:#EF4444">查询失败</p><button class="aq-btn" onclick="ribao_guanlianguanlian('${this.xuanzhongid}')">返回</button>`;
            return;
        }
        const liebiao = jg.shuju || [];
        let html = `<div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:16px">
            <h3 style="font-size:15px;color:#1E293B;margin:0">相关标签 (${leixing})</h3>
            <button class="aq-btn" onclick="ribao_guanlianguanlian('${this.xuanzhongid}')">返回</button>
        </div>`;
        if (liebiao.length === 0) {
            html += '<p style="color:#94A3B8">暂无相关标签</p>';
        } else {
            html += '<div style="display:flex;flex-wrap:wrap;gap:8px">';
            for (const bq of liebiao) {
                html += `<span style="padding:6px 12px;background:#EFF6FF;color:#1E40AF;border-radius:20px;font-size:13px">${bq.zhi}</span>`;
            }
            html += '</div>';
        }
        nr.innerHTML = html;
    }

    async chakanleixingribao(leixing, zhi) {
        this.sousuoleixing = { mc: leixing, z: zhi };
        this.sousuobiaoqianid = null;
        this.sousuoguanjiancizhi = null;
        this.dangqianyeshu = 1;
        this.shuaxinribaoliebiao();
    }

    quxiao() {
        this.shuaxindangqianshitu();
    }

    qiehuanneirong(id) {
        const nryuansu = document.getElementById(`rb_nr_${id}`);
        const zkanniu = document.getElementById(`rb_zk_${id}`);
        if (!nryuansu || !zkanniu) return;
        const rb = this._ribaoshujuhuancun?.[id];
        if (!rb) return;
        const dangqianzhankai = zkanniu.dataset.zhankai === '1';
        if (dangqianzhankai) {
            nryuansu.textContent = rb.neirong.substring(0, 100) + '...';
            zkanniu.textContent = '查看完整内容 ▼';
            zkanniu.dataset.zhankai = '0';
        } else {
            nryuansu.textContent = rb.neirong;
            zkanniu.textContent = '收起 ▲';
            zkanniu.dataset.zhankai = '1';
        }
    }

    shangyiye() {
        if (this.dangqianyeshu > 1) {
            this.dangqianyeshu--;
            this.shuaxinribaoliebiao();
        }
    }

    xiayiye() {
        this.dangqianyeshu++;
        this.shuaxinribaoliebiao();
    }

    xinzengshitu() {
        this.xuanranxinzengribao();
    }

    xinzengbiaoqian() {
        this.xuanranxinzengbiaoqian();
    }

    xinzengleixing() {
        this.xuanranxinzengleixing();
    }

    sousuoguanjianci() {
        const v = document.getElementById('rb_gjc')?.value?.trim();
        if (!v) {
            this.sousuoguanjiancizhi = null;
            this.dangqianyeshu = 1;
            this.shuaxinribaoliebiao();
            return;
        }
        this.sousuoguanjiancizhi = v;
        this.sousuobiaoqianid = null;
        this.sousuoleixing = null;
        this.sousuoyonghuid = null;
        this.dangqianyeshu = 1;
        this.shuaxinribaoliebiao();
    }

    sousuobiaoqian_xuanze() {
        const v = document.getElementById('rb_bqxz')?.value?.trim();
        if (!v) return this.luoji.rizhi('请输入标签关键词', 'warn');
        this.luoji.biaoqian_chaxun_quanbu().then(jg => {
            const liebiao = jg?.zhuangtaima === 200 ? jg.shuju || [] : [];
            const pipei = liebiao.find(bq => bq.zhi && bq.zhi.includes(v));
            this.sousuobiaoqianid = pipei ? pipei.id : '-1';
            this.sousuoleixing = null;
            this.sousuoguanjiancizhi = null;
            this.sousuoyonghuid = null;
            this.dangqianyeshu = 1;
            this.shuaxinribaoliebiao();
        });
    }

    sousuoyonghuid_xuanze() {
        const v = document.getElementById('rb_yhid')?.value?.trim();
        if (!v) return this.luoji.rizhi('请输入用户ID', 'warn');
        this.sousuoyonghuid = v;
        this.sousuobiaoqianid = null;
        this.sousuoleixing = null;
        this.sousuoguanjiancizhi = null;
        this.dangqianyeshu = 1;
        this.shuaxinribaoliebiao();
    }

    dianjibibaoqian(leixing, zhi) {
        this.sousuoleixing = { mc: leixing, z: zhi };
        this.sousuobiaoqianid = null;
        this.sousuoguanjiancizhi = null;
        this.sousuoyonghuid = null;
        this.dangqianyeshu = 1;
        this.shuaxinribaoliebiao();
    }

    qingchusousuo() {
        this.sousuobiaoqianid = null;
        this.sousuoleixing = null;
        this.sousuoguanjiancizhi = null;
        this.sousuoyonghuid = null;
        this.dangqianyeshu = 1;
        this.shuaxinribaoliebiao();
    }

    sousuoshuru_huiche(shijian) {
        if (shijian.key === 'Enter') {
            this.sousuoguanjianci();
        }
    }

    async shuaxinrenwuliebiao() {
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = '<p style="color:#64748B">加载中...</p>';
        const [dengdaijg, yichuli, kechuli, zhuangtaijg] = await Promise.all([
            this.luoji.renwu_chaxun_dengdai(50),
            this.luoji.renwu_tongji_zhuangtai('true'),
            this.luoji.renwu_tongji_kechuli(),
            this.luoji.renwu_biaoqian_ai_zhuangtai()
        ]);
        const liebiao = dengdaijg?.zhuangtaima === 200 ? dengdaijg.shuju || [] : [];
        const yichulishu = yichuli?.zhuangtaima === 200 ? yichuli.shuju?.count ?? 0 : 0;
        const kechulishu = kechuli?.zhuangtaima === 200 ? kechuli.shuju?.count ?? 0 : 0;
        const yunxingzhong = zhuangtaijg?.zhuangtaima === 200 && zhuangtaijg.shuju?.yunxingzhong === true;
        let html = `<div style="display:flex;gap:16px;margin-bottom:16px">
            <div style="background:#F0FDF4;border:1px solid #BBF7D0;border-radius:8px;padding:12px 20px;flex:1;text-align:center">
                <div style="font-size:24px;font-weight:600;color:#16A34A">${kechulishu}</div>
                <div style="font-size:12px;color:#4ADE80">待处理</div>
            </div>
            <div style="background:#EFF6FF;border:1px solid #BFDBFE;border-radius:8px;padding:12px 20px;flex:1;text-align:center">
                <div style="font-size:24px;font-weight:600;color:#2563EB">${yichulishu}</div>
                <div style="font-size:12px;color:#60A5FA">已处理</div>
            </div>
            <div style="background:${yunxingzhong ? '#ECFDF5' : '#F8FAFC'};border:1px solid ${yunxingzhong ? '#A7F3D0' : '#E2E8F0'};border-radius:8px;padding:12px 20px;flex:1;text-align:center">
                <div style="font-size:24px;font-weight:600;color:${yunxingzhong ? '#059669' : '#94A3B8'}">${yunxingzhong ? '●' : '○'}</div>
                <div style="font-size:12px;color:${yunxingzhong ? '#10B981' : '#94A3B8'}">${yunxingzhong ? '运行中' : '已停止'}</div>
            </div>
        </div>`;
        html += '<div style="margin-bottom:16px;display:flex;gap:8px;align-items:center">';
        html += '<input id="rw_ribaoid" type="text" placeholder="日报ID" style="height:36px;padding:0 12px;border:1px solid #E2E8F0;border-radius:6px;width:140px;font-size:13px;box-sizing:border-box">';
        html += '<button class="aq-btn aq-btn-lv" onclick="ribao_xinzengrenwu()" style="height:36px">新增任务</button>';
        html += '<div style="height:20px;width:1px;background:#E2E8F0"></div>';
        if (yunxingzhong) {
            html += '<button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_renwu_tingzhi()" style="height:36px">停止AI处理</button>';
        } else {
            html += '<button class="aq-btn aq-btn-zhu" onclick="ribao_renwu_qidong()" style="height:36px">启动AI处理</button>';
        }
        html += '</div>';
        if (liebiao.length === 0) {
            nr.innerHTML = html + '<p style="color:#64748B">暂无待处理任务</p>';
            return;
        }
        html += '<div style="overflow-x:auto"><table class="aq-biao"><thead><tr>' +
            '<th>任务ID</th><th>日报ID</th><th>用户ID</th><th>状态</th><th>尝试</th><th>标签结果</th><th>创建时间</th><th>操作</th>' +
            '</tr></thead><tbody>';
        for (const rw of liebiao) {
            const zhuangtai = rw.zhuangtai === 'true' ? '已处理' : '待处理';
            const zhuangtaiyanse = rw.zhuangtai === 'true' ? '#16A34A' : '#F59E0B';
            const jieguo = rw.biaoqianjieguo ? (rw.biaoqianjieguo.length > 30 ? rw.biaoqianjieguo.substring(0, 30) + '...' : rw.biaoqianjieguo) : '-';
            html += `<tr>
                <td>${rw.id}</td><td>${rw.ribaoid}</td><td>${rw.yonghuid}</td>
                <td><span style="color:${zhuangtaiyanse};font-weight:600">${zhuangtai}</span></td>
                <td>${rw.changshicishu}/${rw.zuidachangshicishu}</td>
                <td style="max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap" title="${(rw.biaoqianjieguo || '').replace(/"/g, '&quot;')}">${jieguo}</td>
                <td style="white-space:nowrap">${rw.chuangjianshijian}</td>
                <td style="white-space:nowrap">
                    <button class="aq-btn aq-btn-xiao aq-btn-huang" onclick="ribao_chongxinruidui('${rw.id}')">重新入队</button>
                    <button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_shanchurenwu('${rw.id}')">删除</button>
                </td>
            </tr>`;
        }
        html += '</tbody></table></div>';
        nr.innerHTML = html;
    }

    async chongxinruidui(id) {
        const jg = await this.luoji.renwu_chongxin_ruidui(id);
        if (jg && jg.zhuangtaima === 200) this.shuaxinrenwuliebiao();
    }

    async shanchurenwu(id) {
        if (!await aqqueren('删除任务', '确认删除此任务？')) return;
        const jg = await this.luoji.renwu_shanchu(id);
        if (jg && jg.zhuangtaima === 200) this.shuaxinrenwuliebiao();
    }

    async xinzengrenwu() {
        const ribaoid = document.getElementById('rw_ribaoid')?.value?.trim();
        if (!ribaoid) return this.luoji.rizhi('请输入日报ID', 'warn');
        const jg = await this.luoji.renwu_xinzeng(ribaoid);
        if (jg && jg.zhuangtaima === 200) this.shuaxinrenwuliebiao();
    }

    async renwu_qidong() {
        this.luoji.rizhi('正在启动AI标签处理...', 'info');
        this.luoji.renwu_biaoqian_ai_chuli().then(jg => {
            if (jg) this.shuaxinrenwuliebiao();
        });
        await new Promise(r => setTimeout(r, 500));
        await this.shuaxinrenwuliebiao();
    }

    async renwu_tingzhi() {
        const jg = await this.luoji.renwu_biaoqian_ai_tingzhi();
        if (jg?.zhuangtaima === 200) {
            await new Promise(r => setTimeout(r, 300));
            await this.shuaxinrenwuliebiao();
        }
    }

    async qiehuanquanbu() {
        this.chakanquanbu = !this.chakanquanbu;
        this.dangqianyeshu = 1;
        this.qingchusousuo();
        await this.xuanran();
        await this.shuaxinribaoliebiao();
    }

    shezhiquanxian(shifouquanxian) {
        this.shifouquanxian = !!shifouquanxian;
        this.chakanquanbu = false;
        this.dangqianshitu = 'ribao';
        this.dangqianyeshu = 1;
        this.shuaxindangqianshitu();
    }
}
