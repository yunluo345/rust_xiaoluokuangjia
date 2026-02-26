// 日报管理 - 界面层
import * as gj from './jiemian_gongju.js';

function jiexishijian(canchuo) {
    const ms = Number(canchuo);
    if (!ms || isNaN(ms)) return canchuo || '';
    const d = new Date(ms);
    const pad = n => String(n).padStart(2, '0');
    return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`;
}

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
        this.renwushaixuan = null;
        this.renwuyeshu = 1;
        this.renwumeiyetiaoshu = 10;
        this.sousuobiaoqianid = null;
        this.sousuoleixing = null;
        this.sousuoguanjiancizhi = null;
        this.sousuoyonghuid = null;
        this._tupu_daohang_biaoqianid = null;
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
               <button class="aq-btn aq-btn-zhu" onclick="ribao_qiehuanshitu('tupu')">图谱</button>
               <button class="aq-btn aq-btn-lv" onclick="ribao_shuaxin()">刷新数据</button>`
            : `<button class="aq-btn ${this.dangqianshitu === 'ribao' ? 'aq-btn-lv' : 'aq-btn-zhu'}" onclick="ribao_qiehuanshitu('ribao')">我的日报</button>
               <button class="aq-btn ${this.dangqianshitu === 'quanburibao' ? 'aq-btn-lv' : 'aq-btn-zhu'}" onclick="ribao_qiehuanshitu('quanburibao')">全部日报</button>
               <button class="aq-btn aq-btn-zhu" onclick="ribao_qiehuanshitu('tupu')">图谱</button>
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
            ? ['ribao', 'biaoqian', 'leixing', 'renwu', 'tupu']
            : ['ribao', 'quanburibao', 'tupu'];
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
            'renwu': () => this.shuaxinrenwuliebiao(),
            'tupu': () => this.shuaxintupushitu()
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

        let html = '<div style="margin-bottom:16px">';
        html += '<div style="display:flex;gap:10px;align-items:center;flex-wrap:wrap">';
        html += '<button class="aq-btn aq-btn-lv" onclick="ribao_xinzengshitu()" style="height:36px">新增日报</button>';
        html += '<button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_ribao_piliangshanchu()" style="height:36px">批量删除</button>';
        html += '<button class="aq-btn aq-btn-xiao aq-btn-zhu" onclick="ribao_piliang_xinzengrenwu()" style="height:36px">批量添加任务</button>';
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
        const guanlianjieguo = await Promise.all(
            liebiao.map(rb => this.luoji.guanlian_chaxun_ribaoid_daixinxi(rb.id))
        );
        for (let i = 0; i < liebiao.length; i++) {
            const gljg = guanlianjieguo[i];
            ribaobiaoqianmap[liebiao[i].id] = gljg?.zhuangtaima === 200 ? gljg.shuju || [] : [];
        }

        html = '<div style="margin-bottom:16px">';
        html += '<div style="display:flex;gap:10px;align-items:center;flex-wrap:wrap">';
        html += '<button class="aq-btn aq-btn-lv" onclick="ribao_xinzengshitu()" style="height:36px">新增日报</button>';
        html += '<button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_ribao_piliangshanchu()" style="height:36px">批量删除</button>';
        html += '<button class="aq-btn aq-btn-xiao aq-btn-zhu" onclick="ribao_piliang_xinzengrenwu()" style="height:36px">批量添加任务</button>';
        html += '<label style="display:flex;align-items:center;gap:4px;cursor:pointer;font-size:13px;color:#64748B;height:36px"><input type="checkbox" onchange="ribao_ribao_quanxuan(this)" style="width:16px;height:16px;cursor:pointer">全选</label>';
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
            if (rb.zhaiyao) {
                neironghtml += `<div style="margin-top:8px;padding:8px 12px;background:linear-gradient(135deg,#F0FDF4,#ECFDF5);border:1px solid #BBF7D0;border-radius:6px;font-size:13px;color:#15803D;line-height:1.5"><span style="font-weight:600">摘要：</span>${rb.zhaiyao}</div>`;
            }

            html += `<div style="background:#FFFFFF;border:1px solid #E2E8F0;border-radius:8px;padding:14px;display:flex;gap:10px">
                <input type="checkbox" class="rb_pl_xz" data-id="${rb.id}" style="width:18px;height:18px;cursor:pointer;accent-color:#3B82F6;margin-top:2px;flex-shrink:0">
                <div style="flex:1">
                <div style="display:flex;justify-content:space-between;align-items:start;margin-bottom:10px">
                    <div style="flex:1">
                        <div style="font-size:12px;color:#64748B;margin-bottom:4px">ID: ${rb.id} | 发布者: ${rb.fabuzhemingcheng || rb.fabuzhezhanghao || rb.yonghuid}${rb.fabuzhezhanghao ? '（' + rb.fabuzhezhanghao + '）' : ''} | ${jiexishijian(rb.fabushijian)}</div>
                        ${neironghtml}
                    </div>
                    <div style="display:flex;gap:6px;margin-left:12px">
                        <button class="aq-btn aq-btn-xiao aq-btn-huang" onclick="ribao_bianji('${rb.id}')">编辑</button>
                        <button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_shanchu('${rb.id}')">删除</button>
                        <button class="aq-btn aq-btn-xiao" onclick="ribao_guanlianguanlian('${rb.id}')">管理标签</button>
                        ${this._cunzai_kuozhan_daotu(rb) ? `<button class="aq-btn aq-btn-xiao aq-btn-zhu" onclick="ribao_siweidaotu('${rb.id}')">导图</button>` : ''}
                        ${this._cunzai_kuozhan_guanxi(rb) ? `<button class="aq-btn aq-btn-xiao" onclick="ribao_guanxifenxi('${rb.id}')" style="background:#F5F3FF;color:#7C3AED">关系</button>` : ''}
                        ${biaoqianlie.length > 0 ? `<button class="aq-btn aq-btn-xiao" onclick="ribao_tiaozhuan_tupu('${rb.id}')" style="background:#F0F9FF;color:#0369A1">图谱</button>` : ''}
                    </div>
                </div>`;

            if (biaoqianlie.length > 0) {
                html += '<div style="display:flex;flex-wrap:wrap;gap:6px;margin-top:8px">';
                for (const bq of biaoqianlie) {
                    const leixing = bq.leixingmingcheng || '未知';
                    const zhi = bq.zhi || '';
                    html += `<span style="display:inline-flex;align-items:center;gap:4px;padding:4px 10px;background:#EFF6FF;color:#1E40AF;border-radius:16px;font-size:12px;transition:background 200ms" onmouseover="this.style.background='#DBEAFE'" onmouseout="this.style.background='#EFF6FF'">
                        <span onclick="ribao_dianjibibaoqian('${leixing}','${zhi}')" style="cursor:pointer;display:inline-flex;align-items:center;gap:4px"><span style="color:#64748B">${leixing}:</span>${zhi}</span>
                        <span onclick="ribao_biaoqian_tiaozhuan_tupu('${bq.biaoqianid}')" title="在图谱中查看" style="cursor:pointer;color:#0369A1;font-size:13px;line-height:1;opacity:0.4;transition:opacity 150ms" onmouseover="this.style.opacity='1'" onmouseout="this.style.opacity='0.4'">◎</span>
                    </span>`;
                }
                html += '</div>';
            } else {
                html += '<div style="font-size:12px;color:#94A3B8;margin-top:8px">暂无标签</div>';
            }

            html += '</div></div>';
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
        let html = '<div style="margin-bottom:12px;display:flex;gap:8px"><button class="aq-btn aq-btn-lv" onclick="ribao_xinzengbiaoqian()">新增标签</button><button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_biaoqian_piliangshanchu()">批量删除</button></div>';
        if (liebiao.length === 0) {
            nr.innerHTML = html + '<p style="color:#64748B">暂无标签数据</p>';
            return;
        }
        html += '<div style="overflow-x:auto"><table class="aq-biao"><thead><tr>' +
            '<th><input type="checkbox" onchange="ribao_biaoqian_quanxuan(this)" style="width:16px;height:16px;cursor:pointer"></th><th>ID</th><th>类型ID</th><th>值</th><th>操作</th>' +
            '</tr></thead><tbody>';
        for (const bq of liebiao) {
            html += `<tr>
                <td><input type="checkbox" class="bq_pl_xz" data-id="${bq.id}" style="width:16px;height:16px;cursor:pointer"></td><td>${bq.id}</td><td>${bq.leixingid}</td><td>${bq.zhi}</td>
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
        let html = '<div style="margin-bottom:12px;display:flex;gap:8px"><button class="aq-btn aq-btn-lv" onclick="ribao_xinzengleixing()">新增类型</button><button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_leixing_piliangshanchu()">批量删除</button></div>';
        if (liebiao.length === 0) {
            nr.innerHTML = html + '<p style="color:#64748B">暂无类型数据</p>';
            return;
        }
        html += '<div style="overflow-x:auto"><table class="aq-biao"><thead><tr>' +
            '<th><input type="checkbox" onchange="ribao_leixing_quanxuan(this)" style="width:16px;height:16px;cursor:pointer"></th><th>ID</th><th>名称</th><th>操作</th>' +
            '</tr></thead><tbody>';
        for (const lx of liebiao) {
            html += `<tr>
                <td><input type="checkbox" class="lx_pl_xz" data-id="${lx.id}" style="width:16px;height:16px;cursor:pointer"></td><td>${lx.id}</td><td>${lx.mingcheng}</td>
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
            <div class="aq-hang"><label>发布时间</label><input id="rb_fabushijian" type="datetime-local" value="${jiexishijian(rb.fabushijian).replace(' ', 'T')}"></div>
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

    _jiexi_kuozhan(rb) {
        if (!rb || !rb.kuozhan) return null;
        try {
            const shuju = typeof rb.kuozhan === 'string' ? JSON.parse(rb.kuozhan) : rb.kuozhan;
            return shuju;
        } catch (_e) {
            return null;
        }
    }

    _huoqu_siweidaotu(kuozhan) {
        if (!kuozhan) return null;
        return kuozhan.siweidaotu || (kuozhan.mingcheng ? kuozhan : null);
    }

    _huoqu_guanxifenxi(kuozhan) {
        return kuozhan?.guanxifenxi || null;
    }

    _cunzai_kuozhan_daotu(rb) {
        const kz = this._jiexi_kuozhan(rb);
        return !!this._huoqu_siweidaotu(kz);
    }

    _cunzai_kuozhan_guanxi(rb) {
        const kz = this._jiexi_kuozhan(rb);
        const gx = this._huoqu_guanxifenxi(kz);
        return gx && gx.guanxi && gx.guanxi.length > 0;
    }

    siweidaotu(id) {
        const rb = this._ribaoshujuhuancun?.[id];
        const kuozhan = this._jiexi_kuozhan(rb);
        const shuju = this._huoqu_siweidaotu(kuozhan);
        if (!shuju) return this.luoji.rizhi('无思维导图数据', 'warn');
        this._swdt_dangqianshuju = shuju;
        const shuhtml = this._swdt_jiedian(shuju, 0, 0);
        const zhezhao = document.createElement('div');
        zhezhao.id = 'swdt_motai';
        zhezhao.style.cssText = 'position:fixed;inset:0;background:rgba(15,23,42,0.55);backdrop-filter:blur(6px);z-index:100;display:flex;align-items:center;justify-content:center;animation:tc-danru 150ms ease-out';
        zhezhao.innerHTML = `<div style="background:#fff;border-radius:16px;box-shadow:0 25px 60px rgba(0,0,0,0.2);width:92vw;max-width:1400px;max-height:88vh;display:flex;flex-direction:column;animation:tc-tanchu 200ms ease-out;overflow:hidden">
            <div style="display:flex;align-items:center;justify-content:space-between;padding:16px 24px;border-bottom:1px solid #E2E8F0;flex-shrink:0;background:linear-gradient(135deg,#F8FAFC,#EFF6FF);border-radius:16px 16px 0 0">
                <div style="display:flex;align-items:center;gap:10px">
                    <div style="width:36px;height:36px;border-radius:10px;background:#fff;display:flex;align-items:center;justify-content:center;font-size:14px;font-weight:700;color:#0F172A">AI</div>
                    <div>
                        <div style="font-size:15px;font-weight:700;color:#0F172A">思维导图分析</div>
                        <div style="font-size:12px;color:#94A3B8;margin-top:1px">AI 自动生成的日报深度分析</div>
                    </div>
                </div>
                <div style="display:flex;gap:8px">
                    <button class="aq-btn aq-btn-xiao aq-btn-lv" onclick="ribao_xiazaisiweidaotu()" style="margin:0">下载PNG</button>
                    <button class="aq-btn aq-btn-xiao" onclick="ribao_guanbisiweidaotu()" style="margin:0">关闭</button>
                </div>
            </div>
            <div style="overflow:auto;padding:32px;flex:1;background:linear-gradient(180deg,#FAFBFC 0%,#F1F5F9 100%)">
                <div style="display:inline-flex;min-width:fit-content">${shuhtml}</div>
            </div>
        </div>`;
        zhezhao.addEventListener('click', e => { if (e.target === zhezhao) this.guanbisiweidaotu(); });
        document.body.appendChild(zhezhao);
    }

    guanbisiweidaotu() {
        const el = document.getElementById('swdt_motai');
        if (el) el.remove();
        this._swdt_dangqianshuju = null;
    }

    guanxifenxi(id) {
        const rb = this._ribaoshujuhuancun?.[id];
        const kuozhan = this._jiexi_kuozhan(rb);
        const fenxi = this._huoqu_guanxifenxi(kuozhan);
        if (!fenxi || !fenxi.guanxi || fenxi.guanxi.length === 0) return this.luoji.rizhi('无关系分析数据', 'warn');
        const guanxilie = fenxi.guanxi;
        const zhuti = this._swdt_zhuti();
        const guanxileixing = { '同事': 0, '上下级': 1, '客户': 2, '合作伙伴': 3, '同学': 4, '相关': 5 };
        let liehtml = '';
        for (const gx of guanxilie) {
            const xu = guanxileixing[gx.guanxi] ?? 5;
            const t = zhuti[xu % zhuti.length];
            liehtml += `<div style="display:flex;align-items:center;gap:12px;padding:12px 16px;background:#fff;border:1px solid #E2E8F0;border-radius:10px">
                <div style="display:flex;align-items:center;gap:8px;flex:1">
                    <span style="padding:4px 10px;background:${t.qian};color:${t.zhu};border-radius:16px;font-size:12px;font-weight:600;white-space:nowrap">${gx.ren1 || ''}</span>
                    <span style="color:#94A3B8;font-size:18px">—</span>
                    <span style="padding:3px 8px;background:#F1F5F9;color:#475569;border-radius:6px;font-size:11px;white-space:nowrap">${gx.guanxi || '相关'}</span>
                    <span style="color:#94A3B8;font-size:18px">—</span>
                    <span style="padding:4px 10px;background:${t.qian};color:${t.zhu};border-radius:16px;font-size:12px;font-weight:600;white-space:nowrap">${gx.ren2 || ''}</span>
                </div>
                ${gx.miaoshu ? `<span style="font-size:12px;color:#64748B;flex-shrink:0;max-width:300px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${gx.miaoshu}</span>` : ''}
            </div>`;
        }
        const zhezhao = document.createElement('div');
        zhezhao.id = 'gxfx_motai';
        zhezhao.style.cssText = 'position:fixed;inset:0;background:rgba(15,23,42,0.55);backdrop-filter:blur(6px);z-index:100;display:flex;align-items:center;justify-content:center';
        zhezhao.innerHTML = `<div style="background:#fff;border-radius:16px;box-shadow:0 25px 60px rgba(0,0,0,0.2);width:680px;max-width:90vw;max-height:80vh;display:flex;flex-direction:column;overflow:hidden">
            <div style="display:flex;align-items:center;justify-content:space-between;padding:16px 24px;border-bottom:1px solid #E2E8F0;background:linear-gradient(135deg,#F5F3FF,#EFF6FF);border-radius:16px 16px 0 0">
                <div style="display:flex;align-items:center;gap:10px">
                    <div style="width:36px;height:36px;border-radius:10px;background:#fff;display:flex;align-items:center;justify-content:center;font-size:14px;font-weight:700;color:#7C3AED">AI</div>
                    <div>
                        <div style="font-size:15px;font-weight:700;color:#0F172A">人物关系分析</div>
                        <div style="font-size:12px;color:#94A3B8;margin-top:1px">AI 从日报中提取的人物关系 (${guanxilie.length} 组)</div>
                    </div>
                </div>
                <button class="aq-btn aq-btn-xiao" onclick="document.getElementById('gxfx_motai')?.remove()" style="margin:0">关闭</button>
            </div>
            <div style="overflow:auto;padding:20px;flex:1;display:flex;flex-direction:column;gap:10px;background:#FAFBFC">${liehtml}</div>
        </div>`;
        zhezhao.addEventListener('click', e => { if (e.target === zhezhao) zhezhao.remove(); });
        document.body.appendChild(zhezhao);
    }

    _swdt_zhuti() {
        return [
            { zhu: '#3B82F6', qian: '#EFF6FF', bian: '#BFDBFE' },
            { zhu: '#10B981', qian: '#ECFDF5', bian: '#A7F3D0' },
            { zhu: '#8B5CF6', qian: '#F5F3FF', bian: '#DDD6FE' },
            { zhu: '#F59E0B', qian: '#FFFBEB', bian: '#FDE68A' },
            { zhu: '#EC4899', qian: '#FDF2F8', bian: '#FBCFE8' },
            { zhu: '#06B6D4', qian: '#ECFEFF', bian: '#A5F3FC' },
        ];
    }

    _swdt_jiedian(jd, cj, sy) {
        const zhuti = this._swdt_zhuti();
        const youzi = jd.zijiedian && jd.zijiedian.length > 0;
        let html = '<div style="display:flex;align-items:center">';

        if (cj === 0) {
            html += '<div style="padding:16px 28px;border-radius:16px;background:#fff;color:#0F172A;border:1px solid #E2E8F0;box-shadow:0 4px 16px rgba(0,0,0,0.06);flex-shrink:0;transition:box-shadow 200ms">';
            html += `<div style="font-weight:700;font-size:17px;letter-spacing:0.3px">${jd.mingcheng || '日报分析'}</div>`;
            html += '</div>';
        } else if (cj === 1) {
            const t = zhuti[sy % zhuti.length];
            html += `<div style="padding:10px 16px;border-radius:10px;background:#fff;border-left:3px solid ${t.zhu};box-shadow:0 2px 8px rgba(0,0,0,0.04);flex-shrink:0;min-width:110px;transition:box-shadow 200ms,transform 200ms" onmouseover="this.style.boxShadow='0 4px 14px rgba(0,0,0,0.08)';this.style.transform='translateY(-1px)'" onmouseout="this.style.boxShadow='0 2px 8px rgba(0,0,0,0.04)';this.style.transform='none'">`;
            html += `<div style="font-weight:600;font-size:14px;color:${t.zhu}">${jd.mingcheng || ''}</div>`;
            if (jd.neirong) {
                html += `<div style="font-size:12px;color:#64748B;margin-top:4px;line-height:1.5;white-space:pre-wrap;word-break:break-all;max-width:200px">${jd.neirong}</div>`;
            }
            html += '</div>';
        } else {
            const t = zhuti[sy % zhuti.length];
            html += `<div style="padding:7px 12px;border-radius:8px;background:${t.qian};border:1px solid ${t.bian};flex-shrink:0;max-width:280px;transition:box-shadow 200ms,transform 200ms" onmouseover="this.style.boxShadow='0 3px 10px rgba(0,0,0,0.06)';this.style.transform='translateY(-1px)'" onmouseout="this.style.boxShadow='none';this.style.transform='none'">`;
            html += `<div style="font-weight:600;font-size:13px;color:#334155">${jd.mingcheng || ''}</div>`;
            if (jd.neirong) {
                html += `<div style="font-size:12px;color:#64748B;margin-top:2px;line-height:1.5;white-space:pre-wrap;word-break:break-all">${jd.neirong}</div>`;
            }
            html += '</div>';
        }

        if (youzi) {
            const t = zhuti[sy % zhuti.length];
            const xianse = cj === 0 ? '#CBD5E1' : t.bian;
            const zhuse = cj === 0 ? '#94A3B8' : t.zhu;
            html += `<svg width="36" height="2" style="flex-shrink:0;overflow:visible"><line x1="0" y1="1" x2="36" y2="1" stroke="${xianse}" stroke-width="1.5" stroke-linecap="round"/></svg>`;
            html += '<div style="display:flex;flex-direction:column;position:relative;padding:4px 0">';
            html += `<div style="position:absolute;left:0;top:4px;bottom:4px;width:1.5px;background:linear-gradient(180deg,${xianse}00,${xianse},${xianse},${xianse}00);border-radius:1px"></div>`;
            for (let i = 0; i < jd.zijiedian.length; i++) {
                const ziSy = cj === 0 ? i : sy;
                const ziXianse = cj === 0 ? zhuti[i % zhuti.length].bian : xianse;
                const ziZhuse = cj === 0 ? zhuti[i % zhuti.length].zhu : zhuse;
                html += '<div style="display:flex;align-items:center;margin:3px 0;position:relative">';
                html += `<svg width="28" height="12" style="flex-shrink:0;overflow:visible"><circle cx="1" cy="6" r="2.5" fill="${ziZhuse}" opacity="0.5"/><line x1="5" y1="6" x2="28" y2="6" stroke="${ziXianse}" stroke-width="1.5" stroke-linecap="round"/></svg>`;
                html += this._swdt_jiedian(jd.zijiedian[i], cj + 1, ziSy);
                html += '</div>';
            }
            html += '</div>';
        }

        html += '</div>';
        return html;
    }

    xiazaisiweidaotu() {
        if (!this._swdt_dangqianshuju) return;
        const zhuti = this._swdt_zhuti();
        const H_GAP = 56, V_GAP = 14, PAD = 50;

        const huanhang = (ctx, wen, maxW) => {
            const hangs = [];
            let dang = '';
            for (const zi of Array.from(wen)) {
                if (zi === '\n') { hangs.push(dang); dang = ''; continue; }
                const ce = dang + zi;
                if (ctx.measureText(ce).width > maxW && dang) { hangs.push(dang); dang = zi; }
                else dang = ce;
            }
            if (dang) hangs.push(dang);
            return hangs;
        };

        const celiang = (ctx, jd, cj, sy) => {
            const mcZiti = cj === 0 ? 'bold 16px sans-serif' : cj === 1 ? 'bold 14px sans-serif' : 'bold 13px sans-serif';
            const px = cj === 0 ? 24 : cj === 1 ? 16 : 12;
            const py = cj === 0 ? 14 : cj === 1 ? 10 : 7;
            const mcGao = cj === 0 ? 20 : cj === 1 ? 18 : 16;
            const maxNrW = 200;
            ctx.font = mcZiti;
            const mc = jd.mingcheng || '';
            const mcW = ctx.measureText(mc).width;
            let nrHangs = [], nrH = 0;
            if (jd.neirong) {
                ctx.font = '12px sans-serif';
                nrHangs = huanhang(ctx, jd.neirong, maxNrW);
                nrH = nrHangs.length * 17 + 4;
            }
            const w = Math.max(mcW, nrHangs.length > 0 ? maxNrW : 0) + px * 2;
            const h = py * 2 + mcGao + nrH;
            let zishugao = 0;
            const ziJd = [];
            if (jd.zijiedian && jd.zijiedian.length > 0) {
                for (let i = 0; i < jd.zijiedian.length; i++) {
                    const zi = celiang(ctx, jd.zijiedian[i], cj + 1, cj === 0 ? i : sy);
                    ziJd.push(zi);
                    zishugao += zi.shugao + (i > 0 ? V_GAP : 0);
                }
            }
            return { mc, nr: jd.neirong || '', nrHangs, cj, sy, w, h, shugao: Math.max(h, zishugao), zijiedian: ziJd, x: 0, y: 0 };
        };

        const buju = (jd, x, y) => {
            jd.x = x;
            jd.y = y + (jd.shugao - jd.h) / 2;
            if (jd.zijiedian.length > 0) {
                let ziY = y;
                for (const zi of jd.zijiedian) {
                    buju(zi, x + jd.w + H_GAP, ziY);
                    ziY += zi.shugao + V_GAP;
                }
            }
        };

        const bianjie = (jd) => {
            let mx = jd.x + jd.w, my = jd.y + jd.h;
            for (const zi of jd.zijiedian) {
                const r = bianjie(zi);
                mx = Math.max(mx, r.mx);
                my = Math.max(my, r.my);
            }
            return { mx, my };
        };

        const yuanjiao = (ctx, x, y, w, h, r) => {
            ctx.beginPath();
            ctx.moveTo(x + r, y);
            ctx.lineTo(x + w - r, y);
            ctx.quadraticCurveTo(x + w, y, x + w, y + r);
            ctx.lineTo(x + w, y + h - r);
            ctx.quadraticCurveTo(x + w, y + h, x + w - r, y + h);
            ctx.lineTo(x + r, y + h);
            ctx.quadraticCurveTo(x, y + h, x, y + h - r);
            ctx.lineTo(x, y + r);
            ctx.quadraticCurveTo(x, y, x + r, y);
            ctx.closePath();
        };

        const huajiedian = (ctx, jd) => {
            const { cj, sy, x, y, w, h } = jd;
            const t = zhuti[sy % zhuti.length];

            if (jd.zijiedian.length > 0) {
                for (const zi of jd.zijiedian) {
                    const fx = x + w, fy = y + h / 2, tx = zi.x, ty = zi.y + zi.h / 2;
                    const cpx = (fx + tx) / 2;
                    ctx.strokeStyle = cj === 0 ? '#D1D5DB' : (zhuti[zi.sy % zhuti.length].bian || '#D1D5DB');
                    ctx.lineWidth = 2;
                    ctx.lineCap = 'round';
                    ctx.beginPath();
                    ctx.moveTo(fx, fy);
                    ctx.bezierCurveTo(cpx, fy, cpx, ty, tx, ty);
                    ctx.stroke();
                }
            }

            if (cj === 0) {
                ctx.save();
                ctx.shadowColor = 'rgba(0,0,0,0.08)';
                ctx.shadowBlur = 16;
                ctx.shadowOffsetY = 4;
                yuanjiao(ctx, x, y, w, h, 14);
                ctx.fillStyle = '#fff';
                ctx.fill();
                ctx.restore();
                yuanjiao(ctx, x, y, w, h, 14);
                ctx.strokeStyle = '#E2E8F0';
                ctx.lineWidth = 1;
                ctx.stroke();
                ctx.font = 'bold 16px sans-serif';
                ctx.fillStyle = '#0F172A';
                ctx.fillText(jd.mc, x + 24, y + 14 + 16);
            } else if (cj === 1) {
                ctx.save();
                ctx.shadowColor = 'rgba(0,0,0,0.06)';
                ctx.shadowBlur = 8;
                ctx.shadowOffsetY = 2;
                yuanjiao(ctx, x, y, w, h, 10);
                ctx.fillStyle = '#fff';
                ctx.fill();
                ctx.restore();
                yuanjiao(ctx, x, y, w, h, 10);
                ctx.strokeStyle = '#E2E8F0';
                ctx.lineWidth = 1;
                ctx.stroke();
                ctx.beginPath();
                ctx.moveTo(x, y + 10);
                ctx.lineTo(x, y + h - 10);
                ctx.strokeStyle = t.zhu;
                ctx.lineWidth = 4;
                ctx.lineCap = 'round';
                ctx.stroke();
                ctx.lineCap = 'butt';
                ctx.font = 'bold 14px sans-serif';
                ctx.fillStyle = '#1E293B';
                ctx.fillText(jd.mc, x + 16, y + 10 + 14);
                if (jd.nrHangs.length > 0) {
                    ctx.font = '12px sans-serif';
                    ctx.fillStyle = '#64748B';
                    let ny = y + 10 + 18 + 4;
                    for (const hang of jd.nrHangs) { ctx.fillText(hang, x + 16, ny + 12); ny += 17; }
                }
            } else {
                yuanjiao(ctx, x, y, w, h, 8);
                ctx.fillStyle = t.qian;
                ctx.fill();
                yuanjiao(ctx, x, y, w, h, 8);
                ctx.strokeStyle = t.bian;
                ctx.lineWidth = 1;
                ctx.stroke();
                ctx.font = 'bold 13px sans-serif';
                ctx.fillStyle = '#334155';
                ctx.fillText(jd.mc, x + 12, y + 7 + 13);
                if (jd.nrHangs.length > 0) {
                    ctx.font = '12px sans-serif';
                    ctx.fillStyle = '#64748B';
                    let ny = y + 7 + 16 + 2;
                    for (const hang of jd.nrHangs) { ctx.fillText(hang, x + 12, ny + 12); ny += 17; }
                }
            }

            for (const zi of jd.zijiedian) huajiedian(ctx, zi);
        };

        const tmpCanvas = document.createElement('canvas');
        const tmpCtx = tmpCanvas.getContext('2d');
        const gen = celiang(tmpCtx, this._swdt_dangqianshuju, 0, 0);
        buju(gen, PAD, PAD);
        const { mx, my } = bianjie(gen);

        const scale = 2;
        const canvas = document.createElement('canvas');
        canvas.width = (mx + PAD) * scale;
        canvas.height = (my + PAD) * scale;
        const ctx = canvas.getContext('2d');
        ctx.scale(scale, scale);
        ctx.fillStyle = '#FFFFFF';
        ctx.fillRect(0, 0, mx + PAD, my + PAD);
        huajiedian(ctx, gen);

        const link = document.createElement('a');
        link.download = '思维导图_' + new Date().toISOString().slice(0, 10) + '.png';
        link.href = canvas.toDataURL('image/png');
        link.click();
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

    async tiaozhuan_tupu(ribaoid) {
        const gljg = await this.luoji.guanlian_chaxun_ribaoid_daixinxi(ribaoid);
        const biaoqianlie = gljg?.zhuangtaima === 200 ? gljg.shuju || [] : [];
        if (biaoqianlie.length === 0) {
            this.luoji.rizhi('该日报暂无标签，无法跳转图谱', 'warn');
            return;
        }
        this._tupu_daohang_biaoqianid = biaoqianlie[0].biaoqianid;
        await this.qiehuanshitu('tupu');
    }

    async biaoqian_tiaozhuan_tupu(biaoqianid) {
        this._tupu_daohang_biaoqianid = biaoqianid;
        await this.qiehuanshitu('tupu');
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
        const [liebiaoqj, yichuli, kechuli, chulizhong, shibaishu_jg, zhuangtaijg] = await Promise.all([
            this.luoji.renwu_chaxun_fenye(this.renwushaixuan, this.renwuyeshu, this.renwumeiyetiaoshu),
            this.luoji.renwu_tongji_zhuangtai('true'),
            this.luoji.renwu_tongji_kechuli(),
            this.luoji.renwu_tongji_zhuangtai('processing'),
            this.luoji.renwu_tongji_zhuangtai('shibai'),
            this.luoji.renwu_biaoqian_ai_zhuangtai()
        ]);
        const liebiao = liebiaoqj?.zhuangtaima === 200 ? liebiaoqj.shuju?.liebiao || [] : [];
        const zongshu = liebiaoqj?.zhuangtaima === 200 ? liebiaoqj.shuju?.zongshu || 0 : 0;
        const yichulishu = yichuli?.zhuangtaima === 200 ? yichuli.shuju?.count ?? 0 : 0;
        const kechulishu = kechuli?.zhuangtaima === 200 ? kechuli.shuju?.count ?? 0 : 0;
        const chulizhongshu = chulizhong?.zhuangtaima === 200 ? chulizhong.shuju?.count ?? 0 : 0;
        const shibaishu = shibaishu_jg?.zhuangtaima === 200 ? shibaishu_jg.shuju?.count ?? 0 : 0;
        const yunxingzhong = zhuangtaijg?.zhuangtaima === 200 && zhuangtaijg.shuju?.yunxingzhong === true;
        const sx = this.renwushaixuan;
        let html = `<div style="display:flex;gap:16px;margin-bottom:16px">
            <div style="background:#F0FDF4;border:1px solid #BBF7D0;border-radius:8px;padding:12px 20px;flex:1;text-align:center">
                <div style="font-size:24px;font-weight:600;color:#16A34A">${kechulishu}</div>
                <div style="font-size:12px;color:#4ADE80">待处理</div>
            </div>
            <div style="background:#FFF7ED;border:1px solid #FED7AA;border-radius:8px;padding:12px 20px;flex:1;text-align:center">
                <div style="font-size:24px;font-weight:600;color:#EA580C">${chulizhongshu}</div>
                <div style="font-size:12px;color:#FB923C">处理中</div>
            </div>
            <div style="background:#EFF6FF;border:1px solid #BFDBFE;border-radius:8px;padding:12px 20px;flex:1;text-align:center">
                <div style="font-size:24px;font-weight:600;color:#2563EB">${yichulishu}</div>
                <div style="font-size:12px;color:#60A5FA">已完成</div>
            </div>
            <div style="background:#FEF2F2;border:1px solid #FECACA;border-radius:8px;padding:12px 20px;flex:1;text-align:center">
                <div style="font-size:24px;font-weight:600;color:#DC2626">${shibaishu}</div>
                <div style="font-size:12px;color:#F87171">失败</div>
            </div>
            <div style="background:${yunxingzhong ? '#ECFDF5' : '#F8FAFC'};border:1px solid ${yunxingzhong ? '#A7F3D0' : '#E2E8F0'};border-radius:8px;padding:12px 20px;flex:1;text-align:center">
                <div style="font-size:24px;font-weight:600;color:${yunxingzhong ? '#059669' : '#94A3B8'}">${yunxingzhong ? '●' : '○'}</div>
                <div style="font-size:12px;color:${yunxingzhong ? '#10B981' : '#94A3B8'}">${yunxingzhong ? '运行中' : '已停止'}</div>
            </div>
        </div>`;
        html += '<div style="margin-bottom:16px;display:flex;gap:8px;align-items:center">';
        html += `<button class="aq-btn ${sx === null ? 'aq-btn-lv' : 'aq-btn-zhu'}" onclick="ribao_renwu_shaixuan(null)" style="height:36px">全部</button>`;
        html += `<button class="aq-btn ${sx === 'false' ? 'aq-btn-lv' : 'aq-btn-zhu'}" onclick="ribao_renwu_shaixuan('false')" style="height:36px">待处理</button>`;
        html += `<button class="aq-btn ${sx === 'processing' ? 'aq-btn-lv' : 'aq-btn-zhu'}" onclick="ribao_renwu_shaixuan('processing')" style="height:36px">处理中</button>`;
        html += `<button class="aq-btn ${sx === 'true' ? 'aq-btn-lv' : 'aq-btn-zhu'}" onclick="ribao_renwu_shaixuan('true')" style="height:36px">已完成</button>`;
        html += `<button class="aq-btn ${sx === 'shibai' ? 'aq-btn-lv' : 'aq-btn-zhu'}" onclick="ribao_renwu_shaixuan('shibai')" style="height:36px">失败</button>`;
        html += '<div style="height:20px;width:1px;background:#E2E8F0"></div>';
        html += '<input id="rw_ribaoid" type="text" placeholder="日报ID" style="height:36px;padding:0 12px;border:1px solid #E2E8F0;border-radius:6px;width:140px;font-size:13px;box-sizing:border-box">';
        html += '<button class="aq-btn aq-btn-lv" onclick="ribao_xinzengrenwu()" style="height:36px">新增任务</button>';
        html += '<button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_renwu_piliangshanchu()" style="height:36px">批量删除</button>';
        html += '<div style="height:20px;width:1px;background:#E2E8F0"></div>';
        html += yunxingzhong
            ? '<button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_renwu_tingzhi()" style="height:36px">停止AI处理</button>'
            : '<button class="aq-btn aq-btn-zhu" onclick="ribao_renwu_qidong()" style="height:36px">启动AI处理</button>';
        html += '</div>';
        const kongwenzimap = { 'true': '暂无已完成任务', 'false': '暂无待处理任务', 'processing': '暂无处理中任务', 'shibai': '暂无失败任务' };
        if (liebiao.length === 0) {
            nr.innerHTML = html + `<p style="color:#64748B">${kongwenzimap[sx] || '暂无任务'}</p>`;
            return;
        }
        const zhuangtaimap = { 'true': ['已完成', '#16A34A'], 'processing': ['处理中', '#EA580C'], 'false': ['待处理', '#F59E0B'], 'shibai': ['失败', '#DC2626'] };
        html += '<div style="overflow-x:auto"><table class="aq-biao"><thead><tr>' +
            '<th><input type="checkbox" onchange="ribao_renwu_quanxuan(this)" style="width:16px;height:16px;cursor:pointer"></th><th>任务ID</th><th>日报ID</th><th>用户ID</th><th>状态</th><th>尝试</th><th>标签结果</th><th>创建时间</th><th>操作</th>' +
            '</tr></thead><tbody>';
        for (const rw of liebiao) {
            const [zhuangtai, zhuangtaiyanse] = zhuangtaimap[rw.zhuangtai] || ['未知', '#94A3B8'];
            const jieguo = rw.biaoqianjieguo ? (rw.biaoqianjieguo.length > 30 ? rw.biaoqianjieguo.substring(0, 30) + '...' : rw.biaoqianjieguo) : '-';
            html += `<tr>
                <td><input type="checkbox" class="rw_pl_xz" data-id="${rw.id}" style="width:16px;height:16px;cursor:pointer"></td><td>${rw.id}</td><td>${rw.ribaoid}</td><td>${rw.yonghuid}</td>
                <td><span style="color:${zhuangtaiyanse};font-weight:600">${zhuangtai}</span></td>
                <td>${rw.changshicishu}/${rw.zuidachangshicishu}</td>
                <td style="max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap" title="${(rw.biaoqianjieguo || '').replace(/"/g, '&quot;')}">${jieguo}</td>
                <td style="white-space:nowrap">${jiexishijian(rw.chuangjianshijian)}</td>
                <td style="white-space:nowrap">
                    <button class="aq-btn aq-btn-xiao aq-btn-huang" onclick="ribao_chongxinruidui('${rw.id}')">重新入队</button>
                    <button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_shanchurenwu('${rw.id}')">删除</button>
                </td>
            </tr>`;
        }
        html += '</tbody></table></div>';
        const zongyeshu = Math.max(1, Math.ceil(zongshu / this.renwumeiyetiaoshu));
        html += `<div style="margin-top:12px;display:flex;gap:8px;align-items:center">
            <button class="aq-btn aq-btn-xiao" onclick="ribao_renwu_shangyiye()" ${this.renwuyeshu <= 1 ? 'disabled' : ''}>上一页</button>
            <span style="color:#64748B">第 ${this.renwuyeshu} / ${zongyeshu} 页（共 ${zongshu} 条）</span>
            <button class="aq-btn aq-btn-xiao" onclick="ribao_renwu_xiayiye()" ${this.renwuyeshu >= zongyeshu ? 'disabled' : ''}>下一页</button>
        </div>`;
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

    renwu_shaixuan(zhi) {
        this.renwushaixuan = zhi;
        this.renwuyeshu = 1;
        this.shuaxinrenwuliebiao();
    }

    renwu_shangyiye() {
        if (this.renwuyeshu > 1) {
            this.renwuyeshu--;
            this.shuaxinrenwuliebiao();
        }
    }

    renwu_xiayiye() {
        this.renwuyeshu++;
        this.shuaxinrenwuliebiao();
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

    async shuaxintupushitu() {
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = '<p style="color:#64748B">加载图谱数据...</p>';
        const leixingjg = await this.luoji.leixing_chaxun_quanbu();
        const leixinglie = leixingjg?.zhuangtaima === 200 ? leixingjg.shuju || [] : [];
        let html = '<div style="display:flex;gap:10px;align-items:center;margin-bottom:12px;flex-wrap:wrap">';
        html += '<button class="aq-btn aq-btn-lv" onclick="ribao_tupu_jiazai(null)">全部标签</button>';
        for (const lx of leixinglie) {
            html += `<button class="aq-btn aq-btn-zhu" onclick="ribao_tupu_jiazai('${lx.mingcheng}')">${lx.mingcheng}</button>`;
        }
        html += '<div style="position:relative;margin-left:auto">';
        html += '<input id="tupu_sousuo_shuru" type="text" placeholder="搜索标签..." style="padding:6px 12px;border:1px solid #CBD5E1;border-radius:8px;font-size:13px;width:180px;outline:none" oninput="ribao_tupu_sousuo_shuru(this.value)">';
        html += '<div id="tupu_sousuo_jieguo" style="position:absolute;top:100%;left:0;right:0;background:white;border:1px solid #E2E8F0;border-radius:8px;margin-top:4px;max-height:300px;overflow-y:auto;display:none;z-index:20;box-shadow:0 4px 12px rgba(0,0,0,0.1)"></div>';
        html += '</div>';
        html += '</div><div id="tupu_rongqi" style="position:relative;background:#FAFBFC;border:1px solid #E2E8F0;border-radius:12px;overflow:hidden"></div>';
        nr.innerHTML = html;
        const mubiao = this._tupu_daohang_biaoqianid;
        this._tupu_daohang_biaoqianid = null;
        if (mubiao) {
            await this._tupu_jiazai_biaoqianid(mubiao);
        } else {
            await this._tupu_jiazai(null);
        }
    }

    async _tupu_jiazai_impl(leixingmingcheng, biaoqianid) {
        const rongqi = document.getElementById('tupu_rongqi');
        if (!rongqi) return;
        rongqi.innerHTML = '<p style="color:#64748B;padding:20px">加载中...</p>';
        let jg;
        if (biaoqianid) {
            jg = await this.luoji.tupu_chaxun_biaoqianid(biaoqianid);
        } else if (leixingmingcheng) {
            jg = await this.luoji.tupu_chaxun_leixingmingcheng(leixingmingcheng);
        } else {
            jg = await this.luoji.tupu_chaxun_quanbu();
        }
        if (!jg || jg.zhuangtaima !== 200 || !jg.shuju) {
            rongqi.innerHTML = '<p style="color:#EF4444;padding:20px">图谱数据加载失败</p>';
            return;
        }
        const { jiedian: jiedianlie, bian: bianlie, guanxi_bian: guanxi_bianlie } = jg.shuju;
        if (!jiedianlie || jiedianlie.length === 0) {
            rongqi.innerHTML = '<p style="color:#94A3B8;padding:20px">暂无图谱数据</p>';
            return;
        }
        this._tupu_xuanran(rongqi, jiedianlie, bianlie || [], guanxi_bianlie || [], biaoqianid);
    }

    async _tupu_jiazai(leixingmingcheng) {
        return this._tupu_jiazai_impl(leixingmingcheng, null);
    }

    async _tupu_jiazai_biaoqianid(biaoqianid) {
        return this._tupu_jiazai_impl(null, biaoqianid);
    }

    _tupu_xuanran(rongqi, jiedianlie, bianlie, guanxi_bianlie, zhongxinid) {
        const zhuti = this._swdt_zhuti();
        let kuan = rongqi.clientWidth || 900;
        let gao = Math.max(700, window.innerHeight - 260);
        rongqi.innerHTML = '';
        rongqi.style.height = gao + 'px';
        const canvas = document.createElement('canvas');
        canvas.width = kuan;
        canvas.height = gao;
        canvas.style.cssText = `display:block;cursor:grab;width:${kuan}px;height:${gao}px`;
        rongqi.appendChild(canvas);
        const ctx = canvas.getContext('2d');

        let suofang = 1, pingyi_x = 0, pingyi_y = 0;
        let shijiezhongxin_x = kuan / 2, shijiezhongxin_y = gao / 2;

        const leixingmap = {};
        let leixingxuhao = 0;
        const kuosan = Math.max(220, Math.sqrt(jiedianlie.length) * 60);
        const jiedian = jiedianlie.map(j => {
            const id = String(j.id ?? '');
            const lx = j.leixingmingcheng || '';
            if (!(lx in leixingmap)) leixingmap[lx] = leixingxuhao++;
            return {
                id, zhi: j.zhi || '', leixing: lx,
                x: shijiezhongxin_x + (Math.random() - 0.5) * kuosan * 2,
                y: shijiezhongxin_y + (Math.random() - 0.5) * kuosan * 2,
                vx: 0, vy: 0,
                banjing: zhongxinid && String(zhongxinid) === id ? 24 : 18
            };
        });
        const idmap = Object.fromEntries(jiedian.map((j, i) => [j.id, i]));
        const bian = bianlie.filter(b => (String(b.yuan) in idmap) && (String(b.mubiao) in idmap)).map(b => ({
            yuan: idmap[String(b.yuan)],
            mubiao: idmap[String(b.mubiao)],
            quanzhong: parseInt(b.quanzhong) || 1
        }));

        // 关系边构建
        const guanxi_bian = guanxi_bianlie.filter(b => (String(b.yuan) in idmap) && (String(b.mubiao) in idmap)).map(b => ({
            yuan: idmap[String(b.yuan)],
            mubiao: idmap[String(b.mubiao)],
            guanxi: b.guanxi || '',
            miaoshu: b.miaoshu || '',
            cishu: parseInt(b.cishu) || 1
        }));

        // 节点度数 → 按连接数缩放半径
        const _du = new Array(jiedian.length).fill(0);
        for (const b of bian) { _du[b.yuan]++; _du[b.mubiao]++; }
        for (const b of guanxi_bian) { _du[b.yuan]++; _du[b.mubiao]++; }
        const _zuidade = Math.max(1, ..._du);
        for (let i = 0; i < jiedian.length; i++) {
            const jichu = zhongxinid && jiedian[i].id === String(zhongxinid) ? 24 : 18;
            jiedian[i].banjing = Math.round(jichu * (0.65 + 0.55 * _du[i] / _zuidade));
        }

        // --- 图谱工具栏 ---
        const _svg = {
            fangda: '<svg width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="8" y1="3.5" x2="8" y2="12.5"/><line x1="3.5" y1="8" x2="12.5" y2="8"/></svg>',
            suoxiao: '<svg width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="3.5" y1="8" x2="12.5" y2="8"/></svg>',
            chongzhi: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M2.5 8a5.5 5.5 0 1 1 1.1 3.3"/><polyline points="0.5 7.5 2.5 11.3 5.2 8.8"/></svg>',
            quanping: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><polyline points="2 6 2 2 6 2"/><polyline points="10 2 14 2 14 6"/><polyline points="14 10 14 14 10 14"/><polyline points="6 14 2 14 2 10"/></svg>',
            tuiquanping: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 2 6 6 2 6"/><polyline points="10 6 14 6 10 2"/><polyline points="10 14 10 10 14 10"/><polyline points="2 10 6 10 6 14"/></svg>',
            fanhui: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><line x1="13" y1="8" x2="3" y2="8"/><polyline points="7 4 3 8 7 12"/></svg>'
        };
        const _gj_anniu = (neirong, tishi, zidingyi) => {
            const btn = document.createElement('button');
            btn.innerHTML = neirong;
            if (tishi) btn.title = tishi;
            btn.style.cssText = 'width:32px;height:32px;padding:0;display:inline-flex;align-items:center;justify-content:center;border:none;background:transparent;color:#64748B;border-radius:8px;cursor:pointer;transition:all 150ms ease;margin:0;min-height:0;font-size:12px';
            if (zidingyi) Object.assign(btn.style, zidingyi);
            btn.onmouseenter = () => { btn.style.background = '#F1F5F9'; btn.style.color = '#0F172A'; };
            btn.onmouseleave = () => { btn.style.background = 'transparent'; btn.style.color = '#64748B'; };
            btn.onmousedown = () => { btn.style.transform = 'scale(0.9)'; };
            btn.onmouseup = () => { btn.style.transform = 'scale(1)'; };
            return btn;
        };
        const _gj_fengefu = () => {
            const d = document.createElement('div');
            d.style.cssText = 'width:1px;height:18px;background:rgba(148,163,184,0.2);margin:0 1px;flex-shrink:0';
            return d;
        };
        const gongju = document.createElement('div');
        gongju.style.cssText = 'position:absolute;top:12px;left:12px;display:flex;gap:2px;z-index:2;align-items:center;background:rgba(255,255,255,0.88);backdrop-filter:blur(20px);-webkit-backdrop-filter:blur(20px);border:1px solid rgba(226,232,240,0.45);border-radius:12px;padding:4px 5px;box-shadow:0 4px 24px rgba(0,0,0,0.06),0 1px 2px rgba(0,0,0,0.03)';
        if (zhongxinid) {
            const fanhui = _gj_anniu(_svg.fanhui + '<span style="margin-left:4px;font-weight:500">返回</span>', '返回全局视图');
            fanhui.style.width = 'auto';
            fanhui.style.padding = '0 10px';
            fanhui.onclick = () => this._tupu_jiazai(null);
            gongju.append(fanhui, _gj_fengefu());
        }
        const suoxiao = _gj_anniu(_svg.suoxiao, '缩小');
        suoxiao.onclick = () => { suofang = Math.max(0.2, suofang / 1.3); };
        const suofangxianshi = document.createElement('span');
        suofangxianshi.style.cssText = 'font-size:11px;color:#94A3B8;min-width:38px;text-align:center;font-weight:600;font-variant-numeric:tabular-nums;user-select:none;letter-spacing:-0.3px';
        suofangxianshi.textContent = '100%';
        const fangda = _gj_anniu(_svg.fangda, '放大');
        fangda.onclick = () => { suofang = Math.min(5, suofang * 1.3); };
        const chongzhi = _gj_anniu(_svg.chongzhi, '重置视图');
        chongzhi.onclick = () => { suofang = 1; pingyi_x = 0; pingyi_y = 0; };
        const quanping = _gj_anniu(_svg.quanping, '全屏');
        quanping.onclick = () => {
            if (document.fullscreenElement === rongqi) {
                document.exitFullscreen();
            } else {
                rongqi.requestFullscreen().catch(() => {});
            }
        };
        gongju.append(suoxiao, suofangxianshi, fangda, _gj_fengefu(), chongzhi, _gj_fengefu(), quanping);
        rongqi.appendChild(gongju);

        let _qianKuan = kuan, _qianGao = gao;
        const _tiaozheng_chicun = () => {
            const shifouquanping = document.fullscreenElement === rongqi;
            const xinKuan = shifouquanping ? window.innerWidth : (rongqi.clientWidth || 900);
            const xinGao = shifouquanping ? window.innerHeight : Math.max(700, window.innerHeight - 260);
            if (xinKuan === kuan && xinGao === gao) return;
            // 全屏时自动调整缩放比例，让节点大小自适应
            const bili = Math.min(xinKuan / kuan, xinGao / gao);
            suofang *= bili;
            pingyi_x *= bili;
            pingyi_y *= bili;
            _qianKuan = kuan; _qianGao = gao;
            kuan = xinKuan; gao = xinGao;
            canvas.width = kuan; canvas.height = gao;
            canvas.style.width = kuan + 'px';
            canvas.style.height = gao + 'px';
            shijiezhongxin_x = kuan / 2; shijiezhongxin_y = gao / 2;
            if (shifouquanping) {
                rongqi.style.height = '100%';
                rongqi.style.borderRadius = '0';
                rongqi.style.border = 'none';
            } else {
                rongqi.style.height = gao + 'px';
                rongqi.style.borderRadius = '12px';
                rongqi.style.border = '1px solid #E2E8F0';
            }
            quanping.innerHTML = shifouquanping ? _svg.tuiquanping : _svg.quanping;
            quanping.title = shifouquanping ? '退出全屏' : '全屏';
        };
        document.addEventListener('fullscreenchange', _tiaozheng_chicun);

        const tuli = document.createElement('div');
        tuli.style.cssText = 'position:absolute;top:12px;right:12px;background:rgba(255,255,255,0.7);backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px);border:1px solid rgba(226,232,240,0.6);border-radius:12px;padding:10px 14px;font-size:12px;display:flex;flex-wrap:wrap;gap:8px;z-index:2;max-width:260px;box-shadow:0 2px 12px rgba(0,0,0,0.04);color:#334155;font-weight:500';
        for (const [ming, xu] of Object.entries(leixingmap)) {
            const yanse = zhuti[xu % zhuti.length].zhu;
            tuli.innerHTML += `<span style="display:flex;align-items:center;gap:5px"><span style="width:8px;height:8px;border-radius:50%;background:${yanse};display:inline-block;flex-shrink:0;box-shadow:0 0 0 2px ${yanse}33"></span>${ming}</span>`;
        }
        // 边类型图例
        tuli.innerHTML += '<span style="display:flex;align-items:center;gap:5px;border-top:1px solid #E2E8F0;padding-top:6px;margin-top:2px;width:100%">' +
            '<span style="width:20px;border-top:2px solid #94A3B8;display:inline-block;flex-shrink:0"></span><span style="color:#64748B;font-size:11px">共现</span>' +
            '<span style="width:20px;border-top:2px dashed #8B5CF6;display:inline-block;flex-shrink:0;margin-left:8px"></span><span style="color:#8B5CF6;font-size:11px">AI关系</span>' +
            '</span>';
        rongqi.appendChild(tuli);

        const xinxi = document.createElement('div');
        xinxi.id = 'tupu_xinxi';
        xinxi.style.cssText = 'position:absolute;bottom:12px;left:12px;background:rgba(255,255,255,0.7);backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px);border:1px solid rgba(226,232,240,0.6);border-radius:12px;padding:10px 16px;font-size:13px;color:#1E293B;z-index:2;display:none;pointer-events:none;box-shadow:0 4px 16px rgba(0,0,0,0.06);font-weight:500;max-width:520px;line-height:1.6';
        rongqi.appendChild(xinxi);

        const celan = document.createElement('div');
        celan.id = 'tupu_celan';
        celan.style.cssText = 'position:absolute;right:0;top:0;width:360px;height:100%;background:rgba(255,255,255,0.85);backdrop-filter:blur(16px);-webkit-backdrop-filter:blur(16px);border-left:1px solid rgba(226,232,240,0.6);z-index:10;overflow-y:auto;display:none;box-shadow:-4px 0 20px rgba(0,0,0,0.06)';
        rongqi.appendChild(celan);

        let tuodong = null;
        let pingyi_tuodong = null;
        let xuanzhong = -1;
        let donghua = true;
        let _wl_wendu = 1.0;
        let _wl_tingzhi = false;
        let _wl_pzjs = 0;

        const mosun = 0.82;
        const paichichangshu = Math.max(3500, jiedian.length * 60);
        const tanhuangchangshu = 0.004;
        const lixiangchangdu = Math.max(250, Math.sqrt(jiedian.length) * 48);
        const zhongxinli = 0.003;
        const _paichu_qieduan2 = Math.max(500000, paichichangshu * 4);

        const shijie_dao_pingmu = (wx, wy) => [
            (wx - shijiezhongxin_x) * suofang + kuan / 2 + pingyi_x,
            (wy - shijiezhongxin_y) * suofang + gao / 2 + pingyi_y
        ];
        const pingmu_dao_shijie = (sx, sy) => [
            (sx - kuan / 2 - pingyi_x) / suofang + shijiezhongxin_x,
            (sy - gao / 2 - pingyi_y) / suofang + shijiezhongxin_y
        ];

        const gengxin = () => {
            if (_wl_tingzhi && !tuodong) return;
            _wl_wendu *= 0.99;
            if (_wl_wendu < 0.005) _wl_wendu = 0.005;
            for (let i = 0; i < jiedian.length; i++) {
                let fx = 0, fy = 0;
                for (let j = 0; j < jiedian.length; j++) {
                    if (i === j) continue;
                    const dx = jiedian[i].x - jiedian[j].x;
                    const dy = jiedian[i].y - jiedian[j].y;
                    const d2 = dx * dx + dy * dy;
                    if (d2 > _paichu_qieduan2) continue;
                    const juli = Math.sqrt(d2) || 1;
                    const li = paichichangshu / d2;
                    fx += (dx / juli) * li;
                    fy += (dy / juli) * li;
                }
                fx += (shijiezhongxin_x - jiedian[i].x) * zhongxinli;
                fy += (shijiezhongxin_y - jiedian[i].y) * zhongxinli;
                jiedian[i].vx = (jiedian[i].vx + fx * _wl_wendu) * mosun;
                jiedian[i].vy = (jiedian[i].vy + fy * _wl_wendu) * mosun;
            }
            for (const b of bian) {
                const a = jiedian[b.yuan], c = jiedian[b.mubiao];
                const dx = c.x - a.x, dy = c.y - a.y;
                const juli = Math.sqrt(dx * dx + dy * dy) || 1;
                const li = (juli - lixiangchangdu) * tanhuangchangshu * _wl_wendu;
                const fx = (dx / juli) * li, fy = (dy / juli) * li;
                a.vx += fx; a.vy += fy;
                c.vx -= fx; c.vy -= fy;
            }
            // 关系边弹簧
            for (const b of guanxi_bian) {
                const a = jiedian[b.yuan], c = jiedian[b.mubiao];
                const dx = c.x - a.x, dy = c.y - a.y;
                const juli = Math.sqrt(dx * dx + dy * dy) || 1;
                const li = (juli - lixiangchangdu) * tanhuangchangshu * _wl_wendu;
                const fx = (dx / juli) * li, fy = (dy / juli) * li;
                a.vx += fx; a.vy += fy;
                c.vx -= fx; c.vy -= fy;
            }
            for (const j of jiedian) {
                if (tuodong && j === jiedian[tuodong.idx]) continue;
                j.x += j.vx;
                j.y += j.vy;
                // 微速归零：消除亚像素抖动
                if (Math.abs(j.vx) < 0.01) j.vx = 0;
                if (Math.abs(j.vy) < 0.01) j.vy = 0;
            }
            // 碰撞检测：每3帧执行一次，温度低时跳过（布局已稳定）
            _wl_pzjs++;
            if (_wl_pzjs % 3 === 0 && _wl_wendu > 0.03) {
                for (let i = 0; i < jiedian.length; i++) {
                    if (tuodong && i === tuodong.idx) continue;
                    for (let j = i + 1; j < jiedian.length; j++) {
                        if (tuodong && j === tuodong.idx) continue;
                        const dx = jiedian[j].x - jiedian[i].x;
                        const dy = jiedian[j].y - jiedian[i].y;
                        const dist = Math.sqrt(dx * dx + dy * dy) || 0.1;
                        const minDist = (jiedian[i].banjing + jiedian[j].banjing) * 2.8;
                        if (dist < minDist) {
                            const li = (minDist - dist) / dist * 0.25;
                            jiedian[i].x -= dx * li; jiedian[i].y -= dy * li;
                            jiedian[j].x += dx * li; jiedian[j].y += dy * li;
                        }
                    }
                }
            }
            // 冻结检测
            if (_wl_wendu <= 0.02 && !tuodong) {
                let zs = 0;
                for (const j of jiedian) zs += j.vx * j.vx + j.vy * j.vy;
                if (zs < jiedian.length * 0.05) {
                    _wl_tingzhi = true;
                    for (const j of jiedian) { j.vx = 0; j.vy = 0; }
                }
            }
        };

        const huizhi = () => {
            ctx.clearRect(0, 0, kuan, gao);
            ctx.save();
            // 背景
            ctx.fillStyle = '#F0F4F8';
            ctx.fillRect(0, 0, kuan, gao);
            // 动态点阵网格（限制绘制数量）
            const gs = 28 * suofang;
            if (gs > 10 && (kuan / gs) * (gao / gs) < 4000) {
                ctx.fillStyle = 'rgba(203,213,225,0.18)';
                const ox = ((pingyi_x % gs) + gs) % gs, oy = ((pingyi_y % gs) + gs) % gs;
                for (let gx = ox; gx < kuan; gx += gs)
                    for (let gy = oy; gy < gao; gy += gs)
                        ctx.fillRect(gx, gy, 1.2, 1.2);
            }
            // --- 边：节点色渐变 + 曲线 ---
            for (let bi = 0; bi < bian.length; bi++) {
                const b = bian[bi];
                const a = jiedian[b.yuan], c = jiedian[b.mubiao];
                const [ax, ay] = shijie_dao_pingmu(a.x, a.y);
                const [cx, cy] = shijie_dao_pingmu(c.x, c.y);
                const gaoliang = bi === xuanzhong_bian || xuanzhong === b.yuan || xuanzhong === b.mubiao;
                if (!gaoliang && ((ax < -50 && cx < -50) || (ax > kuan + 50 && cx > kuan + 50) || (ay < -50 && cy < -50) || (ay > gao + 50 && cy > gao + 50))) continue;
                const edx = cx - ax, edy = cy - ay;
                const elen = Math.sqrt(edx * edx + edy * edy) || 1;
                const curv = Math.min(20, elen * 0.04);
                const cpx = (ax + cx) / 2 + (-edy / elen) * curv;
                const cpy = (ay + cy) / 2 + (edx / elen) * curv;
                ctx.beginPath();
                ctx.moveTo(ax, ay);
                ctx.quadraticCurveTo(cpx, cpy, cx, cy);
                ctx.shadowBlur = 0; ctx.shadowColor = 'transparent';
                if (gaoliang) {
                    if (bi === xuanzhong_bian) {
                        ctx.strokeStyle = '#3B82F6';
                        ctx.lineWidth = Math.min(5, 1.5 + b.quanzhong) * Math.min(suofang, 2);
                        ctx.shadowColor = 'rgba(59,130,246,0.3)'; ctx.shadowBlur = 8;
                    } else {
                        const xt = xuanzhong >= 0 ? zhuti[(leixingmap[jiedian[xuanzhong].leixing] || 0) % zhuti.length] : null;
                        ctx.strokeStyle = xt ? xt.zhu + '88' : 'rgba(100,116,139,0.6)';
                        ctx.lineWidth = Math.min(4, 1.2 + b.quanzhong) * Math.min(suofang, 2);
                    }
                } else {
                    const t1 = zhuti[(leixingmap[jiedian[b.yuan].leixing] || 0) % zhuti.length];
                    const ha = Math.round(Math.min(120, 50 + b.quanzhong * 22)).toString(16).padStart(2, '0');
                    ctx.strokeStyle = t1.zhu + ha;
                    ctx.lineWidth = Math.min(3, 0.8 + b.quanzhong * 0.4) * Math.min(suofang, 2);
                }
                ctx.stroke();
                ctx.shadowBlur = 0; ctx.shadowColor = 'transparent';
            }
            // --- 关系边：虚线 + 暖色渐变 + 标签 ---
            ctx.setLineDash([6, 4]);
            for (let gi = 0; gi < guanxi_bian.length; gi++) {
                const gb = guanxi_bian[gi];
                const a = jiedian[gb.yuan], c = jiedian[gb.mubiao];
                const [ax, ay] = shijie_dao_pingmu(a.x, a.y);
                const [cx, cy] = shijie_dao_pingmu(c.x, c.y);
                const gaoliang = gi === xuanzhong_guanxi_bian || xuanzhong === gb.yuan || xuanzhong === gb.mubiao;
                if (!gaoliang && ((ax < -50 && cx < -50) || (ax > kuan + 50 && cx > kuan + 50) || (ay < -50 && cy < -50) || (ay > gao + 50 && cy > gao + 50))) continue;
                const edx = cx - ax, edy = cy - ay;
                const elen = Math.sqrt(edx * edx + edy * edy) || 1;
                const curv = Math.min(30, elen * 0.06);
                const cpx = (ax + cx) / 2 + (-edy / elen) * curv;
                const cpy = (ay + cy) / 2 + (edx / elen) * curv;
                ctx.beginPath();
                ctx.moveTo(ax, ay);
                ctx.quadraticCurveTo(cpx, cpy, cx, cy);
                ctx.shadowBlur = 0; ctx.shadowColor = 'transparent';
                if (gaoliang) {
                    if (gi === xuanzhong_guanxi_bian) {
                        ctx.strokeStyle = '#8B5CF6';
                        ctx.lineWidth = Math.min(5, 1.5 + gb.cishu * 0.3) * Math.min(suofang, 2);
                        ctx.shadowColor = 'rgba(139,92,246,0.3)'; ctx.shadowBlur = 8;
                    } else {
                        ctx.strokeStyle = 'rgba(139,92,246,0.6)';
                        ctx.lineWidth = Math.min(4, 1.2 + gb.cishu * 0.3) * Math.min(suofang, 2);
                    }
                } else {
                    const ha = Math.round(Math.min(140, 60 + gb.cishu * 25)).toString(16).padStart(2, '0');
                    ctx.strokeStyle = '#8B5CF6' + ha;
                    ctx.lineWidth = Math.min(3, 1.5 + gb.cishu * 0.3) * Math.min(suofang, 2);
                }
                ctx.stroke();
                ctx.shadowBlur = 0; ctx.shadowColor = 'transparent';
                // 关系标签文字（仅显示关系类型，理由在侧栏查看）
                if (suofang >= 0.6 && gb.guanxi) {
                    const lx = (ax + cx) / 2 + (-edy / elen) * curv * 0.5;
                    const ly = (ay + cy) / 2 + (edx / elen) * curv * 0.5;
                    const lfs = Math.max(9, 10 * Math.min(suofang, 1.3));
                    ctx.font = `500 ${lfs}px -apple-system,"Microsoft YaHei",sans-serif`;
                    const tw = ctx.measureText(gb.guanxi).width;
                    const lpp = 5, lph = lfs + 6;
                    ctx.fillStyle = gaoliang ? 'rgba(139,92,246,0.12)' : 'rgba(255,255,255,0.85)';
                    const rrx = lx - tw / 2 - lpp, rry = ly - lph / 2;
                    ctx.beginPath();
                    ctx.roundRect(rrx, rry, tw + lpp * 2, lph, lph / 2);
                    ctx.fill();
                    ctx.strokeStyle = gaoliang ? 'rgba(139,92,246,0.3)' : 'rgba(203,213,225,0.4)';
                    ctx.lineWidth = 0.5;
                    ctx.stroke();
                    ctx.fillStyle = gaoliang ? '#7C3AED' : '#6B7280';
                    ctx.textAlign = 'center';
                    ctx.textBaseline = 'middle';
                    ctx.fillText(gb.guanxi, lx, ly);
                }
            }
            ctx.setLineDash([]);
            // --- 节点 + 智能标签 ---
            const xianshiwenzi = suofang >= 0.4;
            const duoJiedian = jiedian.length > 12;
            const _jianhua = jiedian.length > 25;
            const _linjiSet = new Set();
            if (xuanzhong >= 0) {
                for (const b of bian) {
                    if (b.yuan === xuanzhong) _linjiSet.add(b.mubiao);
                    if (b.mubiao === xuanzhong) _linjiSet.add(b.yuan);
                }
                for (const b of guanxi_bian) {
                    if (b.yuan === xuanzhong) _linjiSet.add(b.mubiao);
                    if (b.mubiao === xuanzhong) _linjiSet.add(b.yuan);
                }
            }
            for (let i = 0; i < jiedian.length; i++) {
                const j = jiedian[i];
                const [sx, sy] = shijie_dao_pingmu(j.x, j.y);
                if (sx < -80 || sx > kuan + 80 || sy < -80 || sy > gao + 80) continue;
                const r = j.banjing * Math.min(suofang, 2.5);
                const t = zhuti[(leixingmap[j.leixing] || 0) % zhuti.length];
                const isXz = i === xuanzhong;
                const isLinji = _linjiSet.has(i);
                if (isXz || isLinji) {
                    // 选中/邻居节点：精细渲染
                    ctx.beginPath();
                    ctx.arc(sx, sy, r * (isXz ? 1.6 : 1.35), 0, Math.PI * 2);
                    ctx.fillStyle = t.zhu + (isXz ? '20' : '10');
                    ctx.fill();
                    const nG = ctx.createRadialGradient(sx - r * 0.3, sy - r * 0.35, r * 0.05, sx, sy, r);
                    nG.addColorStop(0, '#FFFFFF');
                    nG.addColorStop(0.3, isXz ? '#FFFFFF' : t.qian + 'DD');
                    nG.addColorStop(0.7, isXz ? t.qian : t.qian);
                    nG.addColorStop(1, isXz ? t.bian : t.bian + 'CC');
                    ctx.beginPath();
                    ctx.arc(sx, sy, r, 0, Math.PI * 2);
                    ctx.fillStyle = nG;
                    ctx.fill();
                    ctx.strokeStyle = t.zhu + (isXz ? 'CC' : 'AA');
                    ctx.lineWidth = isXz ? 2.5 : 2;
                    ctx.stroke();
                    if (r > 8) {
                        const hlR = r * 0.38;
                        const hlG = ctx.createRadialGradient(sx - r * 0.22, sy - r * 0.28, 0, sx - r * 0.22, sy - r * 0.28, hlR);
                        hlG.addColorStop(0, 'rgba(255,255,255,0.7)');
                        hlG.addColorStop(1, 'rgba(255,255,255,0)');
                        ctx.beginPath();
                        ctx.arc(sx - r * 0.22, sy - r * 0.28, hlR, 0, Math.PI * 2);
                        ctx.fillStyle = hlG;
                        ctx.fill();
                    }
                } else if (_jianhua) {
                    // 大图简化渲染：纯色填充
                    ctx.beginPath();
                    ctx.arc(sx, sy, r, 0, Math.PI * 2);
                    ctx.fillStyle = t.qian;
                    ctx.fill();
                    ctx.strokeStyle = t.zhu + '88';
                    ctx.lineWidth = 1.2;
                    ctx.stroke();
                } else {
                    // 小图精细渲染
                    const nG = ctx.createRadialGradient(sx - r * 0.3, sy - r * 0.35, r * 0.05, sx, sy, r);
                    nG.addColorStop(0, '#FFFFFF');
                    nG.addColorStop(0.3, t.qian + 'DD');
                    nG.addColorStop(0.7, t.qian);
                    nG.addColorStop(1, t.bian + 'CC');
                    ctx.beginPath();
                    ctx.arc(sx, sy, r, 0, Math.PI * 2);
                    ctx.fillStyle = nG;
                    ctx.fill();
                    ctx.strokeStyle = t.zhu + '99';
                    ctx.lineWidth = 1.2;
                    ctx.stroke();
                    if (r > 6) {
                        const hlR = r * 0.38;
                        const hlG = ctx.createRadialGradient(sx - r * 0.22, sy - r * 0.28, 0, sx - r * 0.22, sy - r * 0.28, hlR);
                        hlG.addColorStop(0, 'rgba(255,255,255,0.7)');
                        hlG.addColorStop(1, 'rgba(255,255,255,0)');
                        ctx.beginPath();
                        ctx.arc(sx - r * 0.22, sy - r * 0.28, hlR, 0, Math.PI * 2);
                        ctx.fillStyle = hlG;
                        ctx.fill();
                    }
                }
                ctx.shadowBlur = 0; ctx.shadowColor = 'transparent';
                // 智能标签：节点多时只显示重要的
                let labelAlpha = 1;
                if (duoJiedian) {
                    if (xuanzhong >= 0) {
                        labelAlpha = isXz ? 1 : isLinji ? 0.9 : 0.1;
                    } else {
                        labelAlpha = _du[i] >= Math.max(2, _zuidade * 0.25) ? 0.95 : 0.15;
                    }
                }
                if (xianshiwenzi && r > 3 && labelAlpha > 0.08) {
                    const fs = Math.max(10, 12 * Math.min(suofang, 1.5));
                    ctx.font = `500 ${fs}px -apple-system,"Microsoft YaHei",sans-serif`;
                    const maxLW = Math.max(60, 140 * Math.min(suofang, 1.5));
                    let wenzi = j.zhi;
                    if (wenzi.length > 10) wenzi = wenzi.slice(0, 10);
                    const _tw0 = ctx.measureText(wenzi).width;
                    if (_tw0 > maxLW) {
                        wenzi = wenzi.slice(0, Math.max(1, Math.floor(wenzi.length * (maxLW - 8) / _tw0))) + '…';
                    }
                    const tw = ctx.measureText(wenzi).width;
                    const ppx = 7, ppy = 3, pw = tw + ppx * 2, ph = fs + ppy * 2, prad = ph / 2;
                    const ly = sy + r + ph / 2 + 4;
                    const pl = sx - pw / 2, pt = ly - ph / 2;
                    ctx.beginPath();
                    ctx.moveTo(pl + prad, pt);
                    ctx.arcTo(pl + pw, pt, pl + pw, pt + ph, prad);
                    ctx.arcTo(pl + pw, pt + ph, pl, pt + ph, prad);
                    ctx.arcTo(pl, pt + ph, pl, pt, prad);
                    ctx.arcTo(pl, pt, pl + prad, pt, prad);
                    ctx.closePath();
                    ctx.fillStyle = `rgba(255,255,255,${(0.88 * labelAlpha).toFixed(2)})`;
                    ctx.fill();
                    if (labelAlpha > 0.5) {
                        ctx.strokeStyle = `rgba(226,232,240,${(0.5 * labelAlpha).toFixed(2)})`;
                        ctx.lineWidth = 0.5;
                        ctx.stroke();
                    }
                    ctx.fillStyle = `rgba(30,41,59,${labelAlpha.toFixed(2)})`;
                    ctx.textAlign = 'center';
                    ctx.textBaseline = 'middle';
                    ctx.fillText(wenzi, sx, ly);
                }
            }
            ctx.restore();
        };

        let _qianSfb = '';
        const xunhuan = () => {
            if (!donghua) return;
            gengxin();
            huizhi();
            const _sfb = Math.round(suofang * 100) + '%';
            if (_sfb !== _qianSfb) { suofangxianshi.textContent = _sfb; _qianSfb = _sfb; }
            requestAnimationFrame(xunhuan);
        };

        const zhaojiedian = (mx, my) => {
            const [wx, wy] = pingmu_dao_shijie(mx, my);
            for (let i = jiedian.length - 1; i >= 0; i--) {
                const dx = jiedian[i].x - wx, dy = jiedian[i].y - wy;
                const r = jiedian[i].banjing + 4 / suofang;
                if (dx * dx + dy * dy <= r * r) return i;
            }
            return -1;
        };

        const zhaobiaobian = (mx, my) => {
            const [wx, wy] = pingmu_dao_shijie(mx, my);
            let zuijin = -1, zuijinjuli = 12 / suofang;
            for (let i = 0; i < bian.length; i++) {
                const a = jiedian[bian[i].yuan], c = jiedian[bian[i].mubiao];
                const dx = c.x - a.x, dy = c.y - a.y;
                const len2 = dx * dx + dy * dy;
                if (len2 === 0) continue;
                let t = ((wx - a.x) * dx + (wy - a.y) * dy) / len2;
                t = Math.max(0, Math.min(1, t));
                const px = a.x + t * dx, py = a.y + t * dy;
                const dist = Math.sqrt((wx - px) * (wx - px) + (wy - py) * (wy - py));
                if (dist < zuijinjuli) { zuijinjuli = dist; zuijin = i; }
            }
            return zuijin;
        };

        const zhaoguanxibian = (mx, my) => {
            const [wx, wy] = pingmu_dao_shijie(mx, my);
            let zuijin = -1, zuijinjuli = 12 / suofang;
            for (let i = 0; i < guanxi_bian.length; i++) {
                const a = jiedian[guanxi_bian[i].yuan], c = jiedian[guanxi_bian[i].mubiao];
                const dx = c.x - a.x, dy = c.y - a.y;
                const len2 = dx * dx + dy * dy;
                if (len2 === 0) continue;
                let t = ((wx - a.x) * dx + (wy - a.y) * dy) / len2;
                t = Math.max(0, Math.min(1, t));
                const px = a.x + t * dx, py = a.y + t * dy;
                const dist = Math.sqrt((wx - px) * (wx - px) + (wy - py) * (wy - py));
                if (dist < zuijinjuli) { zuijinjuli = dist; zuijin = i; }
            }
            return zuijin;
        };

        let yidong_juli = 0;
        let anxia_weizhi = null;
        let zuihou_dianji_shijian = 0;
        let zuihou_dianji_id = '';
        let xuanzhong_bian = -1;
        let xuanzhong_guanxi_bian = -1;

        canvas.onmousedown = e => {
            const rect = canvas.getBoundingClientRect();
            const mx = (e.clientX - rect.left) * (kuan / rect.width);
            const my = (e.clientY - rect.top) * (gao / rect.height);
            yidong_juli = 0;
            anxia_weizhi = { x: e.clientX, y: e.clientY };
            const idx = zhaojiedian(mx, my);
            if (idx >= 0) {
                if (_wl_tingzhi) { _wl_tingzhi = false; _wl_wendu = 0.15; }
                tuodong = { idx, ox: e.clientX, oy: e.clientY };
                canvas.style.cursor = 'grabbing';
            } else {
                pingyi_tuodong = { ox: e.clientX, oy: e.clientY };
                canvas.style.cursor = 'move';
            }
        };
        canvas.onmousemove = e => {
            const rect = canvas.getBoundingClientRect();
            const mx = (e.clientX - rect.left) * (kuan / rect.width);
            const my = (e.clientY - rect.top) * (gao / rect.height);
            if (tuodong) {
                const bili_x = kuan / rect.width, bili_y = gao / rect.height;
                const dx = (e.clientX - tuodong.ox) * bili_x / suofang;
                const dy = (e.clientY - tuodong.oy) * bili_y / suofang;
                yidong_juli += Math.abs(dx) + Math.abs(dy);
                jiedian[tuodong.idx].x += dx;
                jiedian[tuodong.idx].y += dy;
                jiedian[tuodong.idx].vx = 0;
                jiedian[tuodong.idx].vy = 0;
                tuodong.ox = e.clientX;
                tuodong.oy = e.clientY;
            } else if (pingyi_tuodong) {
                const bili_x = kuan / rect.width, bili_y = gao / rect.height;
                pingyi_x += (e.clientX - pingyi_tuodong.ox) * bili_x;
                pingyi_y += (e.clientY - pingyi_tuodong.oy) * bili_y;
                pingyi_tuodong.ox = e.clientX;
                pingyi_tuodong.oy = e.clientY;
            }
            const idx = zhaojiedian(mx, my);
            xuanzhong = idx;
            if (idx >= 0) {
                xuanzhong_bian = -1;
                xuanzhong_guanxi_bian = -1;
                xinxi.style.display = 'block';
                xinxi.innerHTML = `<b>${jiedian[idx].leixing}</b>: ${jiedian[idx].zhi}`;
                canvas.style.cursor = tuodong ? 'grabbing' : 'pointer';
            } else {
                const kejianche = !tuodong && !pingyi_tuodong;
                const gx_idx = kejianche ? zhaoguanxibian(mx, my) : -1;
                const bian_idx = gx_idx < 0 && kejianche ? zhaobiaobian(mx, my) : -1;
                xuanzhong_guanxi_bian = gx_idx;
                xuanzhong_bian = bian_idx;
                if (gx_idx >= 0) {
                    const gb = guanxi_bian[gx_idx];
                    xinxi.style.display = 'block';
                    xinxi.innerHTML = `<b>${jiedian[gb.yuan].zhi}</b> — <span style="color:#8B5CF6">${gb.guanxi}</span> — <b>${jiedian[gb.mubiao].zhi}</b> (${gb.cishu}篇日报)`;
                    canvas.style.cursor = 'pointer';
                } else if (bian_idx >= 0) {
                    const b = bian[bian_idx];
                    xinxi.style.display = 'block';
                    xinxi.innerHTML = `<b>${jiedian[b.yuan].zhi}</b> ↔ <b>${jiedian[b.mubiao].zhi}</b> (共现${b.quanzhong}次)`;
                    canvas.style.cursor = 'pointer';
                } else {
                    xinxi.style.display = 'none';
                    canvas.style.cursor = tuodong ? 'grabbing' : (pingyi_tuodong ? 'move' : 'grab');
                }
            }
        };
        canvas.onmouseup = e => {
            const dianji_jiedian = tuodong && yidong_juli < 5 ? tuodong.idx : -1;
            const shifou_weidong = anxia_weizhi && (Math.abs(e.clientX - anxia_weizhi.x) + Math.abs(e.clientY - anxia_weizhi.y)) < 6;
            tuodong = null;
            pingyi_tuodong = null;
            anxia_weizhi = null;
            canvas.style.cursor = 'grab';
            if (dianji_jiedian >= 0) {
                const xianzai = Date.now();
                const dangqianid = jiedian[dianji_jiedian].id;
                if (xianzai - zuihou_dianji_shijian < 400 && dangqianid === zuihou_dianji_id) {
                    zuihou_dianji_shijian = 0;
                    zuihou_dianji_id = '';
                    this._tupu_jiazai_biaoqianid(dangqianid);
                } else {
                    zuihou_dianji_shijian = xianzai;
                    zuihou_dianji_id = dangqianid;
                    this._tupu_xianshi_celan_jiedian(jiedian[dianji_jiedian]);
                }
            } else if (shifou_weidong) {
                const rect = canvas.getBoundingClientRect();
                const mx = (e.clientX - rect.left) * (kuan / rect.width);
                const my = (e.clientY - rect.top) * (gao / rect.height);
                const gx_idx = zhaoguanxibian(mx, my);
                if (gx_idx >= 0) {
                    const gb = guanxi_bian[gx_idx];
                    this._tupu_xianshi_celan_guanxibian(jiedian[gb.yuan], jiedian[gb.mubiao], gb);
                } else {
                    const bian_idx = zhaobiaobian(mx, my);
                    if (bian_idx >= 0) {
                        const b = bian[bian_idx];
                        this._tupu_xianshi_celan_bian(jiedian[b.yuan], jiedian[b.mubiao], b.quanzhong);
                    } else {
                        const celan = document.getElementById('tupu_celan');
                        if (celan) celan.style.display = 'none';
                    }
                }
            }
        };
        canvas.onmouseleave = () => { tuodong = null; pingyi_tuodong = null; xuanzhong = -1; xinxi.style.display = 'none'; };
        canvas.onwheel = e => {
            e.preventDefault();
            const yinzi = e.deltaY < 0 ? 1.15 : 1 / 1.15;
            suofang = Math.max(0.15, Math.min(6, suofang * yinzi));
        };

        donghua = true;
        xunhuan();
        this._tupu_tingzhi = () => { donghua = false; document.removeEventListener('fullscreenchange', _tiaozheng_chicun); };
    }

    _tupu_celan_guanbi_html() {
        return '<button onclick="document.getElementById(\'tupu_celan\').style.display=\'none\'" style="width:28px;height:28px;border-radius:6px;background:transparent;border:none;cursor:pointer;color:#94A3B8;display:flex;align-items:center;justify-content:center;transition:all 150ms;margin:0;min-height:0;padding:0;flex-shrink:0" onmouseenter="this.style.background=\'#F1F5F9\';this.style.color=\'#475569\'" onmouseleave="this.style.background=\'transparent\';this.style.color=\'#94A3B8\'"><svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="4" y1="4" x2="12" y2="12"/><line x1="12" y1="4" x2="4" y2="12"/></svg></button>';
    }

    _tupu_celan_tou_html(tubiao, biaoti, tubiao_bg) {
        return `<div style="display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid #F1F5F9;flex-shrink:0;background:linear-gradient(135deg,#F8FAFC,#EFF6FF)">
            <div style="display:flex;align-items:center;gap:8px">
                <div style="width:28px;height:28px;border-radius:8px;background:${tubiao_bg || 'linear-gradient(135deg,#3B82F6,#2563EB)'};display:flex;align-items:center;justify-content:center;flex-shrink:0">${tubiao}</div>
                <span style="font-size:14px;font-weight:600;color:#0F172A">${biaoti}</span>
            </div>
            ${this._tupu_celan_guanbi_html()}
        </div>`;
    }

    _tupu_xianshi_celan_jiedian(j) {
        const celan = document.getElementById('tupu_celan');
        if (!celan) return;
        celan.style.display = 'block';
        const tubiao = '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="white" stroke-width="2" stroke-linecap="round"><circle cx="8" cy="8" r="4.5"/></svg>';
        celan.innerHTML = `
            <div style="display:flex;flex-direction:column;height:100%">
                ${this._tupu_celan_tou_html(tubiao, '\u8282\u70b9\u8be6\u60c5')}
                <div style="flex:1;overflow-y:auto;padding:16px">
                    <div style="background:linear-gradient(135deg,#F8FAFC,#F1F5F9);border:1px solid #E2E8F0;border-radius:10px;padding:14px;margin-bottom:14px;border-left:3px solid #3B82F6">
                        <div style="display:inline-block;padding:2px 8px;background:#EFF6FF;color:#3B82F6;border-radius:10px;font-size:11px;font-weight:600;letter-spacing:0.3px">${j.leixing}</div>
                        <div style="font-size:14px;font-weight:600;color:#0F172A;margin-top:8px;line-height:1.5">${j.zhi}</div>
                    </div>
                    <button class="aq-btn aq-btn-xiao" onclick="ribao_tupu_jiazai_biaoqianid_celan('${j.id}')" style="width:100%;display:flex;align-items:center;justify-content:center;gap:6px;background:transparent;color:#3B82F6;border:1.5px solid #BFDBFE;border-radius:8px;padding:9px;margin-bottom:16px;font-weight:500;transition:all 150ms" onmouseenter="this.style.background='#EFF6FF';this.style.borderColor='#3B82F6'" onmouseleave="this.style.background='transparent';this.style.borderColor='#BFDBFE'">
                        <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><circle cx="7" cy="7" r="4.5"/><line x1="10.5" y1="10.5" x2="14" y2="14"/><line x1="5.5" y1="7" x2="8.5" y2="7"/><line x1="7" y1="5.5" x2="7" y2="8.5"/></svg>
                        \u67e5\u770b\u5b50\u56fe
                    </button>
                    <div style="display:flex;align-items:center;gap:6px;margin-bottom:10px">
                        <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="#64748B" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="12" height="11" rx="1.5"/><line x1="2" y1="7" x2="14" y2="7"/><line x1="6" y1="3" x2="6" y2="7"/></svg>
                        <span style="font-size:13px;font-weight:600;color:#475569">\u5173\u8054\u65e5\u62a5</span>
                    </div>
                    <div id="tupu_celan_ribaolie"><p style="color:#94A3B8;font-size:13px">\u52a0\u8f7d\u4e2d...</p></div>
                </div>
            </div>`;
        this._tupu_celan_yeshu = 1;
        this._tupu_celan_biaoqianid = j.id;
        this._tupu_celan_yuan_id = null;
        this._tupu_celan_mubiao_id = null;
        this._tupu_celan_jiazai_jiedian_ribao(j.id, 1);
    }

    _tupu_xianshi_celan_guanxibian(yuan, mubiao, gb) {
        const celan = document.getElementById('tupu_celan');
        if (!celan) return;
        celan.style.display = 'block';
        const tubiao = '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="white" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M4 8h8"/><circle cx="3" cy="8" r="2"/><circle cx="13" cy="8" r="2"/></svg>';
        celan.innerHTML = `
            <div style="display:flex;flex-direction:column;height:100%">
                ${this._tupu_celan_tou_html(tubiao, 'AI \u5173\u7cfb\u8be6\u60c5', 'linear-gradient(135deg,#8B5CF6,#7C3AED)')}
                <div style="flex:1;overflow-y:auto;padding:16px">
                    <div style="background:linear-gradient(135deg,#F5F3FF,#FDF2F8);border:1px solid #DDD6FE;border-radius:10px;padding:14px;margin-bottom:16px;border-left:3px solid #8B5CF6">
                        <div style="display:flex;align-items:center;gap:8px;flex-wrap:wrap">
                            <span style="background:#EDE9FE;color:#7C3AED;padding:3px 10px;border-radius:12px;font-size:13px;font-weight:600">${yuan.zhi}</span>
                            <span style="display:inline-flex;align-items:center;gap:4px;padding:2px 8px;background:rgba(139,92,246,0.1);color:#7C3AED;border-radius:8px;font-size:11px;font-weight:600">
                                <svg width="10" height="10" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="2" y1="8" x2="14" y2="8"/></svg>
                                ${gb.guanxi || '\u76f8\u5173'}
                                <svg width="10" height="10" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="2" y1="8" x2="14" y2="8"/></svg>
                            </span>
                            <span style="background:#EDE9FE;color:#7C3AED;padding:3px 10px;border-radius:12px;font-size:13px;font-weight:600">${mubiao.zhi}</span>
                        </div>
                        <div style="font-size:12px;color:#6B7280;margin-top:10px">\u51fa\u73b0\u4e8e <b style="color:#7C3AED">${gb.cishu}</b> \u7bc7\u65e5\u62a5</div>
                        ${gb.miaoshu ? `<div style="font-size:12px;color:#475569;margin-top:10px;line-height:1.6;padding:10px 12px;background:rgba(255,255,255,0.7);border-radius:8px;border:1px solid rgba(221,214,254,0.4)">${gb.miaoshu}</div>` : ''}
                    </div>
                    <div style="display:flex;align-items:center;gap:6px;margin-bottom:10px">
                        <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="#64748B" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="12" height="11" rx="1.5"/><line x1="2" y1="7" x2="14" y2="7"/><line x1="6" y1="3" x2="6" y2="7"/></svg>
                        <span style="font-size:13px;font-weight:600;color:#475569">\u5173\u8054\u65e5\u62a5</span>
                    </div>
                    <div id="tupu_celan_ribaolie"><p style="color:#94A3B8;font-size:13px">\u52a0\u8f7d\u4e2d...</p></div>
                </div>
            </div>`;
        this._tupu_celan_yeshu = 1;
        this._tupu_celan_biaoqianid = null;
        this._tupu_celan_yuan_id = yuan.id;
        this._tupu_celan_mubiao_id = mubiao.id;
        this._tupu_celan_jiazai_bian_ribao(yuan.id, mubiao.id, 1);
    }

    _tupu_xianshi_celan_bian(yuan, mubiao, quanzhong) {
        const celan = document.getElementById('tupu_celan');
        if (!celan) return;
        celan.style.display = 'block';
        const tubiao = '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="white" stroke-width="2" stroke-linecap="round"><line x1="3" y1="8" x2="13" y2="8"/></svg>';
        celan.innerHTML = `
            <div style="display:flex;flex-direction:column;height:100%">
                ${this._tupu_celan_tou_html(tubiao, '\u5171\u73b0\u8be6\u60c5', 'linear-gradient(135deg,#0EA5E9,#0284C7)')}
                <div style="flex:1;overflow-y:auto;padding:16px">
                    <div style="background:linear-gradient(135deg,#F0F9FF,#F1F5F9);border:1px solid #E2E8F0;border-radius:10px;padding:14px;margin-bottom:16px;border-left:3px solid #0EA5E9">
                        <div style="border:1px solid #E0F2FE;border-radius:8px;padding:10px 12px;background:#FFFFFF">
                            <div style="display:inline-block;padding:2px 8px;background:#E0F2FE;color:#0369A1;border-radius:10px;font-size:11px;font-weight:600;margin-bottom:4px">${yuan.leixing}</div>
                            <div style="font-size:14px;font-weight:600;color:#0F172A;line-height:1.5">${yuan.zhi}</div>
                        </div>
                        <div style="display:flex;align-items:center;justify-content:center;padding:6px 0">
                            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#94A3B8" stroke-width="1.5" stroke-linecap="round"><line x1="8" y1="3" x2="8" y2="13"/><polyline points="5.5 10 8 13 10.5 10"/><polyline points="5.5 6 8 3 10.5 6"/></svg>
                        </div>
                        <div style="border:1px solid #E0F2FE;border-radius:8px;padding:10px 12px;background:#FFFFFF">
                            <div style="display:inline-block;padding:2px 8px;background:#E0F2FE;color:#0369A1;border-radius:10px;font-size:11px;font-weight:600;margin-bottom:4px">${mubiao.leixing}</div>
                            <div style="font-size:14px;font-weight:600;color:#0F172A;line-height:1.5">${mubiao.zhi}</div>
                        </div>
                        <div style="font-size:12px;color:#64748B;margin-top:10px">\u5171\u73b0 <b style="color:#0369A1">${quanzhong}</b> \u6b21</div>
                    </div>
                    <div style="display:flex;align-items:center;gap:6px;margin-bottom:10px">
                        <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="#64748B" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="12" height="11" rx="1.5"/><line x1="2" y1="7" x2="14" y2="7"/><line x1="6" y1="3" x2="6" y2="7"/></svg>
                        <span style="font-size:13px;font-weight:600;color:#475569">\u5171\u73b0\u65e5\u62a5</span>
                    </div>
                    <div id="tupu_celan_ribaolie"><p style="color:#94A3B8;font-size:13px">\u52a0\u8f7d\u4e2d...</p></div>
                </div>
            </div>`;
        this._tupu_celan_yeshu = 1;
        this._tupu_celan_biaoqianid = null;
        this._tupu_celan_yuan_id = yuan.id;
        this._tupu_celan_mubiao_id = mubiao.id;
        this._tupu_celan_jiazai_bian_ribao(yuan.id, mubiao.id, 1);
    }

    _tupu_celan_xuanran_ribaolie(liebiao, zongshu, yeshu, meiyetiaoshu) {
        const rongqi = document.getElementById('tupu_celan_ribaolie');
        if (!rongqi) return;
        if (!liebiao || liebiao.length === 0) {
            rongqi.innerHTML = '<div style="text-align:center;padding:20px 0"><svg width="32" height="32" viewBox="0 0 16 16" fill="none" stroke="#CBD5E1" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" style="margin-bottom:8px"><rect x="2" y="3" width="12" height="11" rx="1.5"/><line x1="2" y1="7" x2="14" y2="7"/><line x1="6" y1="3" x2="6" y2="7"/></svg><div style="color:#94A3B8;font-size:13px">\u6682\u65e0\u5173\u8054\u65e5\u62a5</div></div>';
            return;
        }
        const zongyeshu = Math.ceil(zongshu / meiyetiaoshu);
        let html = `<div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:10px">
            <span style="display:inline-flex;align-items:center;gap:4px;padding:2px 8px;background:#F1F5F9;border-radius:8px;font-size:11px;color:#64748B;font-weight:500">
                <b style="color:#334155">${zongshu}</b> \u7bc7
            </span>
            ${zongyeshu > 1 ? `<span style="font-size:11px;color:#94A3B8">${yeshu} / ${zongyeshu}</span>` : ''}
        </div>`;
        for (const r of liebiao) {
            const riqi = jiexishijian(r.fabushijian);
            const zhaiyao = (r.neirong || '').replace(/<[^>]+>/g, '').substring(0, 80);
            html += `<div onclick="ribao_tupu_chakanribao('${r.id}')" style="border:1px solid #E2E8F0;border-radius:8px;padding:10px 12px;margin-bottom:8px;cursor:pointer;transition:all 150ms;border-left:3px solid transparent" onmouseenter="this.style.borderLeftColor='#3B82F6';this.style.background='#F8FAFC';this.style.boxShadow='0 2px 8px rgba(0,0,0,0.04)'" onmouseleave="this.style.borderLeftColor='transparent';this.style.background='';this.style.boxShadow='none'">
                <div style="display:flex;align-items:center;gap:6px;font-size:11px;color:#94A3B8">
                    <svg width="11" height="11" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><circle cx="8" cy="8" r="6"/><polyline points="8 5 8 8.5 10.5 10"/></svg>
                    ${riqi}
                    ${r.fabuzhemingcheng ? `<span style="padding:1px 6px;background:#F1F5F9;border-radius:6px;color:#64748B;font-size:10px">${r.fabuzhemingcheng}</span>` : ''}
                </div>
                <div style="font-size:12px;color:#334155;margin-top:6px;line-height:1.6;display:-webkit-box;-webkit-line-clamp:3;-webkit-box-orient:vertical;overflow:hidden">${zhaiyao}${zhaiyao.length >= 80 ? '...' : ''}</div>
            </div>`;
        }
        if (zongyeshu > 1) {
            html += '<div style="display:flex;gap:6px;justify-content:center;margin-top:10px">';
            if (yeshu > 1) html += '<button onclick="ribao_tupu_celan_shangyiye()" style="display:inline-flex;align-items:center;gap:4px;padding:5px 12px;background:transparent;border:1px solid #E2E8F0;border-radius:6px;font-size:12px;color:#475569;cursor:pointer;transition:all 150ms;margin:0;min-height:0" onmouseenter="this.style.background=\'#F1F5F9\'" onmouseleave="this.style.background=\'transparent\'"><svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><polyline points="10 3 5 8 10 13"/></svg>\u4e0a\u4e00\u9875</button>';
            if (yeshu < zongyeshu) html += '<button onclick="ribao_tupu_celan_xiayiye()" style="display:inline-flex;align-items:center;gap:4px;padding:5px 12px;background:transparent;border:1px solid #E2E8F0;border-radius:6px;font-size:12px;color:#475569;cursor:pointer;transition:all 150ms;margin:0;min-height:0" onmouseenter="this.style.background=\'#F1F5F9\'" onmouseleave="this.style.background=\'transparent\'">\u4e0b\u4e00\u9875<svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><polyline points="6 3 11 8 6 13"/></svg></button>';
            html += '</div>';
        }
        rongqi.innerHTML = html;
        this._tupu_celan_yeshu = yeshu;
    }

    async _tupu_celan_jiazai_jiedian_ribao(biaoqianid, yeshu) {
        const meiyetiaoshu = 5;
        const jg = await this.luoji.tupu_ribao_fenye(biaoqianid, yeshu, meiyetiaoshu);
        if (!jg || jg.zhuangtaima !== 200) {
            const rongqi = document.getElementById('tupu_celan_ribaolie');
            if (rongqi) rongqi.innerHTML = '<p style="color:#EF4444;font-size:13px">加载失败</p>';
            return;
        }
        const { liebiao = [], zongshu = 0 } = jg.shuju || {};
        this._tupu_celan_xuanran_ribaolie(liebiao, zongshu, yeshu, meiyetiaoshu);
    }

    async _tupu_celan_jiazai_bian_ribao(yuan_biaoqianid, mubiao_biaoqianid, yeshu) {
        const meiyetiaoshu = 5;
        const jg = await this.luoji.tupu_bian_ribao_fenye(yuan_biaoqianid, mubiao_biaoqianid, yeshu, meiyetiaoshu);
        if (!jg || jg.zhuangtaima !== 200) {
            const rongqi = document.getElementById('tupu_celan_ribaolie');
            if (rongqi) rongqi.innerHTML = '<p style="color:#EF4444;font-size:13px">加载失败</p>';
            return;
        }
        const { liebiao = [], zongshu = 0 } = jg.shuju || {};
        this._tupu_celan_xuanran_ribaolie(liebiao, zongshu, yeshu, meiyetiaoshu);
    }

    tupu_celan_shangyiye() {
        if (this._tupu_celan_yeshu <= 1) return;
        const yeshu = this._tupu_celan_yeshu - 1;
        if (this._tupu_celan_yuan_id && this._tupu_celan_mubiao_id) {
            this._tupu_celan_jiazai_bian_ribao(this._tupu_celan_yuan_id, this._tupu_celan_mubiao_id, yeshu);
        } else if (this._tupu_celan_biaoqianid) {
            this._tupu_celan_jiazai_jiedian_ribao(this._tupu_celan_biaoqianid, yeshu);
        }
    }

    tupu_celan_xiayiye() {
        const yeshu = this._tupu_celan_yeshu + 1;
        if (this._tupu_celan_yuan_id && this._tupu_celan_mubiao_id) {
            this._tupu_celan_jiazai_bian_ribao(this._tupu_celan_yuan_id, this._tupu_celan_mubiao_id, yeshu);
        } else if (this._tupu_celan_biaoqianid) {
            this._tupu_celan_jiazai_jiedian_ribao(this._tupu_celan_biaoqianid, yeshu);
        }
    }

    tupu_chakanribao(ribaoid) {
        this.qiehuanshitu('ribao');
        setTimeout(() => this.bianji(ribaoid), 200);
    }

    async tupu_sousuo_shuru(guanjianci) {
        clearTimeout(this._tupu_sousuo_timer);
        const jieguo_div = document.getElementById('tupu_sousuo_jieguo');
        if (!jieguo_div) return;
        if (!guanjianci || guanjianci.trim().length === 0) {
            jieguo_div.style.display = 'none';
            return;
        }
        this._tupu_sousuo_timer = setTimeout(async () => {
            const jg = await this.luoji.tupu_sousuo(guanjianci.trim());
            if (!jg || jg.zhuangtaima !== 200 || !jg.shuju?.liebiao) {
                jieguo_div.style.display = 'none';
                return;
            }
            const liebiao = jg.shuju.liebiao;
            if (liebiao.length === 0) {
                jieguo_div.innerHTML = '<div style="padding:12px;color:#94A3B8;font-size:13px">未找到匹配标签</div>';
                jieguo_div.style.display = 'block';
                return;
            }
            let html = '';
            for (const item of liebiao) {
                html += `<div style="padding:8px 12px;cursor:pointer;border-bottom:1px solid #F1F5F9;font-size:13px" onmouseover="this.style.background='#F8FAFC'" onmouseout="this.style.background=''" onclick="ribao_tupu_sousuo_xuanze('${item.biaoqianid}')">
                    <span style="color:#0F172A">${item.zhi}</span>
                    <span style="color:#94A3B8;font-size:11px;margin-left:6px">${item.leixingmingcheng}</span>
                    <span style="color:#64748B;font-size:11px;float:right">${item.ribao_zongshu || 0}篇</span>
                </div>`;
            }
            jieguo_div.innerHTML = html;
            jieguo_div.style.display = 'block';
        }, 300);
    }

    tupu_sousuo_xuanze(biaoqianid) {
        const jieguo_div = document.getElementById('tupu_sousuo_jieguo');
        if (jieguo_div) jieguo_div.style.display = 'none';
        const shuru = document.getElementById('tupu_sousuo_shuru');
        if (shuru) shuru.value = '';
        this._tupu_jiazai_biaoqianid(biaoqianid);
    }

    qiehuanquanbu() {
        this.chakanquanbu = !this.chakanquanbu;
        this.dangqianyeshu = 1;
        this.qingchusousuo();
        this.xuanran();
        this.shuaxinribaoliebiao();
    }

    shezhiquanxian(shifouquanxian) {
        this.shifouquanxian = !!shifouquanxian;
        this.chakanquanbu = false;
        this.dangqianshitu = 'ribao';
        this.dangqianyeshu = 1;
        this.shuaxindangqianshitu();
    }

    quanxuan_ribao(cb) { gj.quanxuan('rb_pl_xz', cb); }
    quanxuan_biaoqian(cb) { gj.quanxuan('bq_pl_xz', cb); }
    quanxuan_leixing(cb) { gj.quanxuan('lx_pl_xz', cb); }
    quanxuan_renwu(cb) { gj.quanxuan('rw_pl_xz', cb); }

    async piliangshanchu_ribao() {
        await gj.piliangshanchu(this.luoji, { leibie: 'rb_pl_xz', mingcheng: '日报', shanchufn: id => this.luoji.ribao_piliang_shanchu(id), shuaxinfn: () => this.shuaxinribaoliebiao(), tishi: '此操作不可撤销。' });
    }
    async piliang_xinzengrenwu() {
        if (!await aqqueren('批量添加任务', '确定为所有尚无任务的日报批量创建标签提取任务吗？', 'queren')) return;
        const jg = await this.luoji.renwu_piliang_xinzeng_quanbu();
        if (jg?.zhuangtaima === 200) {
            const shu = jg.shuju?.xinzengshu ?? 0;
            this.luoji.rizhi(`批量创建完成，新增 ${shu} 个任务`, 'ok');
            this.shuaxinribaoliebiao();
        }
    }
    async piliangshanchu_biaoqian() {
        await gj.piliangshanchu(this.luoji, { leibie: 'bq_pl_xz', mingcheng: '标签', shanchufn: id => this.luoji.biaoqian_piliang_shanchu(id), shuaxinfn: () => this.shuaxinbiaoqianliebiao() });
    }
    async piliangshanchu_leixing() {
        await gj.piliangshanchu(this.luoji, { leibie: 'lx_pl_xz', mingcheng: '类型', shanchufn: id => this.luoji.leixing_piliang_shanchu(id), shuaxinfn: () => this.shuaxinleixingliebiao(), tishi: '关联标签也会被删除。' });
    }
    async piliangshanchu_renwu() {
        await gj.piliangshanchu(this.luoji, { leibie: 'rw_pl_xz', mingcheng: '任务', shanchufn: id => this.luoji.renwu_piliang_shanchu(id), shuaxinfn: () => this.shuaxinrenwuliebiao() });
    }
}
