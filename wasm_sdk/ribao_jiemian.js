// 日报管理 - 界面层
import * as gj from './jiemian_gongju.js';
import { FenxiZhuangtai } from './fenxi_zhuangtai.js';
import { FenxiApiClient } from './ribao_luoji.js';
import * as fxr from './fenxi_xuanran.js';

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
        this.sousuoshijian = null;
        this._tupu_daohang_biaoqianid = null;
        this._bq_bianji_id = null;
        this._bq_xuanzhong_leixingid = null;
        // 分析视图状态与适配器
        this._fenxi_zt = new FenxiZhuangtai();
        this._fenxi_api = new FenxiApiClient(luoji);
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
               <button class="aq-btn aq-btn-zhu" onclick="ribao_qiehuanshitu('fenxi')">分析</button>
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
            ? ['ribao', 'biaoqian', 'leixing', 'renwu', 'tupu', 'fenxi']
            : ['ribao', 'quanburibao', 'tupu'];
        this.dangqianshitu = yunxushitu.includes(shitu) ? shitu : 'ribao';
        this.chakanquanbu = this.dangqianshitu === 'quanburibao';
        await this.xuanran();
        await this.shuaxindangqianshitu();
    }

    _youcunzai_shaixuan() {
        return this.sousuobiaoqianid || this.sousuoleixing || this.sousuoguanjiancizhi || this.sousuoyonghuid || this.sousuoshijian;
    }

    _qingkong_sousuozhuangtai() {
        this.sousuobiaoqianid = null;
        this.sousuoleixing = null;
        this.sousuoguanjiancizhi = null;
        this.sousuoyonghuid = null;
        this.sousuoshijian = null;
        this.dangqianyeshu = 1;
    }

    _goujian_sousuolan(xianshiquanxuan = false) {
        const shuruyang = 'height:36px;padding:0 12px;border:1px solid #E2E8F0;border-radius:6px;font-size:13px;box-sizing:border-box';
        const shijianyang = 'height:36px;padding:0 8px;border:1px solid #E2E8F0;border-radius:6px;font-size:13px;box-sizing:border-box';
        let html = '<div style="margin-bottom:16px">';
        html += '<div style="display:flex;gap:10px;align-items:center;flex-wrap:wrap">';
        html += '<button class="aq-btn aq-btn-lv" onclick="ribao_xinzengshitu()" style="height:36px">新增日报</button>';
        html += '<button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_ribao_piliangshanchu()" style="height:36px">批量删除</button>';
        html += '<button class="aq-btn aq-btn-xiao aq-btn-zhu" onclick="ribao_piliang_xinzengrenwu()" style="height:36px">批量添加任务</button>';
        if (xianshiquanxuan) {
            html += '<label style="display:flex;align-items:center;gap:4px;cursor:pointer;font-size:13px;color:#64748B;height:36px"><input type="checkbox" onchange="ribao_ribao_quanxuan(this)" style="width:16px;height:16px;cursor:pointer">全选</label>';
        }
        html += '<div style="height:20px;width:1px;background:#E2E8F0"></div>';
        html += `<input id="rb_gjc" type="text" value="${this.sousuoguanjiancizhi || ''}" onkeydown="ribao_sousuoshuru_huiche(event)" placeholder="搜索内容关键词" style="${shuruyang};width:160px">`;
        html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_sousuoguanjianci()" style="height:36px">搜索</button>';
        html += '<div style="height:20px;width:1px;background:#E2E8F0"></div>';
        html += `<input id="rb_yhid" type="text" placeholder="用户ID" style="${shuruyang};width:120px">`;
        html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_sousuoyonghuid_xuanze()" style="height:36px">查询</button>';
        html += '<div style="height:20px;width:1px;background:#E2E8F0"></div>';
        html += `<input id="rb_bqxz" type="text" placeholder="标签关键词筛选" style="${shuruyang};width:160px">`;
        html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_sousuobiaoqian_xuanze()" style="height:36px">筛选</button>';
        html += '</div>';
        html += '<div style="display:flex;gap:8px;align-items:center;flex-wrap:wrap;margin-top:8px">';
        html += '<span style="font-size:13px;color:#64748B">时间范围：</span>';
        html += `<input id="rb_sj_kaishi" type="datetime-local" style="${shijianyang}">`;
        html += '<span style="color:#94A3B8">~</span>';
        html += `<input id="rb_sj_jieshu" type="datetime-local" style="${shijianyang}">`;
        html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_sousuoshijian()" style="height:36px">时间搜索</button>';
        if (this._youcunzai_shaixuan()) {
            html += '<button class="aq-btn aq-btn-xiao" onclick="ribao_qingchusousuo()" style="height:36px">清除筛选</button>';
        }
        html += '</div></div>';
        return html;
    }

    async shuaxindangqianshitu() {
        const shitumap = {
            'ribao': () => this.shuaxinribaoliebiao(),
            'quanburibao': () => this.shuaxinribaoliebiao(),
            'biaoqian': () => this.shuaxinbiaoqianliebiao(),
            'leixing': () => this.shuaxinleixingliebiao(),
            'renwu': () => this.shuaxinrenwuliebiao(),
            'tupu': () => this.shuaxintupushitu(),
            'fenxi': () => this.shuaxinfenxishitu()
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
        } else if (this.sousuoshijian) {
            jg = await this.luoji.fabushijianchaxunfenye_shipei(this.sousuoshijian.kaishi, this.sousuoshijian.jieshu, this.dangqianyeshu, this.meiyetiaoshu);
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

        let html = this._goujian_sousuolan(false);

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
            liebiao.map(rb => this.luoji.guanlian_chaxun_ribaoid_daixinxi_shipei(rb.id))
        );
        for (let i = 0; i < liebiao.length; i++) {
            const gljg = guanlianjieguo[i];
            ribaobiaoqianmap[liebiao[i].id] = gljg?.zhuangtaima === 200 ? gljg.shuju || [] : [];
        }

        html = this._goujian_sousuolan(true);

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
                        ${rb.biaoti ? `<div style="font-size:15px;font-weight:600;color:#0F172A;margin-bottom:6px">${rb.biaoti}</div>` : ''}
                        <div style="font-size:12px;color:#64748B;margin-bottom:4px">ID: ${rb.id} | 发布者: ${rb.fabuzhemingcheng || rb.fabuzhezhanghao || rb.yonghuid}${rb.fabuzhezhanghao ? '（' + rb.fabuzhezhanghao + '）' : ''} | ${jiexishijian(rb.fabushijian)}</div>
                        ${neironghtml}
                    </div>
                    <div style="display:flex;gap:6px;margin-left:12px">
                        <button class="aq-btn aq-btn-xiao aq-btn-huang" onclick="ribao_bianji('${rb.id}')">编辑</button>
                        <button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_shanchu('${rb.id}')">删除</button>
                        ${this.shifouquanxian ? `<button class="aq-btn aq-btn-xiao" onclick="ribao_guanlianguanlian('${rb.id}')">管理标签</button>` : ''}
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
        this._bq_xuanzhong_leixingid = null;
        this._bq_bianji_id = null;
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = '<p style="color:#64748B">加载中...</p>';
        const leixingjg = await this.luoji.leixing_chaxun_quanbu();
        if (!leixingjg || leixingjg.zhuangtaima !== 200) {
            nr.innerHTML = '<p style="color:#EF4444">类型加载失败</p>';
            return;
        }
        const biaoqianjg = await this.luoji.biaoqian_chaxun_quanbu();
        if (!biaoqianjg || biaoqianjg.zhuangtaima !== 200) {
            nr.innerHTML = '<p style="color:#EF4444">标签加载失败</p>';
            return;
        }
        const leixinglie = leixingjg.shuju || [];
        const biaoqianlie = biaoqianjg.shuju || [];
        const leixingmap = Object.fromEntries(leixinglie.map(x => [x.id, x.mingcheng]));
        let html = '<div style="margin-bottom:12px;display:flex;gap:8px;align-items:center"><button class="aq-btn aq-btn-lv" onclick="ribao_xinzengbiaoqian()">新增标签</button><button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_biaoqian_piliangshanchu()">批量删除</button><label style="display:flex;align-items:center;gap:4px;cursor:pointer;font-size:13px;color:#64748B;height:36px"><input type="checkbox" onchange="ribao_biaoqian_quanxuan(this)" style="width:16px;height:16px;cursor:pointer">全选</label></div>';
        if (biaoqianlie.length === 0) {
            nr.innerHTML = html + '<p style="color:#64748B">暂无标签数据</p>';
            return;
        }
        html += '<div style="display:grid;gap:12px">';
        for (const bq of biaoqianlie) {
            const leixingming = leixingmap[bq.leixingid] || '未知类型';
            html += `<div style="padding:12px;background:#F8FAFC;border-radius:8px;display:flex;gap:10px;align-items:center">
                <input type="checkbox" class="bq_pl_xz" data-id="${bq.id}" style="width:16px;height:16px;cursor:pointer;flex-shrink:0">
                <div style="flex:1"><span style="font-size:14px;color:#1E293B">${bq.zhi}</span><span style="margin-left:8px;font-size:12px;color:#64748B">[${leixingming}]</span></div>
                <div style="display:flex;gap:8px;flex-shrink:0">
                    <button class="aq-btn aq-btn-xiao" onclick="ribao_bianjibiaoqian('${bq.id}')">编辑</button>
                    <button class="aq-btn aq-btn-xiao" onclick="ribao_shanchubiaoqian('${bq.id}')" style="background:#FEE2E2;color:#DC2626">删除</button>
                </div>
            </div>`;
        }
        html += '</div>';
        nr.innerHTML = html;
    }

    async shuaxinleixingliebiao() {
        this._bq_xuanzhong_leixingid = null;
        this._bq_bianji_id = null;
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = '<p style="color:#64748B">加载中...</p>';
        const jg = await this.luoji.leixing_chaxun_quanbu();
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = `<p style="color:#EF4444">加载失败: ${jg ? jg.xiaoxi : '请求错误'}</p>`;
            return;
        }
        const liebiao = jg.shuju || [];
        let html = '<div style="margin-bottom:12px;display:flex;gap:8px;align-items:center"><button class="aq-btn aq-btn-lv" onclick="ribao_xinzengleixing()">新增类型</button><button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_leixing_piliangshanchu()">批量删除</button><label style="display:flex;align-items:center;gap:4px;cursor:pointer;font-size:13px;color:#64748B;height:36px"><input type="checkbox" onchange="ribao_leixing_quanxuan(this)" style="width:16px;height:16px;cursor:pointer">全选</label></div>';
        if (liebiao.length === 0) {
            nr.innerHTML = html + '<p style="color:#64748B">暂无类型数据</p>';
            return;
        }
        html += '<div style="display:grid;gap:12px">';
        for (const lx of liebiao) {
            html += `<div style="padding:12px;background:#F8FAFC;border-radius:8px;display:flex;gap:10px;align-items:center">
                <input type="checkbox" class="lx_pl_xz" data-id="${lx.id}" style="width:16px;height:16px;cursor:pointer;flex-shrink:0">
                <span style="flex:1;font-size:14px;color:#1E293B">${lx.mingcheng}</span>
                <div style="display:flex;gap:8px;flex-shrink:0">
                    <button class="aq-btn aq-btn-xiao" onclick="ribao_bianjibiaoqian_leixing('${lx.id}')">标签</button>
                    <button class="aq-btn aq-btn-xiao" onclick="ribao_bianjileixing('${lx.id}')">编辑</button>
                    <button class="aq-btn aq-btn-xiao" onclick="ribao_shanchuleixing('${lx.id}')" style="background:#FEE2E2;color:#DC2626">删除</button>
                </div>
            </div>`;
        }
        html += '</div>';
        nr.innerHTML = html;
    }

    xuanranxinzengribao() {
        const nr = document.getElementById('ribao_neirong');
        const yonghuru = this.shifouquanxian
            ? '<div class="aq-hang"><label>用户ID</label><input id="rb_yonghuid" type="text" placeholder="用户ID"></div>'
            : '';
        nr.innerHTML = `<div class="aq-biaodan">
            ${yonghuru}
            <div class="aq-hang"><label>日报标题</label><input id="rb_biaoti" type="text" placeholder="请输入日报标题" style="border:1px solid #E2E8F0;border-radius:8px;padding:8px 12px;font-size:14px;outline:none;color:#1E293B;background:#fff;width:100%;box-sizing:border-box"></div>
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
            biaoti: hq('rb_biaoti'),
            neirong: hq('rb_neirong'),
            fabushijian: hq('rb_fabushijian')
        };
        if ((!this.shifouquanxian ? false : !shuju.yonghuid) || !shuju.biaoti || !shuju.neirong || !shuju.fabushijian) {
            this.luoji.rizhi('请填写所有必填字段', 'warn');
            return;
        }
        const jg = await this.luoji.ribao_xinzeng(shuju.yonghuid, shuju.biaoti, shuju.neirong, shuju.fabushijian);
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

    async xuanranxinzengbiaoqian() {
        const leixingjg = await this.luoji.leixing_chaxun_quanbu();
        if (!leixingjg || leixingjg.zhuangtaima !== 200 || !leixingjg.shuju || leixingjg.shuju.length === 0) {
            const nr = document.getElementById('ribao_neirong');
            nr.innerHTML = '<p style="color:#F59E0B">请先创建标签类型</p><button class="aq-btn" onclick="ribao_quxiao()">返回</button>';
            return;
        }
        const xuanxiang = leixingjg.shuju.map(x => `<option value="${x.id}">${this._bq_zhuanyi(x.mingcheng)}</option>`).join('');
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = `<div class="aq-biaodan">
            <div class="aq-hang"><label>标签类型</label>
                <select id="bq_leixingid" style="border:1px solid #E2E8F0;border-radius:8px;padding:8px 12px;font-size:14px;outline:none;color:#1E293B;background:#fff;cursor:pointer">
                    ${xuanxiang}
                </select>
            </div>
            <div class="aq-hang"><label>标签值</label><input id="bq_zhi" type="text" placeholder="输入标签值"></div>
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
            this._bq_xuanzhong_leixingid ? this.bianjibiaoqian_leixing(this._bq_xuanzhong_leixingid) : this.shuaxinbiaoqianliebiao();
        } else if (jg && jg.zhuangtaima === 403) {
            this.luoji.rizhi('权限不足：' + jg.xiaoxi, 'warn');
        }
    }

    async shanchubiaoqian(id) {
        if (!await aqqueren('删除标签', '确认删除此标签？')) return;
        const jg = await this.luoji.biaoqian_shanchu(id);
        if (jg && jg.zhuangtaima === 200) {
            this._bq_xuanzhong_leixingid ? this.bianjibiaoqian_leixing(this._bq_xuanzhong_leixingid) : this.shuaxinbiaoqianliebiao();
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
        const jg = await this.luoji.biaoqian_chaxun_id(id);
        if (!jg || jg.zhuangtaima !== 200) return;
        this._bq_bianji_id = id;
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = `<div class="aq-biaodan">
            <div class="aq-hang"><label>标签值</label><input id="bq_bianji_zhi" type="text" value="${this._bq_zhuanyi(jg.shuju.zhi)}"></div>
            <div style="margin-top:12px">
                <button class="aq-btn aq-btn-huang" onclick="ribao_tijiaobjbiaoqian()">保存</button>
                <button class="aq-btn" onclick="ribao_quxiao()">取消</button>
            </div></div>`;
    }

    async tijiaobjbiaoqian() {
        const zhi = document.getElementById('bq_bianji_zhi')?.value?.trim();
        if (!zhi || !this._bq_bianji_id) return;
        const jg = await this.luoji.biaoqian_gengxin(this._bq_bianji_id, zhi);
        if (jg && jg.zhuangtaima === 200) {
            this._bq_bianji_id = null;
            this._bq_xuanzhong_leixingid ? this.bianjibiaoqian_leixing(this._bq_xuanzhong_leixingid) : this.shuaxinbiaoqianliebiao();
        }
    }

    async bianjileixing(id) {
        const jg = await this.luoji.leixing_chaxun_id(id);
        if (!jg || jg.zhuangtaima !== 200) return;
        this._bq_bianji_id = id;
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = `<div class="aq-biaodan">
            <div class="aq-hang"><label>类型名称</label><input id="lx_bianji_mingcheng" type="text" value="${this._bq_zhuanyi(jg.shuju.mingcheng)}"></div>
            <div style="margin-top:12px">
                <button class="aq-btn aq-btn-huang" onclick="ribao_tijiaobjleixing()">保存</button>
                <button class="aq-btn" onclick="ribao_quxiao()">取消</button>
            </div></div>`;
    }

    async tijiaobjleixing() {
        const mingcheng = document.getElementById('lx_bianji_mingcheng')?.value?.trim();
        if (!mingcheng || !this._bq_bianji_id) return;
        const jg = await this.luoji.leixing_gengxin(this._bq_bianji_id, mingcheng);
        if (jg && jg.zhuangtaima === 200) {
            this._bq_bianji_id = null;
            this.shuaxinleixingliebiao();
        }
    }

    async bianjibiaoqian_leixing(leixingid) {
        this._bq_xuanzhong_leixingid = leixingid;
        this._bq_bianji_id = null;
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = '<p style="color:#64748B">加载中...</p>';
        const jg = await this.luoji.biaoqian_chaxun_leixingid(leixingid);
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = `<p style="color:#EF4444">${jg?.xiaoxi || '加载失败'}</p><button class="aq-btn" onclick="ribao_biaoqian_fanhui()">返回</button>`;
            return;
        }
        const lie = jg.shuju || [];
        let html = `<div style="margin-bottom:12px;display:flex;gap:8px;align-items:center">
            <button class="aq-btn aq-btn-lv" onclick="ribao_xinzengbiaoqian_leixing('${leixingid}')">新增标签</button>
            <button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_biaoqian_piliangshanchu()">批量删除</button>
            <label style="display:flex;align-items:center;gap:4px;cursor:pointer;font-size:13px;color:#64748B;height:36px"><input type="checkbox" onchange="ribao_biaoqian_quanxuan(this)" style="width:16px;height:16px;cursor:pointer">全选</label>
            <button class="aq-btn" onclick="ribao_biaoqian_fanhui()">返回</button>
        </div>`;
        if (lie.length === 0) {
            nr.innerHTML = html + '<p style="color:#94A3B8">暂无标签</p>';
            return;
        }
        html += '<div style="display:grid;gap:12px">';
        for (const bq of lie) {
            html += `<div style="padding:12px;background:#F8FAFC;border-radius:8px;display:flex;gap:10px;align-items:center">
                <input type="checkbox" class="bq_pl_xz" data-id="${bq.id}" style="width:16px;height:16px;cursor:pointer;flex-shrink:0">
                <span style="flex:1;font-size:14px;color:#1E293B">${bq.zhi}</span>
                <div style="display:flex;gap:8px;flex-shrink:0">
                    <button class="aq-btn aq-btn-xiao" onclick="ribao_bianjibiaoqian('${bq.id}')">编辑</button>
                    <button class="aq-btn aq-btn-xiao" onclick="ribao_shanchubiaoqian('${bq.id}')" style="background:#FEE2E2;color:#DC2626">删除</button>
                </div>
            </div>`;
        }
        html += '</div>';
        nr.innerHTML = html;
    }

    xinzengbiaoqian_leixing(leixingid) {
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = `<div class="aq-biaodan">
            <div class="aq-hang"><label>标签值</label><input id="bq_zhi" type="text" placeholder="输入标签值"></div>
            <div style="margin-top:12px">
                <button class="aq-btn aq-btn-lv" onclick="ribao_tijiaoxinzengbiaoqian_leixing('${leixingid}')">提交</button>
                <button class="aq-btn" onclick="ribao_quxiao()">取消</button>
            </div></div>`;
    }

    async tijiaoxinzengbiaoqian_leixing(leixingid) {
        const zhi = document.getElementById('bq_zhi')?.value?.trim();
        if (!zhi) return;
        const jg = await this.luoji.biaoqian_xinzeng(leixingid, zhi);
        if (jg && jg.zhuangtaima === 200) this.bianjibiaoqian_leixing(leixingid);
    }

    biaoqian_fanhui() {
        this._bq_xuanzhong_leixingid = null;
        this._bq_bianji_id = null;
        this.shuaxinleixingliebiao();
    }

    _bq_zhuanyi(s) {
        return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
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
        this._bq_bianji_id = null;
        if (this._bq_xuanzhong_leixingid && (this.dangqianshitu === 'leixing' || this.dangqianshitu === 'biaoqian')) {
            this.bianjibiaoqian_leixing(this._bq_xuanzhong_leixingid);
            return;
        }
        this._bq_xuanzhong_leixingid = null;
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

    _goujian_guanxileixing_yingshe(leixingpeizhi) {
        const yingshe = {};
        const morenzhuti = this._swdt_zhuti();
        const morenyanse = { zhu: morenzhuti[5].zhu, qian: morenzhuti[5].qian };
        for (const lx of leixingpeizhi) {
            const xinxi = { yanse: lx.yanse || morenyanse, fumian: !!lx.fumian };
            yingshe[lx.mingcheng] = xinxi;
            if (lx.biecheng) {
                for (const bc of lx.biecheng) yingshe[bc] = xinxi;
            }
        }
        return yingshe;
    }

    async _huoqu_guanxileixing_peizhi() {
        if (this._guanxileixing_huancun) return this._guanxileixing_huancun;
        try {
            const jg = await this.luoji.huoqu_guanxifenxi_leixing();
            if (jg?.zhuangtaima === 200 && Array.isArray(jg.shuju)) {
                this._guanxileixing_huancun = this._goujian_guanxileixing_yingshe(jg.shuju);
                return this._guanxileixing_huancun;
            }
        } catch (_) {}
        return null;
    }

    async guanxifenxi(id) {
        const rb = this._ribaoshujuhuancun?.[id];
        const kuozhan = this._jiexi_kuozhan(rb);
        const fenxi = this._huoqu_guanxifenxi(kuozhan);
        if (!fenxi || !fenxi.guanxi || fenxi.guanxi.length === 0) return this.luoji.rizhi('无关系分析数据', 'warn');
        const guanxilie = fenxi.guanxi;
        const lxyingshe = await this._huoqu_guanxileixing_peizhi() || {};
        const morenzhuti = this._swdt_zhuti();
        const morenyanse = { zhu: morenzhuti[5].zhu, qian: morenzhuti[5].qian };
        let liehtml = '';
        for (const gx of guanxilie) {
            const lxmc = gx.guanxi || '相关';
            const lxpz = lxyingshe[lxmc] || { yanse: morenyanse, fumian: false };
            const t = lxpz.yanse;
            const shifufumian = lxpz.fumian;
            const qinggan = gx.qinggan_qingxiang || '中性';
            const qgYanse = qinggan === '负面' ? { bg: '#FEF2F2', color: '#DC2626', icon: '⚠' } : qinggan === '正面' ? { bg: '#ECFDF5', color: '#059669', icon: '✓' } : { bg: '#F8FAFC', color: '#64748B', icon: '' };
            liehtml += `<div style="display:flex;align-items:center;gap:12px;padding:12px 16px;background:#fff;border:1px solid ${shifufumian ? '#FECACA' : '#E2E8F0'};border-radius:10px;${shifufumian ? 'border-left:3px solid ' + t.zhu : ''}">
                <div style="display:flex;align-items:center;gap:8px;flex:1">
                    <span style="padding:4px 10px;background:${t.qian};color:${t.zhu};border-radius:16px;font-size:12px;font-weight:600;white-space:nowrap">${gx.ren1 || ''}</span>
                    <span style="color:#94A3B8;font-size:18px">—</span>
                    <span style="padding:3px 8px;background:${shifufumian ? t.qian : '#F1F5F9'};color:${shifufumian ? t.zhu : '#475569'};border-radius:6px;font-size:11px;white-space:nowrap;font-weight:${shifufumian ? '700' : '400'}">${lxmc}</span>
                    <span style="color:#94A3B8;font-size:18px">—</span>
                    <span style="padding:4px 10px;background:${t.qian};color:${t.zhu};border-radius:16px;font-size:12px;font-weight:600;white-space:nowrap">${gx.ren2 || ''}</span>
                    ${qinggan !== '中性' ? `<span style="padding:2px 6px;background:${qgYanse.bg};color:${qgYanse.color};border-radius:4px;font-size:10px;white-space:nowrap;margin-left:4px">${qgYanse.icon} ${qinggan}</span>` : ''}
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
            { zhu: '#6366F1', qian: '#EEF2FF', bian: '#C7D2FE' },
            { zhu: '#06B6D4', qian: '#ECFEFF', bian: '#A5F3FC' },
            { zhu: '#10B981', qian: '#ECFDF5', bian: '#A7F3D0' },
            { zhu: '#F59E0B', qian: '#FFFBEB', bian: '#FDE68A' },
            { zhu: '#EC4899', qian: '#FDF2F8', bian: '#FBCFE8' },
            { zhu: '#8B5CF6', qian: '#F5F3FF', bian: '#DDD6FE' },
            { zhu: '#EF4444', qian: '#FEF2F2', bian: '#FECACA' },
            { zhu: '#14B8A6', qian: '#F0FDFA', bian: '#99F6E4' },
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
        this._qingkong_sousuozhuangtai();
        if (v) this.sousuoguanjiancizhi = v;
        this.shuaxinribaoliebiao();
    }

    sousuobiaoqian_xuanze() {
        const v = document.getElementById('rb_bqxz')?.value?.trim();
        if (!v) return this.luoji.rizhi('请输入标签关键词', 'warn');
        this.luoji.biaoqian_chaxun_quanbu().then(jg => {
            const liebiao = jg?.zhuangtaima === 200 ? jg.shuju || [] : [];
            const pipei = liebiao.find(bq => bq.zhi && bq.zhi.includes(v));
            this._qingkong_sousuozhuangtai();
            this.sousuobiaoqianid = pipei ? pipei.id : '-1';
            this.shuaxinribaoliebiao();
        });
    }

    sousuoyonghuid_xuanze() {
        const v = document.getElementById('rb_yhid')?.value?.trim();
        if (!v) return this.luoji.rizhi('请输入用户ID', 'warn');
        this._qingkong_sousuozhuangtai();
        this.sousuoyonghuid = v;
        this.shuaxinribaoliebiao();
    }

    sousuoshijianfanwei() {
        const kaishi = document.getElementById('rb_sj_kaishi')?.value?.trim();
        const jieshu = document.getElementById('rb_sj_jieshu')?.value?.trim();
        if (!kaishi || !jieshu) return this.luoji.rizhi('请选择完整的时间范围', 'warn');
        this._qingkong_sousuozhuangtai();
        this.sousuoshijian = { kaishi: new Date(kaishi).getTime().toString(), jieshu: new Date(jieshu).getTime().toString() };
        this.shuaxinribaoliebiao();
    }

    dianjibibaoqian(leixing, zhi) {
        this._qingkong_sousuozhuangtai();
        this.sousuoleixing = { mc: leixing, z: zhi };
        this.shuaxinribaoliebiao();
    }

    async tiaozhuan_tupu(ribaoid) {
        const gljg = await this.luoji.guanlian_chaxun_ribaoid_daixinxi_shipei(ribaoid);
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
        this._qingkong_sousuozhuangtai();
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
                    ${(rw.zhuangtai === 'false' || rw.zhuangtai === 'shibai') ? `<button class="aq-btn aq-btn-xiao aq-btn-lv" onclick="ribao_renwu_dange_chuli('${rw.id}')">处理</button>` : ''}
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

    async renwu_dange_chuli(id) {
        this.luoji.rizhi('正在处理任务[' + id + ']...', 'info');
        this.luoji.renwu_dange_chuli(id).then(jg => {
            if (jg) this.shuaxinrenwuliebiao();
        });
        await new Promise(r => setTimeout(r, 500));
        await this.shuaxinrenwuliebiao();
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
        const guanxi_secai = ['#8B5CF6', '#EC4899', '#F59E0B', '#06B6D4', '#10B981', '#EF4444', '#6366F1', '#14B8A6'];
        const _guanxi_secai_map = {};
        let _guanxi_secai_idx = 0;
        const guanxi_bian = guanxi_bianlie.filter(b => (String(b.yuan) in idmap) && (String(b.mubiao) in idmap)).map(b => {
            const gx = b.guanxi || '';
            if (gx && !(gx in _guanxi_secai_map)) _guanxi_secai_map[gx] = _guanxi_secai_idx++;
            return {
                yuan: idmap[String(b.yuan)], mubiao: idmap[String(b.mubiao)],
                guanxi: gx, miaoshu: b.miaoshu || '', cishu: parseInt(b.cishu) || 1,
                _secai: gx ? guanxi_secai[(_guanxi_secai_map[gx] || 0) % guanxi_secai.length] : guanxi_secai[0],
                _pianyi: 0
            };
        });

        // 多边偏移索引：同对节点多条关系边均匀展开，避免重叠
        const _goujian_duobian_suoyin = (bianlie) => {
            const suoyin = new Map();
            for (let i = 0; i < bianlie.length; i++) {
                const jian = Math.min(bianlie[i].yuan, bianlie[i].mubiao) + '_' + Math.max(bianlie[i].yuan, bianlie[i].mubiao);
                const zu = suoyin.get(jian) || [];
                zu.push(i);
                suoyin.set(jian, zu);
            }
            for (const [, zu] of suoyin) {
                const zongshu = zu.length;
                zu.forEach((idx, xuhao) => { bianlie[idx]._pianyi = (xuhao - (zongshu - 1) / 2) * 28; });
            }
        };
        _goujian_duobian_suoyin(guanxi_bian);
        bian.forEach(b => { b._pianyi = 0; });
        _goujian_duobian_suoyin(bian);

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
        gongju.style.cssText = 'position:absolute;top:12px;left:12px;display:flex;gap:2px;z-index:2;align-items:center;background:rgba(255,255,255,0.92);backdrop-filter:blur(20px);-webkit-backdrop-filter:blur(20px);border:1px solid rgba(226,232,240,0.45);border-radius:14px;padding:4px 5px;box-shadow:0 4px 24px rgba(0,0,0,0.06),0 1px 2px rgba(0,0,0,0.03)';
        if (zhongxinid) {
            const fanhui = _gj_anniu(_svg.fanhui + '<span style="margin-left:4px;font-weight:500">返回</span>', '返回全局视图');
            fanhui.style.width = 'auto';
            fanhui.style.padding = '0 10px';
            fanhui.onclick = () => this._tupu_jiazai(null);
            gongju.append(fanhui, _gj_fengefu());
        }
        const suoxiao = _gj_anniu(_svg.suoxiao, '缩小');
        suoxiao.onclick = () => { suofang = Math.max(0.2, suofang / 1.3); _xuyao_huizhi = true; };
        const suofangxianshi = document.createElement('span');
        suofangxianshi.style.cssText = 'font-size:11px;color:#94A3B8;min-width:38px;text-align:center;font-weight:600;font-variant-numeric:tabular-nums;user-select:none;letter-spacing:-0.3px';
        suofangxianshi.textContent = '100%';
        const fangda = _gj_anniu(_svg.fangda, '放大');
        fangda.onclick = () => { suofang = Math.min(5, suofang * 1.3); _xuyao_huizhi = true; };
        const chongzhi = _gj_anniu(_svg.chongzhi, '重置视图');
        chongzhi.onclick = () => { suofang = 1; pingyi_x = 0; pingyi_y = 0; _xuyao_huizhi = true; };
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
        let _tuli_guanxi_html = '';
        for (const [gx, ci] of Object.entries(_guanxi_secai_map)) {
            const sc = guanxi_secai[ci % guanxi_secai.length];
            _tuli_guanxi_html += `<span style="display:flex;align-items:center;gap:4px"><span style="width:16px;border-top:2px dashed ${sc};display:inline-block;flex-shrink:0"></span><span style="color:${sc};font-size:10px">${gx}</span></span>`;
        }
        tuli.innerHTML += '<span style="display:flex;align-items:center;gap:5px;flex-wrap:wrap;border-top:1px solid #E2E8F0;padding-top:6px;margin-top:2px;width:100%">' +
            '<span style="width:16px;border-top:2px solid #94A3B8;display:inline-block;flex-shrink:0"></span><span style="color:#64748B;font-size:10px">共现</span>' +
            _tuli_guanxi_html + '</span>';
        rongqi.appendChild(tuli);

        const xinxi = document.createElement('div');
        xinxi.id = 'tupu_xinxi';
        xinxi.style.cssText = 'position:absolute;bottom:14px;left:50%;transform:translateX(-50%);background:rgba(255,255,255,0.92);backdrop-filter:blur(16px);-webkit-backdrop-filter:blur(16px);border:1px solid rgba(226,232,240,0.5);border-radius:12px;padding:10px 18px;font-size:13px;color:#0F172A;z-index:2;display:none;pointer-events:none;box-shadow:0 4px 20px rgba(0,0,0,0.06);font-weight:500;max-width:560px;line-height:1.6;white-space:nowrap';
        rongqi.appendChild(xinxi);

        const celan = document.createElement('div');
        celan.id = 'tupu_celan';
        celan.style.cssText = 'position:absolute;right:0;top:0;width:360px;height:100%;background:rgba(255,255,255,0.88);backdrop-filter:blur(16px);-webkit-backdrop-filter:blur(16px);border-left:1px solid rgba(226,232,240,0.5);z-index:10;overflow-y:auto;display:none;box-shadow:-4px 0 24px rgba(0,0,0,0.06);transition:transform 200ms ease,opacity 200ms ease;transform:translateX(0)';
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
        const _bh_theta = 0.8;
        const MAX_SUDU = 18;

        const shijie_dao_pingmu = (wx, wy) => [
            (wx - shijiezhongxin_x) * suofang + kuan / 2 + pingyi_x,
            (wy - shijiezhongxin_y) * suofang + gao / 2 + pingyi_y
        ];
        const pingmu_dao_shijie = (sx, sy) => [
            (sx - kuan / 2 - pingyi_x) / suofang + shijiezhongxin_x,
            (sy - gao / 2 - pingyi_y) / suofang + shijiezhongxin_y
        ];

        // Barnes-Hut 四叉树：O(n log n) 排斥力 + 碰撞检测
        const _sishu_xinjian = (x1, y1, x2, y2) => ({
            x1, y1, x2, y2, zx: (x1 + x2) / 2, zy: (y1 + y2) / 2,
            zi: null, is_ye: true, idx: -1,
            sx: 0, sy: 0, sz: 0, jishu: 0, max_r: 0
        });
        const _sishu_charu = (gen, x, y, zhiliang, idx, banjing, shendu) => {
            if (shendu > 20) return;
            gen.sx += x * zhiliang; gen.sy += y * zhiliang; gen.sz += zhiliang;
            gen.jishu++; gen.max_r = Math.max(gen.max_r, banjing);
            if (gen.is_ye) {
                if (gen.idx < 0) { gen.idx = idx; return; }
                gen.is_ye = false;
                gen.zi = [null, null, null, null];
                const oi = gen.idx;
                gen.idx = -1;
                _sishu_charu_qi(gen, jiedian[oi].x, jiedian[oi].y, 1, oi, jiedian[oi].banjing, shendu + 1);
            }
            _sishu_charu_qi(gen, x, y, zhiliang, idx, banjing, shendu + 1);
        };
        const _sishu_charu_qi = (gen, x, y, zl, idx, r, sd) => {
            const qi = (y < gen.zy ? 0 : 2) + (x < gen.zx ? 0 : 1);
            if (!gen.zi[qi]) {
                gen.zi[qi] = _sishu_xinjian(
                    qi & 1 ? gen.zx : gen.x1, qi & 2 ? gen.zy : gen.y1,
                    qi & 1 ? gen.x2 : gen.zx, qi & 2 ? gen.y2 : gen.zy
                );
            }
            _sishu_charu(gen.zi[qi], x, y, zl, idx, r, sd);
        };
        const _sishu_paichu = (i, jx, jy, gen) => {
            if (!gen || gen.jishu === 0) return [0, 0];
            if (gen.is_ye) {
                if (gen.idx === i || gen.idx < 0) return [0, 0];
                const dx = jx - jiedian[gen.idx].x, dy = jy - jiedian[gen.idx].y;
                const d2 = dx * dx + dy * dy || 1;
                const li = paichichangshu / d2;
                const juli = Math.sqrt(d2);
                return [(dx / juli) * li, (dy / juli) * li];
            }
            const cx = gen.sx / gen.sz, cy = gen.sy / gen.sz;
            const dx = jx - cx, dy = jy - cy;
            const d2 = dx * dx + dy * dy || 1;
            const kuan2 = gen.x2 - gen.x1;
            if (kuan2 * kuan2 / d2 < _bh_theta * _bh_theta) {
                const li = paichichangshu * gen.jishu / d2;
                const juli = Math.sqrt(d2);
                return [(dx / juli) * li, (dy / juli) * li];
            }
            let fx = 0, fy = 0;
            for (let q = 0; q < 4; q++) {
                const [rfx, rfy] = _sishu_paichu(i, jx, jy, gen.zi[q]);
                fx += rfx; fy += rfy;
            }
            return [fx, fy];
        };
        const _sishu_pengzhuang = (i, gen) => {
            if (!gen || gen.jishu === 0) return;
            if (gen.is_ye) {
                if (gen.idx <= i || gen.idx < 0) return;
                const j = gen.idx;
                if (tuodong && (i === tuodong.idx || j === tuodong.idx)) return;
                const dx = jiedian[j].x - jiedian[i].x, dy = jiedian[j].y - jiedian[i].y;
                const dist = Math.sqrt(dx * dx + dy * dy) || 0.1;
                const minDist = (jiedian[i].banjing + jiedian[j].banjing) * 2.8;
                if (dist < minDist) {
                    const li = (minDist - dist) / dist * 0.25;
                    jiedian[i].x -= dx * li; jiedian[i].y -= dy * li;
                    jiedian[j].x += dx * li; jiedian[j].y += dy * li;
                }
                return;
            }
            const cx = gen.sx / gen.sz, cy = gen.sy / gen.sz;
            const dx = jiedian[i].x - cx, dy = jiedian[i].y - cy;
            const dist = Math.sqrt(dx * dx + dy * dy) || 0.1;
            const qieduan = (gen.x2 - gen.x1) + (jiedian[i].banjing + gen.max_r) * 3;
            if (dist > qieduan) return;
            for (let q = 0; q < 4; q++) _sishu_pengzhuang(i, gen.zi[q]);
        };

        const _tanhuangli = (bianlie_ref) => {
            for (const b of bianlie_ref) {
                const a = jiedian[b.yuan], c = jiedian[b.mubiao];
                const dx = c.x - a.x, dy = c.y - a.y;
                const juli = Math.sqrt(dx * dx + dy * dy) || 1;
                const li = (juli - lixiangchangdu) * tanhuangchangshu * _wl_wendu;
                const fx = (dx / juli) * li, fy = (dy / juli) * li;
                a.vx += fx; a.vy += fy;
                c.vx -= fx; c.vy -= fy;
            }
        };

        const gengxin = () => {
            if (_wl_tingzhi && !tuodong) return;
            _wl_wendu *= 0.99;
            if (_wl_wendu < 0.005) _wl_wendu = 0.005;
            // Barnes-Hut：构建四叉树 → 近似排斥力
            let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
            for (const j of jiedian) {
                if (j.x < minX) minX = j.x; if (j.y < minY) minY = j.y;
                if (j.x > maxX) maxX = j.x; if (j.y > maxY) maxY = j.y;
            }
            const fanwei = Math.max(maxX - minX, maxY - minY, 1) + 20;
            const gen = _sishu_xinjian(minX - 10, minY - 10, minX + fanwei, minY + fanwei);
            for (let i = 0; i < jiedian.length; i++) {
                _sishu_charu(gen, jiedian[i].x, jiedian[i].y, 1, i, jiedian[i].banjing, 0);
            }
            for (let i = 0; i < jiedian.length; i++) {
                const [fx, fy] = _sishu_paichu(i, jiedian[i].x, jiedian[i].y, gen);
                const gx = (shijiezhongxin_x - jiedian[i].x) * zhongxinli;
                const gy = (shijiezhongxin_y - jiedian[i].y) * zhongxinli;
                let nvx = (jiedian[i].vx + (fx + gx) * _wl_wendu) * mosun;
                let nvy = (jiedian[i].vy + (fy + gy) * _wl_wendu) * mosun;
                // 速度钳制：防止节点在初始密集布局下爆炸式弹射
                if (nvx > MAX_SUDU) nvx = MAX_SUDU; else if (nvx < -MAX_SUDU) nvx = -MAX_SUDU;
                if (nvy > MAX_SUDU) nvy = MAX_SUDU; else if (nvy < -MAX_SUDU) nvy = -MAX_SUDU;
                jiedian[i].vx = nvx;
                jiedian[i].vy = nvy;
            }
            _tanhuangli(bian);
            _tanhuangli(guanxi_bian);
            for (const j of jiedian) {
                if (tuodong && j === jiedian[tuodong.idx]) continue;
                j.x += j.vx; j.y += j.vy;
                if (Math.abs(j.vx) < 0.01) j.vx = 0;
                if (Math.abs(j.vy) < 0.01) j.vy = 0;
            }
            // 碰撞检测：复用四叉树空间查询
            _wl_pzjs++;
            if (_wl_pzjs % 3 === 0 && _wl_wendu > 0.03) {
                for (let i = 0; i < jiedian.length; i++) _sishu_pengzhuang(i, gen);
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

        // 视口外判断辅助
        const _shikou_wai = (ax, ay, cx, cy) =>
            (ax < -50 && cx < -50) || (ax > kuan + 50 && cx > kuan + 50) ||
            (ay < -50 && cy < -50) || (ay > gao + 50 && cy > gao + 50);

        // 曲线偏移计算（支持多边偏移）
        const _quxian_pianyi = (ax, ay, cx, cy, pianyi, jichu) => {
            const edx = cx - ax, edy = cy - ay;
            const elen = Math.sqrt(edx * edx + edy * edy) || 1;
            const curv = pianyi + Math.min(jichu, elen * 0.04);
            return {
                cpx: (ax + cx) / 2 + (-edy / elen) * curv,
                cpy: (ay + cy) / 2 + (edx / elen) * curv,
                edx, edy, elen, curv
            };
        };

        const huizhi = () => {
            ctx.clearRect(0, 0, kuan, gao);
            ctx.save();
            ctx.fillStyle = '#FAFBFC';
            ctx.fillRect(0, 0, kuan, gao);
            // --- 共现边：柔和实线 + 多边偏移 ---
            for (let bi = 0; bi < bian.length; bi++) {
                const b = bian[bi];
                const a = jiedian[b.yuan], c = jiedian[b.mubiao];
                const [ax, ay] = shijie_dao_pingmu(a.x, a.y);
                const [cx, cy] = shijie_dao_pingmu(c.x, c.y);
                const gaoliang = bi === xuanzhong_bian || xuanzhong === b.yuan || xuanzhong === b.mubiao;
                if (!gaoliang && _shikou_wai(ax, ay, cx, cy)) continue;
                const { cpx, cpy } = _quxian_pianyi(ax, ay, cx, cy, b._pianyi || 0, 20);
                ctx.beginPath();
                ctx.moveTo(ax, ay);
                ctx.quadraticCurveTo(cpx, cpy, cx, cy);
                ctx.shadowBlur = 0; ctx.shadowColor = 'transparent';
                if (gaoliang) {
                    if (bi === xuanzhong_bian) {
                        ctx.strokeStyle = '#6366F1';
                        ctx.lineWidth = Math.min(4, 1.2 + b.quanzhong * 0.6) * Math.min(suofang, 2);
                        ctx.shadowColor = 'rgba(99,102,241,0.25)'; ctx.shadowBlur = 8;
                    } else {
                        const xt = xuanzhong >= 0 ? zhuti[(leixingmap[jiedian[xuanzhong].leixing] || 0) % zhuti.length] : null;
                        ctx.strokeStyle = xt ? xt.zhu + '66' : 'rgba(100,116,139,0.45)';
                        ctx.lineWidth = Math.min(3, 1 + b.quanzhong * 0.4) * Math.min(suofang, 2);
                    }
                } else {
                    const t1 = zhuti[(leixingmap[jiedian[b.yuan].leixing] || 0) % zhuti.length];
                    const ha = Math.round(Math.min(100, 35 + b.quanzhong * 18)).toString(16).padStart(2, '0');
                    ctx.strokeStyle = t1.zhu + ha;
                    ctx.lineWidth = Math.min(2.5, 0.6 + b.quanzhong * 0.3) * Math.min(suofang, 2);
                }
                ctx.stroke();
                ctx.shadowBlur = 0; ctx.shadowColor = 'transparent';
            }
            // --- 关系边：虚线 + 按关系类型着色 + 多边偏移 + 标签 ---
            ctx.setLineDash([4, 3]);
            for (let gi = 0; gi < guanxi_bian.length; gi++) {
                const gb = guanxi_bian[gi];
                const a = jiedian[gb.yuan], c = jiedian[gb.mubiao];
                const [ax, ay] = shijie_dao_pingmu(a.x, a.y);
                const [cx, cy] = shijie_dao_pingmu(c.x, c.y);
                const gaoliang = gi === xuanzhong_guanxi_bian || xuanzhong === gb.yuan || xuanzhong === gb.mubiao;
                if (!gaoliang && _shikou_wai(ax, ay, cx, cy)) continue;
                const { cpx, cpy, edx, edy, elen, curv } = _quxian_pianyi(ax, ay, cx, cy, gb._pianyi, 24);
                const secai = gb._secai;
                ctx.beginPath();
                ctx.moveTo(ax, ay);
                ctx.quadraticCurveTo(cpx, cpy, cx, cy);
                ctx.shadowBlur = 0; ctx.shadowColor = 'transparent';
                if (gaoliang) {
                    if (gi === xuanzhong_guanxi_bian) {
                        ctx.strokeStyle = secai;
                        ctx.lineWidth = Math.min(4, 1.5 + gb.cishu * 0.3) * Math.min(suofang, 2);
                        ctx.shadowColor = secai + '40'; ctx.shadowBlur = 10;
                    } else {
                        ctx.strokeStyle = secai + '88';
                        ctx.lineWidth = Math.min(3, 1.2 + gb.cishu * 0.3) * Math.min(suofang, 2);
                    }
                } else {
                    const ha = Math.round(Math.min(130, 50 + gb.cishu * 22)).toString(16).padStart(2, '0');
                    ctx.strokeStyle = secai + ha;
                    ctx.lineWidth = Math.min(2.5, 1.2 + gb.cishu * 0.2) * Math.min(suofang, 2);
                }
                ctx.stroke();
                ctx.shadowBlur = 0; ctx.shadowColor = 'transparent';
                if (suofang >= 0.6 && gb.guanxi) {
                    const lx = (ax + cx) / 2 + (-edy / elen) * curv * 0.5;
                    const ly = (ay + cy) / 2 + (edx / elen) * curv * 0.5;
                    const lfs = Math.max(9, 10 * Math.min(suofang, 1.3));
                    ctx.font = `500 ${lfs}px -apple-system,"Microsoft YaHei",sans-serif`;
                    const tw = ctx.measureText(gb.guanxi).width;
                    const lpp = 6, lph = lfs + 6;
                    const rrx = lx - tw / 2 - lpp, rry = ly - lph / 2;
                    ctx.beginPath();
                    ctx.roundRect(rrx, rry, tw + lpp * 2, lph, lph / 2);
                    ctx.fillStyle = gaoliang ? secai + '18' : 'rgba(255,255,255,0.9)';
                    ctx.fill();
                    ctx.strokeStyle = gaoliang ? secai + '40' : 'rgba(226,232,240,0.5)';
                    ctx.lineWidth = 0.5;
                    ctx.stroke();
                    ctx.fillStyle = gaoliang ? secai : '#6B7280';
                    ctx.textAlign = 'center';
                    ctx.textBaseline = 'middle';
                    ctx.fillText(gb.guanxi, lx, ly);
                }
            }
            ctx.setLineDash([]);
            // --- 节点：现代扁平风格 + 微投影 ---
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
                // 投影层
                if (isXz) {
                    ctx.shadowColor = t.zhu + '50'; ctx.shadowBlur = 16; ctx.shadowOffsetY = 2;
                } else if (!_jianhua) {
                    ctx.shadowColor = 'rgba(0,0,0,0.08)'; ctx.shadowBlur = 6; ctx.shadowOffsetY = 2;
                }
                if (isXz || isLinji) {
                    ctx.beginPath();
                    ctx.arc(sx, sy, r * (isXz ? 1.5 : 1.3), 0, Math.PI * 2);
                    ctx.fillStyle = t.zhu + (isXz ? '18' : '0C');
                    ctx.fill();
                }
                ctx.beginPath();
                ctx.arc(sx, sy, r, 0, Math.PI * 2);
                if (_jianhua && !isXz && !isLinji) {
                    ctx.fillStyle = t.qian;
                } else {
                    const nG = ctx.createRadialGradient(sx, sy - r * 0.3, r * 0.1, sx, sy, r);
                    nG.addColorStop(0, '#FFFFFF');
                    nG.addColorStop(0.5, t.qian);
                    nG.addColorStop(1, t.bian);
                    ctx.fillStyle = nG;
                }
                ctx.fill();
                ctx.strokeStyle = t.zhu + (isXz ? 'DD' : isLinji ? 'AA' : '66');
                ctx.lineWidth = isXz ? 2.5 : isLinji ? 2 : 1.2;
                ctx.stroke();
                ctx.shadowBlur = 0; ctx.shadowColor = 'transparent'; ctx.shadowOffsetY = 0;
                // 标签
                let labelAlpha = 1;
                if (duoJiedian) {
                    labelAlpha = xuanzhong >= 0
                        ? (isXz ? 1 : isLinji ? 0.9 : 0.08)
                        : (_du[i] >= Math.max(2, _zuidade * 0.25) ? 0.95 : 0.12);
                }
                if (xianshiwenzi && r > 3 && labelAlpha > 0.08) {
                    const fs = Math.max(10, 12 * Math.min(suofang, 1.5));
                    ctx.font = `500 ${fs}px -apple-system,"Microsoft YaHei",sans-serif`;
                    const maxLW = Math.max(60, 140 * Math.min(suofang, 1.5));
                    let wenzi = j.zhi;
                    if (wenzi.length > 10) wenzi = wenzi.slice(0, 10);
                    const _tw0 = ctx.measureText(wenzi).width;
                    if (_tw0 > maxLW) wenzi = wenzi.slice(0, Math.max(1, Math.floor(wenzi.length * (maxLW - 8) / _tw0))) + '…';
                    const tw = ctx.measureText(wenzi).width;
                    const ppx = 7, pw = tw + ppx * 2, ph = fs + 6, prad = ph / 2;
                    const ly = sy + r + ph / 2 + 5;
                    const pl = sx - pw / 2, pt = ly - ph / 2;
                    ctx.beginPath();
                    ctx.roundRect(pl, pt, pw, ph, prad);
                    ctx.fillStyle = `rgba(255,255,255,${(0.92 * labelAlpha).toFixed(2)})`;
                    ctx.fill();
                    if (labelAlpha > 0.5) {
                        ctx.strokeStyle = `rgba(226,232,240,${(0.45 * labelAlpha).toFixed(2)})`;
                        ctx.lineWidth = 0.5;
                        ctx.stroke();
                    }
                    ctx.fillStyle = `rgba(15,23,42,${labelAlpha.toFixed(2)})`;
                    ctx.textAlign = 'center';
                    ctx.textBaseline = 'middle';
                    ctx.fillText(wenzi, sx, ly);
                }
            }
            ctx.restore();
        };

        // 按需重绘标记
        let _xuyao_huizhi = true;
        let _qianSfb = '';
        const xunhuan = () => {
            if (!donghua) return;
            const buju_huoyue = !_wl_tingzhi || !!tuodong;
            if (buju_huoyue) gengxin();
            if (buju_huoyue || _xuyao_huizhi) {
                huizhi();
                _xuyao_huizhi = false;
            }
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
                const gb = guanxi_bian[i];
                const a = jiedian[gb.yuan], c = jiedian[gb.mubiao];
                const dx = c.x - a.x, dy = c.y - a.y;
                const len2 = dx * dx + dy * dy;
                if (len2 === 0) continue;
                const elen = Math.sqrt(len2);
                const nx = -dy / elen, ny = dx / elen;
                const pianyi = gb._pianyi + Math.min(24, elen * 0.04);
                const midx = (a.x + c.x) / 2 + nx * pianyi;
                const midy = (a.y + c.y) / 2 + ny * pianyi;
                const caiyang = [0.2, 0.35, 0.5, 0.65, 0.8];
                for (const st of caiyang) {
                    const inv = 1 - st;
                    const px = inv * inv * a.x + 2 * inv * st * midx + st * st * c.x;
                    const py = inv * inv * a.y + 2 * inv * st * midy + st * st * c.y;
                    const dist = Math.sqrt((wx - px) * (wx - px) + (wy - py) * (wy - py));
                    if (dist < zuijinjuli) { zuijinjuli = dist; zuijin = i; }
                }
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
                if (_wl_tingzhi) { _wl_tingzhi = false; _wl_wendu = 0.08; for (const j of jiedian) { j.vx = 0; j.vy = 0; } }
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
            _xuyao_huizhi = true;
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
                    xinxi.innerHTML = `<b>${jiedian[gb.yuan].zhi}</b> — <span style="color:${gb._secai}">${gb.guanxi}</span> — <b>${jiedian[gb.mubiao].zhi}</b> (${gb.cishu}篇日报)`;
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
                    const _gx_lie = guanxi_bian
                        .filter(gb => gb.yuan === dianji_jiedian || gb.mubiao === dianji_jiedian)
                        .map(gb => ({
                            duifang: jiedian[gb.yuan === dianji_jiedian ? gb.mubiao : gb.yuan],
                            guanxi: gb.guanxi, miaoshu: gb.miaoshu, cishu: gb.cishu, secai: gb._secai
                        }));
                    this._tupu_xianshi_celan_jiedian(jiedian[dianji_jiedian], _gx_lie);
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
        canvas.onmouseleave = () => { tuodong = null; pingyi_tuodong = null; xuanzhong = -1; xinxi.style.display = 'none'; _xuyao_huizhi = true; };
        canvas.onwheel = e => {
            e.preventDefault();
            const yinzi = e.deltaY < 0 ? 1.15 : 1 / 1.15;
            suofang = Math.max(0.15, Math.min(6, suofang * yinzi));
            _xuyao_huizhi = true;
        };

        // 预模拟：隐形跑50帧物理迭代，跳过初始爆发阶段
        for (let _ps = 0; _ps < 50; _ps++) gengxin();
        for (const j of jiedian) { j.vx = 0; j.vy = 0; }
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

    _tupu_xianshi_celan_jiedian(j, gx_lie) {
        const celan = document.getElementById('tupu_celan');
        if (!celan) return;
        celan.style.display = 'block';
        const tubiao = '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="white" stroke-width="2" stroke-linecap="round"><circle cx="8" cy="8" r="4.5"/></svg>';
        let gx_html = '';
        if (gx_lie && gx_lie.length > 0) {
            gx_html += `<div style="display:flex;align-items:center;gap:6px;margin-bottom:10px">
                <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="#7C3AED" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><path d="M4 8h8"/><circle cx="3" cy="8" r="2"/><circle cx="13" cy="8" r="2"/></svg>
                <span style="font-size:13px;font-weight:600;color:#475569">AI \u5173\u7cfb</span>
                <span style="display:inline-flex;align-items:center;padding:1px 7px;background:#F5F3FF;border-radius:8px;font-size:11px;color:#7C3AED;font-weight:600">${gx_lie.length}</span>
            </div>`;
            for (const gx of gx_lie) {
                const _esc_yuan = j.id;
                const _esc_mb = gx.duifang.id;
                gx_html += `<div onclick="ribao_tupu_celan_chakanGuanxi('${_esc_yuan}','${_esc_mb}')" style="border:1px solid #EDE9FE;border-radius:8px;padding:10px 12px;margin-bottom:8px;cursor:pointer;transition:all 150ms;border-left:3px solid ${gx.secai || '#8B5CF6'}" onmouseenter="this.style.background='#F5F3FF';this.style.boxShadow='0 2px 8px rgba(139,92,246,0.08)'" onmouseleave="this.style.background='';this.style.boxShadow='none'">
                    <div style="display:flex;align-items:center;gap:6px;flex-wrap:wrap">
                        <span style="display:inline-flex;align-items:center;gap:3px;padding:2px 8px;background:rgba(139,92,246,0.08);color:${gx.secai || '#7C3AED'};border-radius:8px;font-size:11px;font-weight:600">
                            <svg width="10" height="10" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="2" y1="8" x2="14" y2="8"/></svg>
                            ${gx.guanxi || '\u76f8\u5173'}
                        </span>
                        <span style="font-size:12px;color:#0F172A;font-weight:500">${gx.duifang.zhi}</span>
                    </div>
                    <div style="display:flex;align-items:center;gap:8px;margin-top:6px;font-size:11px;color:#94A3B8">
                        <span>${gx.cishu} \u7bc7\u65e5\u62a5</span>
                        ${gx.miaoshu ? `<span style="color:#64748B">${gx.miaoshu.substring(0, 40)}${gx.miaoshu.length > 40 ? '...' : ''}</span>` : ''}
                    </div>
                </div>`;
            }
            gx_html += '<div style="height:8px"></div>';
        }
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
                    ${gx_html}
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

    tupu_celan_chakanGuanxi(yuan_id, mubiao_id) {
        this._tupu_celan_yeshu = 1;
        this._tupu_celan_biaoqianid = null;
        this._tupu_celan_yuan_id = yuan_id;
        this._tupu_celan_mubiao_id = mubiao_id;
        const celan = document.getElementById('tupu_celan');
        if (!celan) return;
        celan.style.display = 'block';
        const tubiao = '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="white" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M4 8h8"/><circle cx="3" cy="8" r="2"/><circle cx="13" cy="8" r="2"/></svg>';
        celan.innerHTML = `
            <div style="display:flex;flex-direction:column;height:100%">
                ${this._tupu_celan_tou_html(tubiao, 'AI \u5173\u7cfb\u65e5\u62a5', 'linear-gradient(135deg,#8B5CF6,#7C3AED)')}
                <div style="flex:1;overflow-y:auto;padding:16px">
                    <div id="tupu_celan_ribaolie"><p style="color:#94A3B8;font-size:13px">\u52a0\u8f7d\u4e2d...</p></div>
                </div>
            </div>`;
        this._tupu_celan_jiazai_bian_ribao(yuan_id, mubiao_id, 1);
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
        await gj.piliangshanchu(this.luoji, { leibie: 'bq_pl_xz', mingcheng: '标签', shanchufn: id => this.luoji.biaoqian_piliang_shanchu(id), shuaxinfn: () => this._bq_xuanzhong_leixingid ? this.bianjibiaoqian_leixing(this._bq_xuanzhong_leixingid) : this.shuaxinbiaoqianliebiao() });
    }
    async piliangshanchu_leixing() {
        await gj.piliangshanchu(this.luoji, { leibie: 'lx_pl_xz', mingcheng: '类型', shanchufn: id => this.luoji.leixing_piliang_shanchu(id), shuaxinfn: () => this.shuaxinleixingliebiao(), tishi: '关联标签也会被删除。' });
    }
    async piliangshanchu_renwu() {
        await gj.piliangshanchu(this.luoji, { leibie: 'rw_pl_xz', mingcheng: '任务', shanchufn: id => this.luoji.renwu_piliang_shanchu(id), shuaxinfn: () => this.shuaxinrenwuliebiao() });
    }

    // ========== 分析视图（状态→_fenxi_zt，渲染→fxr，API→_fenxi_api）==========

    async shuaxinfenxishitu() {
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = '<p style="color:#94A3B8;font-size:14px">加载分析配置...</p>';

        // 通过适配器获取配置
        const pzGui = await this._fenxi_api.huoqu_shiti_leixing();
        const shiti_leixinglie = pzGui.chenggong ? pzGui.shuju : [
            { mingcheng: '项目名称', biaoti: '项目', guanlianfenxi: true },
            { mingcheng: '客户公司', biaoti: '客户', guanlianfenxi: true },
        ];
        this._fenxi_zt.shezhiPeizhi(shiti_leixinglie);

        nr.innerHTML = '<p style="color:#94A3B8;font-size:14px">加载实体数据...</p>';

        // 并行请求所有类型的列表
        const jieguolie = await Promise.all(
            shiti_leixinglie.map(lx => this._fenxi_api.shiti_liebiao(lx.mingcheng))
        );

        // 存储每个类型的数据到状态层
        for (let i = 0; i < shiti_leixinglie.length; i++) {
            const gui = jieguolie[i];
            this._fenxi_zt.shezhiShujulie(shiti_leixinglie[i].mingcheng, gui.chenggong ? gui.shuju : []);
        }

        // 生成 placeholder
        const sousuotishi = this._fenxi_zt.huoquSousuoTishi();

        let html = `
<style>
    .fenxi-layout{display:flex;gap:16px;height:calc(100vh - 160px);min-height:560px}
    .fenxi-panel{background:#FFFFFF;border:1px solid #E2E8F0;border-radius:12px;box-shadow:0 6px 22px rgba(15,23,42,0.06)}
    .fenxi-sidebar{width:340px;flex-shrink:0;display:flex;flex-direction:column;gap:12px;overflow:hidden}
    .fenxi-sidebar .fenxi-panel{overflow:hidden;display:flex;flex-direction:column;max-height:100%}
    .fenxi-search{padding:12px;display:flex;gap:8px;align-items:center;flex-shrink:0}
    .fenxi-search input{flex:1;height:36px;padding:0 12px;border:1px solid #E2E8F0;border-radius:10px;font-size:13px;outline:none;background:#FFFFFF}
    .fenxi-search input:focus{border-color:#93C5FD;box-shadow:0 0 0 3px rgba(59,130,246,0.12)}
    .fenxi-panel-title{padding:10px 12px;border-top:1px solid #F1F5F9;font-size:12px;font-weight:700;color:#475569;letter-spacing:0.2px;flex-shrink:0}
    .fenxi-list{padding:10px;display:flex;flex-direction:column;gap:8px;max-height:220px;overflow:auto;flex-shrink:1}
    .fenxi-item{display:flex;align-items:center;gap:10px;padding:10px 12px;background:#FFFFFF;border:1px solid #E2E8F0;border-radius:10px;cursor:pointer;transition:all 150ms ease}
    .fenxi-item:hover{border-color:#93C5FD;box-shadow:0 10px 18px rgba(15,23,42,0.06);transform:translateY(-1px)}
    .fenxi-item-main{flex:1;min-width:0}
    .fenxi-item-title{font-size:14px;font-weight:600;color:#0F172A;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
    .fenxi-item-sub{font-size:12px;color:#94A3B8;margin-top:2px}
    .fenxi-checkbox{width:16px;height:16px;cursor:pointer;flex-shrink:0;accent-color:#3B82F6}
    .fenxi-sidebar-footer{padding:12px;border-top:1px solid #F1F5F9;flex-shrink:0}
    .fenxi-btn-block{width:100%}
    .fenxi-sidebar-scroll{flex:1;overflow:auto;min-height:0}

    .fenxi-main{flex:1;overflow:hidden;display:flex;flex-direction:column}
    .fenxi-main-inner{height:100%;overflow:auto;padding:16px;background:linear-gradient(180deg,#FFFFFF 0%,#F8FAFC 100%);border-radius:12px}

    .fenxi-title{font-size:16px;font-weight:800;color:#0F172A;margin:0 0 6px 0;letter-spacing:0.2px}
    .fenxi-sub{font-size:12px;color:#94A3B8;margin-bottom:14px}
    .fenxi-actions{display:flex;gap:8px;align-items:center;flex-wrap:wrap;margin-bottom:16px}
    .fenxi-status{font-size:12px;color:#64748B}

    .fenxi-section{margin-bottom:16px}
    .fenxi-section-h{font-size:12px;font-weight:700;color:#475569;margin-bottom:8px}
    .fenxi-tag{display:inline-flex;align-items:center;gap:6px;padding:4px 10px;background:#F1F5F9;border:1px solid #E2E8F0;border-radius:999px;font-size:12px;color:#475569}

    .fenxi-ribao-list{display:flex;flex-direction:column;gap:8px;max-height:420px;overflow:auto;padding:10px;background:#FFFFFF;border:1px solid #E2E8F0;border-radius:12px}
    .fenxi-ribao{border:1px solid #E2E8F0;border-radius:12px;overflow:hidden;background:#FFFFFF}
    .fenxi-ribao summary{list-style:none;padding:10px 12px;cursor:pointer;user-select:none;outline:none}
    .fenxi-ribao summary::-webkit-details-marker{display:none}
    .fenxi-ribao:hover{border-color:#CBD5E1}
    .fenxi-ribao-head{display:flex;gap:10px;align-items:flex-start;justify-content:space-between}
    .fenxi-ribao-meta{font-size:11px;color:#94A3B8;white-space:nowrap;flex-shrink:0}
    .fenxi-ribao-title{font-size:13px;font-weight:700;color:#0F172A;line-height:1.35;flex:1;min-width:0;word-break:break-word}
    .fenxi-ribao-zhaiyao{display:block;margin-top:6px;font-size:12px;color:#64748B;line-height:1.55;white-space:pre-wrap;word-break:break-word}
    .fenxi-ribao-body{padding:12px 12px;border-top:1px solid #F1F5F9;font-size:13px;color:#334155;line-height:1.7;white-space:pre-wrap;word-break:break-word;overflow-wrap:anywhere}

    .fenxi-ai-loading{display:flex;align-items:center;gap:10px;padding:12px;margin-bottom:10px;background:#FFFFFF;border:1px solid #E2E8F0;border-radius:12px}
    .fenxi-ai-error{padding:12px;margin-bottom:10px;background:#FEF2F2;border:1px solid #FECACA;border-radius:12px;color:#B91C1C;font-size:13px;line-height:1.6}

    .fenxi-ai-card{padding:14px;margin-bottom:12px;background:#FFFFFF;border:1px solid #E2E8F0;border-radius:12px;box-shadow:0 8px 18px rgba(15,23,42,0.04)}
    .fenxi-ai-card-tou{display:flex;align-items:center;justify-content:space-between;gap:10px;margin-bottom:10px}
    .fenxi-ai-card-h{display:flex;align-items:center;gap:10px;min-width:0}
    .fenxi-dot{width:10px;height:10px;border-radius:999px;flex-shrink:0}
    .fenxi-ai-title{font-size:14px;font-weight:800;color:#0F172A;letter-spacing:0.2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
    .fenxi-ai-body{font-size:13px;color:#334155;line-height:1.7;word-break:break-word;overflow-wrap:anywhere}

    .fenxi-kv{margin-bottom:8px}
    .fenxi-k{font-size:12px;font-weight:700;color:#475569;margin-bottom:4px}
    .fenxi-v{font-size:13px;color:#334155;line-height:1.7;white-space:pre-wrap;word-break:break-word;overflow-wrap:anywhere}

    .fenxi-badge{display:inline-flex;align-items:center;padding:2px 8px;border-radius:999px;font-size:11px;font-weight:800;white-space:nowrap}
    .fenxi-badge-gao{background:#FEF2F2;color:#B91C1C;border:1px solid #FECACA}
    .fenxi-badge-zhong{background:#FFFBEB;color:#92400E;border:1px solid #FDE68A}
    .fenxi-badge-di{background:#F0FDF4;color:#166534;border:1px solid #BBF7D0}

    .fenxi-issue{display:flex;gap:10px;align-items:flex-start;padding:10px 12px;border:1px solid #E2E8F0;border-radius:12px;background:#FFFFFF;margin-bottom:8px}
    .fenxi-issue-text{flex:1;min-width:0;font-size:13px;color:#0F172A;line-height:1.6;white-space:pre-wrap;word-break:break-word;overflow-wrap:anywhere}

    @media (max-width: 980px){
        .fenxi-layout{flex-direction:column;height:auto;min-height:0}
        .fenxi-sidebar{width:100%}
        .fenxi-list{max-height:220px}
        .fenxi-ribao-list{max-height:320px}
    }
</style>
<div class="fenxi-layout">
    <div class="fenxi-sidebar">
        <div class="fenxi-panel">
            <div class="fenxi-search">
                <input id="fenxi_guolv" type="text" placeholder="搜索${sousuotishi}..." oninput="ribao_fenxi_guolv()">
            </div>
            <div class="fenxi-sidebar-scroll">`;

        // 动态生成每个类型的列表区域
        for (const lx of shiti_leixinglie) {
            const liebiao = this._fenxi_zt.huoquShujulie(lx.mingcheng);
            html += `<div class="fenxi-panel-title">${lx.biaoti}列表（${liebiao.length}）</div>`;
            html += `<div class="fenxi-list">${fxr.xuanran_fenxi_liebiao(liebiao, lx.mingcheng, !!lx.guanlianfenxi)}</div>`;
        }

        html += `</div>`; // fenxi-sidebar-scroll

        // 综合关联分析按钮（跨类型，收集所有勾选项）
        const guanlianLeixinglie = this._fenxi_zt.huoquGuanlianLeixinglie();
        if (guanlianLeixinglie.length > 0) {
            html += `<div class="fenxi-sidebar-footer">`;
            html += `<button class="aq-btn aq-btn-zhu fenxi-btn-block" onclick="ribao_fenxi_zonghe_guanlian()">综合关联分析（跨类型勾选至少2个）</button>`;
            html += `</div>`;
        }

        html += `
        </div>
    </div>
    <div class="fenxi-panel fenxi-main">
        <div id="fenxi_jieguo_qu" class="fenxi-main-inner">
            <div class="fenxi-title">分析</div>
            <div class="fenxi-sub">点击左侧实体，查看关联日报，并按维度持续生成分析结果。</div>
        </div>
    </div>
</div>`;
        nr.innerHTML = html;
    }

    _xuanran_fenxi_liebiao(liebiao, leixing, guanlianfenxi = false) {
        return fxr.xuanran_fenxi_liebiao(liebiao, leixing, guanlianfenxi);
    }

    fenxi_guolv() {
        const guanjianci = (document.getElementById('fenxi_guolv')?.value || '').trim().toLowerCase();
        const xiang = document.querySelectorAll('.fenxi_liebiao_xiang');
        for (const el of xiang) {
            const mc = (el.dataset.mingcheng || '').toLowerCase();
            el.style.display = (!guanjianci || mc.includes(guanjianci)) ? 'flex' : 'none';
        }
    }

    fenxi_xiangmu_gouxuan(cb) {
        this._fenxi_zt.gouxuanShiti('项目名称', cb.dataset.mingcheng, cb.checked);
    }

    fenxi_shiti_gouxuan(cb) {
        this._fenxi_zt.gouxuanShiti(cb.dataset.leixing, cb.dataset.mingcheng, cb.checked);
    }

    async fenxi_jiaoliu(shiti_leixing, shiti_mingcheng) {
        const jieguo_qu = document.getElementById('fenxi_jieguo_qu');
        if (!jieguo_qu) return;
        this._fenxi_zt.shezhiDangqianShiti(shiti_leixing, shiti_mingcheng);
        jieguo_qu.innerHTML = `<div style="display:flex;align-items:center;gap:8px"><div class="aq-xuanzhuan"></div><span style="color:#64748B;font-size:14px">正在加载「${shiti_mingcheng}」的日报数据...</span></div>`;
        const gui = await this._fenxi_api.shiti_ribao(shiti_leixing, shiti_mingcheng);
        if (!gui.chenggong) {
            jieguo_qu.innerHTML = `<p style="color:#EF4444">加载失败: ${gui.xiaoxi}</p>`;
            return;
        }
        const { ribaolie, biaoqianlie } = gui.shuju;
        jieguo_qu.innerHTML = fxr.xuanran_shiti_xiangqing(shiti_mingcheng, shiti_leixing, ribaolie, biaoqianlie);
    }

    async fenxi_kaishi_fenxi() {
        const shiti = this._fenxi_zt.dangqian_shiti;
        if (!shiti) return;
        const { leixing, mingcheng } = shiti;
        // 读取用户勾选的维度
        const weidu_liebiao = [];
        const xzlie = document.querySelectorAll('.fenxi_weidu_xz:checked');
        for (const cb of xzlie) {
            if (cb.value) weidu_liebiao.push(cb.value);
        }
        if (weidu_liebiao.length === 0) {
            this.luoji.rizhi('请至少选择一个分析维度', 'warn');
            return;
        }
        this._fenxi_zt.kaishiFenxi();
        const kaishi_btn = document.getElementById('fenxi_kaishi_btn');
        const tingzhi_btn = document.getElementById('fenxi_tingzhi_btn');
        const zhuangtai_el = document.getElementById('fenxi_zhuangtai');
        const ai_qu = document.getElementById('fenxi_ai_jieguo');
        if (!ai_qu) return;
        if (kaishi_btn) kaishi_btn.style.display = 'none';
        if (tingzhi_btn) tingzhi_btn.style.display = 'inline-block';
        ai_qu.innerHTML = '';

        let shifoutingzhi = false;
        for (let i = 0; i < weidu_liebiao.length; i++) {
            if (!this._fenxi_zt.yunxingzhong) { shifoutingzhi = true; break; }
            const weidu = weidu_liebiao[i];
            if (zhuangtai_el) zhuangtai_el.textContent = `正在分析：${weidu}（${i + 1}/${weidu_liebiao.length}）`;
            const zhanyong_id = 'fenxi_zhanyong_' + i;
            ai_qu.insertAdjacentHTML('beforeend', `<div id="${zhanyong_id}" class="fenxi-ai-loading"><div class="aq-xuanzhuan"></div><span style="font-size:13px;color:#475569">正在分析：${weidu}</span></div>`);
            try {
                const gui = await this._fenxi_api.ai_shendu(leixing, mingcheng, weidu);
                const zhanyong = document.getElementById(zhanyong_id);
                if (!zhanyong) break;
                if (gui.chenggong && gui.shuju?.ai_fenxi) {
                    zhanyong.outerHTML = fxr.xuanran_shendu_kapian(weidu, gui.shuju.ai_fenxi);
                } else {
                    zhanyong.outerHTML = `<div class="fenxi-ai-error">${weidu} 分析失败：${gui.xiaoxi}</div>`;
                }
            } catch(e) {
                const zhanyong = document.getElementById(zhanyong_id);
                if (zhanyong) zhanyong.outerHTML = `<div class="fenxi-ai-error">${weidu} 分析异常</div>`;
            }
        }

        this._fenxi_zt.tingzhiFenxi();
        if (kaishi_btn) kaishi_btn.style.display = 'inline-block';
        if (tingzhi_btn) tingzhi_btn.style.display = 'none';
        if (zhuangtai_el) zhuangtai_el.textContent = shifoutingzhi ? '已停止分析' : '分析完成';
    }

    fenxi_tianjia_weidu() {
        const shuru = document.getElementById('fenxi_zidingyi_weidu');
        if (!shuru) return;
        const zhi = shuru.value.trim();
        if (!zhi) return;
        const qu = document.getElementById('fenxi_weidu_qu');
        if (!qu) return;
        // 检查是否已存在
        const yicunzai = Array.from(document.querySelectorAll('.fenxi_weidu_xz')).some(cb => cb.value === zhi);
        if (yicunzai) {
            this.luoji.rizhi('该维度已存在', 'warn');
            return;
        }
        const label = document.createElement('label');
        label.style.cssText = 'display:inline-flex;align-items:center;gap:5px;padding:5px 12px;background:#F0FDF4;border:1px solid #BBF7D0;border-radius:999px;cursor:pointer;font-size:12px;color:#166534;transition:all 150ms;user-select:none';
        label.innerHTML = `<input type="checkbox" class="fenxi_weidu_xz" value="${zhi.replace(/"/g, '&quot;')}" checked style="width:14px;height:14px;accent-color:#16A34A;cursor:pointer"><span style="display:inline-block;width:8px;height:8px;border-radius:50%;background:#16A34A;flex-shrink:0"></span>${zhi}<span onclick="this.parentElement.remove();event.stopPropagation()" style="margin-left:2px;color:#94A3B8;cursor:pointer;font-size:14px;line-height:1">×</span>`;
        qu.appendChild(label);
        shuru.value = '';
    }

    fenxi_quanxuan_weidu(xuanzhong) {
        const cblie = document.querySelectorAll('.fenxi_weidu_xz');
        for (const cb of cblie) cb.checked = xuanzhong;
    }

    fenxi_tingzhi_fenxi() {
        this._fenxi_zt.tingzhiFenxi();
        const kaishi_btn = document.getElementById('fenxi_kaishi_btn');
        const tingzhi_btn = document.getElementById('fenxi_tingzhi_btn');
        const zhuangtai_el = document.getElementById('fenxi_zhuangtai');
        if (kaishi_btn) kaishi_btn.style.display = 'inline-block';
        if (tingzhi_btn) tingzhi_btn.style.display = 'none';
        if (zhuangtai_el) zhuangtai_el.textContent = '已停止分析';
    }

    _xuanran_shendu_fenxi_kapian(weidu, fenxi) {
        return fxr.xuanran_shendu_kapian(weidu, fenxi);
    }

    _xuanran_fenxi_jieguo_shipei(fenxi) {
        return fxr.xuanran_fenxi_jieguo_shipei(fenxi);
    }

    _xuanran_tongyong_json(obj) {
        return fxr.xuanran_tongyong_json(obj);
    }

    _xuanran_jiaoliu_fenxi_jieguo(fenxi) {
        return fxr.xuanran_jiaoliu_fenxi_jieguo(fenxi);
    }

    fenxi_shiti_guanlian(leixingmingcheng) {
        const xuanzhong = this._fenxi_zt.huoquXuanzhonglie(leixingmingcheng);
        if (xuanzhong.length < 2) {
            this.luoji.rizhi('请至少勾选2个进行关联分析', 'warn');
            return;
        }
        const lx_peizhi = this._fenxi_zt.chazhaoLeixingPeizhi(leixingmingcheng);
        const biaoti = lx_peizhi?.biaoti || leixingmingcheng;
        this._guanlian_huancun = { leixing: 'shiti', leixingmingcheng, xuanzhong, biaoti };
        const jieguo_qu = document.getElementById('fenxi_jieguo_qu');
        if (!jieguo_qu) return;
        let html = `<div class="fenxi-title">${biaoti}关联分析</div>`;
        html += `<div class="fenxi-sub">已选：${xuanzhong.join(' / ')}</div>`;
        html += `<div style="margin:16px 0">`;
        html += `<div style="font-size:12px;font-weight:700;color:#475569;margin-bottom:8px">你想让 AI 重点分析什么？（可选）</div>`;
        html += `<textarea id="fenxi_guanlian_tishi" rows="3" placeholder="例如：分析这些${biaoti}之间的关联关系、资源冲突、协作问题...\n留空则使用默认全面分析" style="width:100%;padding:10px 12px;border:1px solid #E2E8F0;border-radius:10px;font-size:13px;outline:none;resize:vertical;box-sizing:border-box;font-family:inherit"></textarea>`;
        html += `</div>`;
        html += `<button class="aq-btn aq-btn-zhu" onclick="ribao_fenxi_guanlian_kaishi()" style="margin-right:8px">开始深度分析</button>`;
        html += `<span style="font-size:12px;color:#94A3B8">将基于日报原文进行深度关联分析</span>`;
        jieguo_qu.innerHTML = html;
    }

    fenxi_zonghe_guanlian() {
        const suoyou = this._fenxi_zt.huoquSuoyouXuanzhong();
        if (suoyou.length < 2) {
            this.luoji.rizhi('请跨类型勾选至少2个实体进行关联分析', 'warn');
            return;
        }
        this._guanlian_huancun = { leixing: 'zonghe', suoyou };
        const jieguo_qu = document.getElementById('fenxi_jieguo_qu');
        if (!jieguo_qu) return;
        const miaoshu = suoyou.map(s => `${s.leixing}:${s.zhi}`).join(' / ');
        let html = `<div class="fenxi-title">综合关联分析</div>`;
        html += `<div class="fenxi-sub">已选实体：${miaoshu}</div>`;
        html += `<div style="margin:16px 0">`;
        html += `<div style="font-size:12px;font-weight:700;color:#475569;margin-bottom:8px">你想让 AI 重点分析什么？（可选）</div>`;
        html += `<textarea id="fenxi_guanlian_tishi" rows="3" placeholder="例如：分析这些人员和项目之间的协作问题、风险点、资源冲突...\n留空则使用默认全面分析" style="width:100%;padding:10px 12px;border:1px solid #E2E8F0;border-radius:10px;font-size:13px;outline:none;resize:vertical;box-sizing:border-box;font-family:inherit"></textarea>`;
        html += `</div>`;
        html += `<button class="aq-btn aq-btn-zhu" onclick="ribao_fenxi_guanlian_kaishi()" style="margin-right:8px">开始深度分析</button>`;
        html += `<span style="font-size:12px;color:#94A3B8">将基于日报原文进行深度关联分析</span>`;
        jieguo_qu.innerHTML = html;
    }

    async fenxi_guanlian_kaishi() {
        const huancun = this._guanlian_huancun;
        if (!huancun) return;
        const tishi = (document.getElementById('fenxi_guanlian_tishi')?.value || '').trim();
        const jieguo_qu = document.getElementById('fenxi_jieguo_qu');
        if (!jieguo_qu) return;

        if (huancun.leixing === 'zonghe') {
            const suoyou = huancun.suoyou;
            const miaoshu = suoyou.map(s => `${s.leixing}:${s.zhi}`).join(' / ');
            jieguo_qu.innerHTML = `<div style="display:flex;align-items:center;gap:8px"><div class="aq-xuanzhuan"></div><span style="color:#64748B;font-size:14px">正在深度分析 ${suoyou.length} 个实体的关联关系，请稍候...</span></div>`;
            const gui = await this._fenxi_api.zonghe_guanlian(suoyou, tishi);
            if (!gui.chenggong) { jieguo_qu.innerHTML = `<p style="color:#EF4444">综合关联分析失败: ${gui.xiaoxi}</p>`; return; }
            const { xiangmu_shuju, ai_fenxi } = gui.shuju;
            let html = `<div class="fenxi-title">综合关联分析</div><div class="fenxi-sub">${miaoshu}</div>`;
            for (const xm of xiangmu_shuju) {
                html += this._xuanran_guanlian_shiti_ka(xm);
            }
            if (ai_fenxi) html += fxr.xuanran_fenxi_jieguo_shipei(ai_fenxi);
            jieguo_qu.innerHTML = html;
        } else {
            const { leixingmingcheng, xuanzhong, biaoti } = huancun;
            jieguo_qu.innerHTML = `<div style="display:flex;align-items:center;gap:8px"><div class="aq-xuanzhuan"></div><span style="color:#64748B;font-size:14px">正在深度分析 ${xuanzhong.length} 个${biaoti}的关联关系，请稍候...</span></div>`;
            const gui = await this._fenxi_api.shiti_guanlian(leixingmingcheng, xuanzhong, tishi);
            if (!gui.chenggong) { jieguo_qu.innerHTML = `<p style="color:#EF4444">关联分析失败: ${gui.xiaoxi}</p>`; return; }
            const { xiangmu_shuju, ai_fenxi } = gui.shuju;
            let html = `<div class="fenxi-title">${biaoti}关联分析</div><div class="fenxi-sub">${biaoti}：${xuanzhong.join(' / ')}</div>`;
            for (const xm of xiangmu_shuju) {
                html += this._xuanran_guanlian_shiti_ka(xm);
            }
            if (ai_fenxi) html += fxr.xuanran_fenxi_jieguo_shipei(ai_fenxi);
            jieguo_qu.innerHTML = html;
        }
    }

    _xuanran_guanlian_shiti_ka(xm) {
        let html = `<div style="margin-bottom:12px;padding:10px 12px;background:#FFF;border:1px solid #E2E8F0;border-radius:6px">`;
        html += `<div style="font-size:14px;font-weight:600;color:#0F172A;margin-bottom:6px">${xm.xiangmu_mingcheng}</div>`;
        if (xm.biaoqianlie && xm.biaoqianlie.length > 0) {
            html += '<div style="display:flex;flex-wrap:wrap;gap:4px">';
            for (const bq of xm.biaoqianlie) {
                html += `<span style="display:inline-block;padding:2px 8px;background:#F1F5F9;border-radius:10px;font-size:11px;color:#475569">${bq.leixingmingcheng}：${bq.zhi}</span>`;
            }
            html += '</div>';
        } else {
            html += '<span style="font-size:12px;color:#94A3B8">暂无关联标签</span>';
        }
        html += '</div>';
        return html;
    }

    _xuanran_xiangmu_guanlian_jieguo(fenxi) {
        return fxr.xuanran_xiangmu_guanlian_jieguo(fenxi);
    }
}
