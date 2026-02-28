// 日报管理 - 界面层
import * as gj from './jiemian_gongju.js';
import { tiqushuju, tiqufenyeshuju, zhuangtai_html, chuli_api_jieguo, tongjigaopin, huoqushitibiaoqian } from './jiemian_gongju.js';
import { FenxiZhuangtai } from './fenxi_zhuangtai.js';
import { FenxiApiClient } from './ribao_luoji.js';
import * as fxr from './fenxi_xuanran.js';
import { TupuXuanran } from './tupu_xuanran.js';

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
        this._tupu_daohang_mubiao = null;
        this._bq_bianji_id = null;
        this._bq_xuanzhong_leixingid = null;
        // 图谱渲染器
        this._tupu_xuanranqi = new TupuXuanran();
        // 分析视图状态与适配器
        this._fenxi_zt = new FenxiZhuangtai();
        this._fenxi_api = new FenxiApiClient(luoji);
        // 建档视图状态（前端真实实现区域）
        this._jiandang_fenbiao = 'gongsi';
        this._jiandang_liebiao_huancun = {};
        this._jiandang_xiangqing = null;
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
               <button class="aq-btn aq-btn-zhu" onclick="ribao_qiehuanshitu('jiandang')">建档</button>
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
            ? ['ribao', 'biaoqian', 'leixing', 'renwu', 'jiandang', 'tupu', 'fenxi']
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
            'jiandang': () => this.shuaxinjiandangshitu(),
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
            liebiao = tiqushuju(jg, []);
            zongshu = liebiao.length;
        } else if (this.sousuoleixing) {
            jg = await this.luoji.guanlian_chaxun_leixingmingcheng_zhi(this.sousuoleixing.mc, this.sousuoleixing.z);
            liebiao = tiqushuju(jg, []);
            zongshu = liebiao.length;
        } else if (this.sousuoguanjiancizhi) {
            jg = await this.luoji.guanjiancichaxunfenye_shipei(this.sousuoguanjiancizhi, this.dangqianyeshu, this.meiyetiaoshu);
            ({ liebiao, zongshu } = tiqufenyeshuju(jg));
        } else if (this.sousuoyonghuid) {
            jg = await this.luoji.ribao_chaxun_yonghuid_fenye(this.sousuoyonghuid, this.dangqianyeshu, this.meiyetiaoshu);
            ({ liebiao, zongshu } = tiqufenyeshuju(jg));
        } else if (this.sousuoshijian) {
            jg = await this.luoji.fabushijianchaxunfenye_shipei(this.sousuoshijian.kaishi, this.sousuoshijian.jieshu, this.dangqianyeshu, this.meiyetiaoshu);
            ({ liebiao, zongshu } = tiqufenyeshuju(jg));
        } else {
            jg = await this.luoji.chaxunfenye_shipei(this.dangqianyeshu, this.meiyetiaoshu, this.chakanquanbu);
            ({ liebiao, zongshu } = tiqufenyeshuju(jg));
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
                neironghtml += `<span id="rb_zk_${rb.id}" onclick="ribao_qiehuanneirong('${rb.id}')" style="color:#3B82F6;font-size:12px;cursor:pointer;user-select:none;display:inline-block;margin-top:4px" onmouseenter="this.style.color='#2563EB'" onmouseleave="this.style.color='#3B82F6'">查看完整内容 ▼</span>`;
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
                        <button class="aq-btn aq-btn-xiao aq-btn-zhu" onclick="ribao_chakanribao('${rb.id}')">查看</button>
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
                    html += `<span style="display:inline-flex;align-items:center;gap:4px;padding:4px 10px;background:#EFF6FF;color:#1E40AF;border-radius:16px;font-size:12px;transition:background 200ms" onmouseenter="this.style.background='#DBEAFE'" onmouseleave="this.style.background='#EFF6FF'">
                        <span onclick="ribao_dianjibibaoqian('${leixing}','${zhi}')" style="cursor:pointer;display:inline-flex;align-items:center;gap:4px"><span style="color:#64748B">${leixing}:</span>${zhi}</span>
                        <span onclick="ribao_biaoqian_tiaozhuan_tupu('${bq.biaoqianid}')" title="在图谱中查看" style="cursor:pointer;color:#0369A1;font-size:13px;line-height:1;opacity:0.4;transition:opacity 150ms" onmouseenter="this.style.opacity='1'" onmouseleave="this.style.opacity='0.4'">◎</span>
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
        nr.innerHTML = zhuangtai_html('jiazai', '加载中...');
        const leixingjg = await this.luoji.leixing_chaxun_quanbu();
        if (!leixingjg || leixingjg.zhuangtaima !== 200) {
            nr.innerHTML = zhuangtai_html('cuowu', '类型加载失败');
            return;
        }
        const biaoqianjg = await this.luoji.biaoqian_chaxun_quanbu();
        if (!biaoqianjg || biaoqianjg.zhuangtaima !== 200) {
            nr.innerHTML = zhuangtai_html('cuowu', '标签加载失败');
            return;
        }
        const leixinglie = tiqushuju(leixingjg, []);
        const biaoqianlie = tiqushuju(biaoqianjg, []);
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
        nr.innerHTML = zhuangtai_html('jiazai', '加载中...');
        const jg = await this.luoji.leixing_chaxun_quanbu();
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = zhuangtai_html('cuowu', `加载失败: ${jg ? jg.xiaoxi : '请求错误'}`);
            return;
        }
        const liebiao = tiqushuju(jg, []);
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
        if (chuli_api_jieguo(this.luoji, jg)) this.shuaxinribaoliebiao();
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
        if (chuli_api_jieguo(this.luoji, jg)) {
            this._bq_xuanzhong_leixingid ? this.bianjibiaoqian_leixing(this._bq_xuanzhong_leixingid) : this.shuaxinbiaoqianliebiao();
        }
    }

    async shanchubiaoqian(id) {
        if (!await aqqueren('删除标签', '确认删除此标签？')) return;
        const jg = await this.luoji.biaoqian_shanchu(id);
        if (chuli_api_jieguo(this.luoji, jg)) {
            this._bq_xuanzhong_leixingid ? this.bianjibiaoqian_leixing(this._bq_xuanzhong_leixingid) : this.shuaxinbiaoqianliebiao();
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
        if (chuli_api_jieguo(this.luoji, jg)) this.shuaxinleixingliebiao();
    }

    async shanchuleixing(id) {
        if (!await aqqueren('删除类型', '确认删除此类型？')) return;
        const jg = await this.luoji.leixing_shanchu(id);
        if (chuli_api_jieguo(this.luoji, jg)) this.shuaxinleixingliebiao();
    }
    async _ribao_huoqu_xiangqing(id) {
        const [ribaojg, biaoqianjg] = await Promise.all([
            this.luoji.ribao_chaxun_id(id),
            this.luoji.guanlian_chaxun_ribaoid_daixinxi_shipei(id)
        ]);
        if (!ribaojg || ribaojg.zhuangtaima !== 200 || !ribaojg.shuju) {
            this.luoji.rizhi('查询日报失败: ' + (ribaojg ? ribaojg.xiaoxi : '请求错误'), 'err');
            return null;
        }
        const biaoqianlie = (biaoqianjg?.zhuangtaima === 200 && Array.isArray(biaoqianjg.shuju))
            ? biaoqianjg.shuju
            : [];
        return { rb: ribaojg.shuju || {}, biaoqianlie };
    }

    _ribao_xuanran_tupu_wenzi(rb, biaoqianlie) {
        const anleixing = new Map();
        for (const bq of (biaoqianlie || [])) {
            const lx = String(bq.leixingmingcheng || '').trim() || '未分类';
            const zhi = String(bq.zhi || '').trim();
            if (!zhi) continue;
            if (!anleixing.has(lx)) anleixing.set(lx, new Set());
            anleixing.get(lx).add(zhi);
        }
        const gongsi = Array.from(anleixing.get('客户公司') || []);
        const lianxiren = Array.from(anleixing.get('客户名字') || []);
        const xiangmu = Array.from(anleixing.get('项目名称') || []);
        const qitaLeixing = Array.from(anleixing.keys()).filter(x => !['客户公司', '客户名字', '项目名称'].includes(x));

        const jieshao = [];
        jieshao.push(`核心：日报「${rb.biaoti || '无标题'}」`);
        jieshao.push(`发布者：${rb.fabuzhemingcheng || rb.fabuzhezhanghao || rb.yonghuid || '-'}，时间：${jiexishijian(rb.fabushijian || '') || '-'}`);
        if (gongsi.length > 0) jieshao.push(`提及公司（${gongsi.length}）：${gongsi.slice(0, 6).join('、')}${gongsi.length > 6 ? ` 等 ${gongsi.length} 个` : ''}`);
        if (lianxiren.length > 0) jieshao.push(`提及联系人（${lianxiren.length}）：${lianxiren.slice(0, 6).join('、')}${lianxiren.length > 6 ? ` 等 ${lianxiren.length} 个` : ''}`);
        if (xiangmu.length > 0) jieshao.push(`提及项目（${xiangmu.length}）：${xiangmu.slice(0, 6).join('、')}${xiangmu.length > 6 ? ` 等 ${xiangmu.length} 个` : ''}`);
        if (qitaLeixing.length > 0) {
            const qita = qitaLeixing.map(lx => `${lx}(${Array.from(anleixing.get(lx) || []).length})`).join('、');
            jieshao.push(`其他实体类型：${qita}`);
        }
        if ((biaoqianlie || []).length === 0) {
            jieshao.push('当前未提取到可关联实体，可先在日报列表补充标签后再看图谱。');
        }

        const shuzhuang = [];
        for (let i = 0; i < jieshao.length; i++) {
            if (i === 0) {
                shuzhuang.push(jieshao[i]);
            } else {
                const qianzhui = i === jieshao.length - 1 ? '└─ ' : '├─ ';
                shuzhuang.push(`${qianzhui}${jieshao[i]}`);
            }
        }

        const gaopin = tongjigaopin(biaoqianlie, null, 8);

        let html = `<div style="margin:10px 0 12px 0;padding:12px;border:1px dashed #CBD5E1;border-radius:8px;background:#F8FAFC">`;
        html += `<div style="font-size:13px;font-weight:600;color:#334155;margin-bottom:8px">文字图谱</div>`;
        html += `<div style="font-size:12px;color:#475569;line-height:1.7;white-space:pre-wrap;background:#FFFFFF;border:1px solid #E2E8F0;border-radius:8px;padding:10px">${this._bq_zhuanyi(shuzhuang.join('\n'))}</div>`;
        if (gaopin.length > 0) {
            html += `<div style="margin-top:8px;font-size:12px;color:#64748B;line-height:1.7">高频线索：`;
            html += gaopin.map(x => `${this._bq_zhuanyi(x.leixing)}「${this._bq_zhuanyi(x.zhi)}」(${x.cishu})`).join('、');
            html += `</div>`;
        }
        html += `</div>`;
        return html;
    }
    async xianshiribaozhidu(id) {
        const xiangqing = await this._ribao_huoqu_xiangqing(id);
        if (!xiangqing) return;
        const { rb, biaoqianlie } = xiangqing;
        const nr = document.getElementById('ribao_neirong');
        if (!nr) return;

        const idjs = this._jiandang_jszhuanyi(rb.id || id || '');
        const biaoti = this._bq_zhuanyi(rb.biaoti || '无标题');
        const fabuzhe = this._bq_zhuanyi(rb.fabuzhemingcheng || rb.fabuzhezhanghao || rb.yonghuid || '-');
        const fabushijian = this._bq_zhuanyi(jiexishijian(rb.fabushijian || ''));
        const neirong = this._bq_zhuanyi(rb.neirong || '');
        const zhaiyao = rb.zhaiyao ? this._bq_zhuanyi(rb.zhaiyao) : '';

        let html = `<div style="padding:14px;border:1px solid #E2E8F0;border-radius:10px;background:#FFFFFF">`;
        html += `<div style="display:flex;justify-content:space-between;align-items:flex-start;gap:10px;flex-wrap:wrap;margin-bottom:12px">`;
        html += `<div><div style="font-size:16px;font-weight:700;color:#0F172A">${biaoti}</div><div style="font-size:12px;color:#64748B;margin-top:4px">ID: ${this._bq_zhuanyi(rb.id || id || '')} ｜ 发布者: ${fabuzhe} ｜ ${fabushijian || '-'}</div></div>`;
        html += `<div style="display:flex;gap:8px;flex-wrap:wrap"><button class="aq-btn aq-btn-xiao" onclick="ribao_quxiao()">返回日报列表</button>`;
        if (this.shifouquanxian) {
            html += `<button class="aq-btn aq-btn-xiao aq-btn-huang" onclick="ribao_bianji('${idjs}')">进入编辑</button>`;
        }
        html += `</div></div>`;
        html += `<div style="font-size:13px;font-weight:600;color:#334155;margin-bottom:8px">关联标签（${biaoqianlie.length}）</div>`;
        if (!biaoqianlie || biaoqianlie.length === 0) {
            html += `<div style="font-size:12px;color:#94A3B8;margin-bottom:10px">暂无标签关联</div>`;
        } else {
            html += `<div style="display:flex;flex-wrap:wrap;gap:6px;margin-bottom:10px">`;
            for (const bq of biaoqianlie) {
                const lx = bq.leixingmingcheng || '未分类';
                const zhi = bq.zhi || '';
                const lxJs = this._jiandang_jszhuanyi(lx);
                const zhiJs = this._jiandang_jszhuanyi(zhi);
                const bqid = bq.biaoqianid ? this._jiandang_jszhuanyi(bq.biaoqianid) : '';
                html += `<span style="display:inline-flex;align-items:center;gap:4px;padding:4px 10px;background:#EFF6FF;color:#1E40AF;border-radius:16px;font-size:12px;transition:background 200ms" onmouseenter="this.style.background='#DBEAFE'" onmouseleave="this.style.background='#EFF6FF'">`;
                html += `<span onclick="ribao_dianjibibaoqian('${lxJs}','${zhiJs}')" style="cursor:pointer;display:inline-flex;align-items:center;gap:4px"><span style="color:#64748B">${this._bq_zhuanyi(lx)}:</span>${this._bq_zhuanyi(zhi)}</span>`;
                if (bqid) {
                    html += `<span onclick="ribao_biaoqian_tiaozhuan_tupu('${bqid}')" title="在图谱中查看" style="cursor:pointer;color:#0369A1;font-size:13px;line-height:1;opacity:0.4;transition:opacity 150ms" onmouseenter="this.style.opacity='1'" onmouseleave="this.style.opacity='0.4'">◎</span>`;
                }
                html += `</span>`;
            }
            html += `</div>`;
        }
        html += this._ribao_xuanran_tupu_wenzi(rb, biaoqianlie);
        if (zhaiyao) {
            html += `<div style="margin-bottom:10px;padding:8px 12px;background:linear-gradient(135deg,#F0FDF4,#ECFDF5);border:1px solid #BBF7D0;border-radius:8px;font-size:13px;color:#15803D;line-height:1.6"><span style="font-weight:600">摘要：</span>${zhaiyao}</div>`;
        }
        html += `<div style="padding:12px;border:1px solid #E2E8F0;border-radius:8px;background:#F8FAFC;white-space:pre-wrap;word-break:break-word;line-height:1.8;color:#1E293B;font-size:14px">${neirong || '<span style="color:#94A3B8">暂无内容</span>'}</div>`;
        html += `</div>`;
        nr.innerHTML = html;
    }

    async chakanribao(id) {
        await this.xianshiribaozhidu(id);
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
        if (chuli_api_jieguo(this.luoji, jg)) {
            this.xuanzhongid = null;
            this.shuaxinribaoliebiao();
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
        nr.innerHTML = zhuangtai_html('jiazai', '加载中...');
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
        nr.innerHTML = zhuangtai_html('jiazai', '加载中...');
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
                html += `<button onclick="ribao_xinzengguanlian('${ribaoid}','${bq.id}')" style="padding:6px 12px;font-size:13px;background:#E0F2FE;color:#0369A1;border:1px solid #BAE6FD;border-radius:20px;cursor:pointer;line-height:1.2;transition:background 200ms" onmouseenter="this.style.background='#BAE6FD'" onmouseleave="this.style.background='#E0F2FE'">${bq.zhi}</button>`;
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
        nr.innerHTML = zhuangtai_html('jiazai', '加载中...');
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
            html += zhuangtai_html('kong', '暂无相关标签');
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
        let guanxilie = fenxi?.guanxi || [];
        let laiyuan = 'kuozhan';
        // 若 kuozhan 中无数据，回退到数据库关系边表查询
        if (guanxilie.length === 0) {
            const jg = await this.luoji.guanxi_chaxun_ribaoid(id);
            if (jg?.zhuangtaima === 200 && Array.isArray(jg.shuju) && jg.shuju.length > 0) {
                guanxilie = jg.shuju;
                laiyuan = 'shujuku';
            }
        }
        if (guanxilie.length === 0) return this.luoji.rizhi('无关系分析数据', 'warn');
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
            liehtml += `<div style="display:flex;flex-direction:column;gap:8px;padding:12px 16px;background:#fff;border:1px solid ${shifufumian ? '#FECACA' : '#E2E8F0'};border-radius:10px;${shifufumian ? 'border-left:3px solid ' + t.zhu : ''}">
                <div style="display:flex;align-items:center;gap:8px;flex-wrap:wrap">
                    <span style="padding:4px 10px;background:${t.qian};color:${t.zhu};border-radius:16px;font-size:12px;font-weight:600;white-space:nowrap">${gx.ren1 || ''}</span>
                    <span style="color:#94A3B8;font-size:18px">—</span>
                    <span style="padding:3px 8px;background:${shifufumian ? t.qian : '#F1F5F9'};color:${shifufumian ? t.zhu : '#475569'};border-radius:6px;font-size:11px;white-space:nowrap;font-weight:${shifufumian ? '700' : '400'}">${lxmc}</span>
                    <span style="color:#94A3B8;font-size:18px">—</span>
                    <span style="padding:4px 10px;background:${t.qian};color:${t.zhu};border-radius:16px;font-size:12px;font-weight:600;white-space:nowrap">${gx.ren2 || ''}</span>
                    ${qinggan !== '中性' ? `<span style="padding:2px 6px;background:${qgYanse.bg};color:${qgYanse.color};border-radius:4px;font-size:10px;white-space:nowrap;margin-left:4px">${qgYanse.icon} ${qinggan}</span>` : ''}
                </div>
                ${gx.miaoshu ? `<div style="font-size:12px;color:#64748B;line-height:1.5;overflow:hidden;display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical">${gx.miaoshu}</div>` : ''}
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
            html += `<div style="padding:10px 16px;border-radius:10px;background:#fff;border-left:3px solid ${t.zhu};box-shadow:0 2px 8px rgba(0,0,0,0.04);flex-shrink:0;min-width:110px;transition:box-shadow 200ms,transform 200ms" onmouseenter="this.style.boxShadow='0 4px 14px rgba(0,0,0,0.08)';this.style.transform='translateY(-1px)'" onmouseleave="this.style.boxShadow='0 2px 8px rgba(0,0,0,0.04)';this.style.transform='none'">`;
            html += `<div style="font-weight:600;font-size:14px;color:${t.zhu}">${jd.mingcheng || ''}</div>`;
            if (jd.neirong) {
                html += `<div style="font-size:12px;color:#64748B;margin-top:4px;line-height:1.5;white-space:pre-wrap;word-break:break-all;max-width:200px">${jd.neirong}</div>`;
            }
            html += '</div>';
        } else {
            const t = zhuti[sy % zhuti.length];
            html += `<div style="padding:7px 12px;border-radius:8px;background:${t.qian};border:1px solid ${t.bian};flex-shrink:0;max-width:280px;transition:box-shadow 200ms,transform 200ms" onmouseenter="this.style.boxShadow='0 3px 10px rgba(0,0,0,0.06)';this.style.transform='translateY(-1px)'" onmouseleave="this.style.boxShadow='none';this.style.transform='none'">`;
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

    _tupu_tiaozhuan_mubiao_cong_biaoqianlie(biaoqianlie) {
        const liebiao = Array.isArray(biaoqianlie) ? biaoqianlie : [];
        if (liebiao.length === 0) return null;
        const youxiao = liebiao
            .map(x => ({ ...x, _biaoqianid: String(x?.biaoqianid || x?.id || '').trim() }))
            .filter(x => x._biaoqianid);
        const youxianLeixing = ['客户公司', '客户名字', '项目名称'];
        for (const lx of youxianLeixing) {
            const pipei = youxiao.find(x => String(x?.leixingmingcheng || '').trim() === lx);
            if (pipei) {
                return {
                    biaoqianid: pipei._biaoqianid,
                    leixingmingcheng: String(pipei.leixingmingcheng || '').trim(),
                    mingcheng: String(pipei.zhi || '').trim(),
                };
            }
        }
        if (youxiao.length > 0) {
            const x = youxiao[0];
            return {
                biaoqianid: x._biaoqianid,
                leixingmingcheng: String(x.leixingmingcheng || '').trim(),
                mingcheng: String(x.zhi || '').trim(),
            };
        }
        const youming = liebiao.find(x => String(x?.zhi || '').trim());
        if (!youming) return null;
        return {
            biaoqianid: null,
            leixingmingcheng: String(youming.leixingmingcheng || '').trim(),
            mingcheng: String(youming.zhi || '').trim(),
        };
    }

    async _tupu_guifan_tiaozhuanmubiao(mubiao) {
        if (!mubiao) return null;
        const biaoqianid = String(mubiao.biaoqianid || '').trim();
        const leixingmingcheng = String(mubiao.leixingmingcheng || '').trim();
        const mingcheng = String(mubiao.mingcheng || '').trim();
        if (biaoqianid) {
            return { biaoqianid, leixingmingcheng, mingcheng };
        }
        if (leixingmingcheng && mingcheng) {
            const leixingjg = await this.luoji.leixing_chaxun_mingcheng(leixingmingcheng);
            const leixingid = leixingjg?.zhuangtaima === 200 ? leixingjg.shuju?.id : null;
            if (leixingid) {
                const biaoqianjg = await this.luoji.biaoqian_chaxun_leixingid_zhi(leixingid, mingcheng);
                const id = biaoqianjg?.zhuangtaima === 200 ? biaoqianjg.shuju?.id : null;
                if (id) {
                    return { biaoqianid: String(id), leixingmingcheng, mingcheng };
                }
            }
            return { biaoqianid: null, leixingmingcheng, mingcheng };
        }
        return null;
    }

    async _tupu_tiaozhuan_bingjihuomubiao(mubiao) {
        const mubiaoxi = await this._tupu_guifan_tiaozhuanmubiao(mubiao);
        if (!mubiaoxi) {
            this.luoji.rizhi('未找到可用于图谱聚焦的目标', 'warn');
            return;
        }
        this._tupu_daohang_biaoqianid = mubiaoxi.biaoqianid || null;
        this._tupu_daohang_mubiao = mubiaoxi;
        await this.qiehuanshitu('tupu');
    }

    async tiaozhuan_tupu(ribaoid) {
        const gljg = await this.luoji.guanlian_chaxun_ribaoid_daixinxi_shipei(ribaoid);
        const biaoqianlie = gljg?.zhuangtaima === 200 ? gljg.shuju || [] : [];
        if (biaoqianlie.length === 0) {
            this.luoji.rizhi('该日报暂无标签，无法跳转图谱', 'warn');
            return;
        }
        const mubiao = this._tupu_tiaozhuan_mubiao_cong_biaoqianlie(biaoqianlie);
        if (!mubiao) {
            this.luoji.rizhi('该日报标签缺少可用图谱定位信息', 'warn');
            return;
        }
        await this._tupu_tiaozhuan_bingjihuomubiao(mubiao);
    }

    async biaoqian_tiaozhuan_tupu(biaoqianid) {
        await this._tupu_tiaozhuan_bingjihuomubiao({ biaoqianid });
    }

    qingchusousuo() {
        this._qingkong_sousuozhuangtai();
        this.shuaxinribaoliebiao();
    }

    _jiandang_jszhuanyi(s) {
        return String(s || '')
            .replace(/\\/g, '\\\\')
            .replace(/\r/g, '\\r')
            .replace(/\n/g, '\\n')
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
            .replace(/"/g, '&quot;')
            .replace(/'/g, "\\'");
    }

    async _jiandang_huoqu_leixingid(leixingmingcheng) {
        const chaxun = await this.luoji.leixing_chaxun_mingcheng(leixingmingcheng);
        if (chaxun?.zhuangtaima === 200 && chaxun.shuju?.id) {
            return chaxun.shuju.id;
        }
        if (!this.shifouquanxian) return null;
        const xin = await this.luoji.leixing_xinzeng(leixingmingcheng);
        if (xin?.zhuangtaima === 200 && xin.shuju?.id) {
            return xin.shuju.id;
        }
        const chaxun2 = await this.luoji.leixing_chaxun_mingcheng(leixingmingcheng);
        return chaxun2?.zhuangtaima === 200 ? chaxun2.shuju?.id : null;
    }

    async _jiandang_huoqu_liebiao(leixingmingcheng, qiangzhishuaxin = false) {
        if (!qiangzhishuaxin && Array.isArray(this._jiandang_liebiao_huancun[leixingmingcheng])) {
            return { chenggong: true, liebiao: this._jiandang_liebiao_huancun[leixingmingcheng] };
        }
        const leixingid = await this._jiandang_huoqu_leixingid(leixingmingcheng);
        if (!leixingid) {
            return { chenggong: false, xiaoxi: `找不到类型：${leixingmingcheng}`, liebiao: [] };
        }

        const [biaoqianjg, fenxigui] = await Promise.all([
            this.luoji.biaoqian_chaxun_leixingid(leixingid),
            this._fenxi_api.shiti_liebiao(leixingmingcheng),
        ]);
        if (biaoqianjg?.zhuangtaima !== 200) {
            return { chenggong: false, xiaoxi: biaoqianjg?.xiaoxi || '查询标签失败', liebiao: [] };
        }

        const ribaoshuMap = new Map();
        if (fenxigui?.chenggong && Array.isArray(fenxigui.shuju)) {
            for (const x of fenxigui.shuju) {
                const zhi = String(x?.zhi || '').trim();
                if (!zhi) continue;
                ribaoshuMap.set(zhi, parseInt(x?.ribao_shu, 10) || 0);
            }
        }

        const yuanlie = Array.isArray(biaoqianjg.shuju) ? biaoqianjg.shuju : [];
        const quchong = new Map();
        for (const x of yuanlie) {
            const zhi = String(x?.zhi || '').trim();
            if (!zhi) continue;
            const id = String(x?.id || '');
            const ribao_shu = ribaoshuMap.get(zhi) || 0;
            if (!quchong.has(zhi)) {
                quchong.set(zhi, { id, zhi, ribao_shu });
            } else {
                const jiu = quchong.get(zhi);
                if (ribao_shu > (parseInt(jiu?.ribao_shu, 10) || 0)) {
                    quchong.set(zhi, { id: jiu?.id || id, zhi, ribao_shu });
                }
            }
        }

        this._jiandang_liebiao_huancun[leixingmingcheng] = Array.from(quchong.values());
        return { chenggong: true, liebiao: this._jiandang_liebiao_huancun[leixingmingcheng] };
    }
    _jiandang_jianyao_liebiao(lie, zuiduo = 6) {
        const quchong = Array.from(new Set((lie || []).map(x => String(x || '').trim()).filter(Boolean)));
        if (quchong.length === 0) return '无';
        const zhan = quchong.slice(0, zuiduo).join('、');
        return quchong.length > zuiduo ? `${zhan} 等 ${quchong.length} 个` : zhan;
    }

    _jiandang_xuanran_tupu_wenzi(leixingmingcheng, mingcheng, ribaolie, biaoqianlie) {
        const guolvQuchong = lx => Array.from(new Set((biaoqianlie || [])
            .filter(x => x.leixingmingcheng === lx)
            .map(x => x.zhi)
            .filter(Boolean)));
        const xiangguanGongsi = leixingmingcheng === '客户公司' ? [] : guolvQuchong('客户公司');
        const xiangguanLianxiren = leixingmingcheng === '客户名字' ? [] : guolvQuchong('客户名字');
        const xiangguanXiangmu = leixingmingcheng === '项目名称' ? [] : guolvQuchong('项目名称');

        const shijianlie = (ribaolie || [])
            .map(rb => Number(rb.fabushijian))
            .filter(ms => !isNaN(ms) && ms > 0);
        const zuizaoriqi = shijianlie.length ? jiexishijian(Math.min(...shijianlie)) : '';
        const zuixinriqi = shijianlie.length ? jiexishijian(Math.max(...shijianlie)) : '';

        const gaopin = tongjigaopin(biaoqianlie, (lx, zhi) => lx === leixingmingcheng && zhi === mingcheng, 6);

        const miaoshuHang = [];
        if (xiangguanGongsi.length > 0) {
            miaoshuHang.push(`关联公司（${xiangguanGongsi.length}）：${this._jiandang_jianyao_liebiao(xiangguanGongsi)}`);
        }
        if (xiangguanLianxiren.length > 0) {
            miaoshuHang.push(`关联联系人（${xiangguanLianxiren.length}）：${this._jiandang_jianyao_liebiao(xiangguanLianxiren)}`);
        }
        if (xiangguanXiangmu.length > 0) {
            miaoshuHang.push(`关联项目（${xiangguanXiangmu.length}）：${this._jiandang_jianyao_liebiao(xiangguanXiangmu)}`);
        }
        if ((ribaolie || []).length > 0) {
            const fanwei = (zuizaoriqi && zuixinriqi) ? `，时间范围：${zuizaoriqi} ~ ${zuixinriqi}` : '';
            miaoshuHang.push(`关联日报（${ribaolie.length}）${fanwei}`);
        }
        if (miaoshuHang.length === 0) {
            miaoshuHang.push('当前暂无可识别的关联节点与证据日报');
        }

        const shuzhuang = [];
        shuzhuang.push(`核心实体：${leixingmingcheng}「${mingcheng}」`);
        for (let i = 0; i < miaoshuHang.length; i++) {
            const qianzhui = i === miaoshuHang.length - 1 ? '└─ ' : '├─ ';
            shuzhuang.push(`${qianzhui}${miaoshuHang[i]}`);
        }

        let html = `<div style="margin:10px 0 12px 0;padding:12px;border:1px dashed #CBD5E1;border-radius:8px;background:#F8FAFC">`;
        html += `<div style="font-size:13px;font-weight:600;color:#334155;margin-bottom:8px">图谱文字化</div>`;
        html += `<div style="font-size:12px;color:#475569;line-height:1.7;white-space:pre-wrap;background:#FFFFFF;border:1px solid #E2E8F0;border-radius:8px;padding:10px">${this._bq_zhuanyi(shuzhuang.join('\n'))}</div>`;
        if (gaopin.length > 0) {
            html += `<div style="margin-top:8px;font-size:12px;color:#64748B;line-height:1.7">高频共现：`;
            html += gaopin.map(x => `${this._bq_zhuanyi(x.leixing)}「${this._bq_zhuanyi(x.zhi)}」(${x.cishu})`).join('、');
            html += `</div>`;
        }
        html += `</div>`;
        return html;
    }

    _jiandang_xuanran_tupu_zitu(leixingmingcheng, mingcheng, biaoqianlie) {
        const jiedianlie = tongjigaopin(biaoqianlie, (lx, zhi) => lx === leixingmingcheng && zhi === mingcheng, 12);

        let html = `<div style="margin:10px 0 12px 0;padding:12px;border:1px solid #E2E8F0;border-radius:8px;background:#FFFFFF">`;
        html += `<div style="display:flex;align-items:center;justify-content:space-between;gap:10px;flex-wrap:wrap;margin-bottom:10px">`;
        html += `<div style="font-size:13px;font-weight:600;color:#334155">关联子图</div>`;
        html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_jiandang_tupu('${this._jiandang_jszhuanyi(leixingmingcheng)}','${this._jiandang_jszhuanyi(mingcheng)}')" style="background:#F5F3FF;color:#7C3AED">在图谱中聚焦</button>`;
        html += `</div>`;
        html += `<div style="display:inline-flex;align-items:center;gap:6px;padding:6px 10px;background:#EEF2FF;color:#4338CA;border-radius:14px;font-size:12px;font-weight:600;margin-bottom:10px">`;
        html += `<span>${this._bq_zhuanyi(leixingmingcheng)}</span><span>「${this._bq_zhuanyi(mingcheng)}」</span>`;
        html += `</div>`;

        if (jiedianlie.length === 0) {
            html += `<div style="font-size:12px;color:#94A3B8">暂无可展示的关联子图节点</div>`;
            html += `</div>`;
            return html;
        }

        html += `<div style="display:grid;gap:8px">`;
        for (const jd of jiedianlie) {
            const lxJs = this._jiandang_jszhuanyi(jd.leixing);
            const zhiJs = this._jiandang_jszhuanyi(jd.zhi);
            const shifou_ke_xiangqing = ['客户公司', '客户名字', '项目名称'].includes(jd.leixing);
            html += `<div style="display:flex;align-items:center;gap:8px;flex-wrap:wrap;padding:8px 10px;background:#F8FAFC;border:1px solid #E2E8F0;border-radius:8px">`;
            html += `<span style="font-size:12px;color:#64748B">${this._bq_zhuanyi(leixingmingcheng)}</span>`;
            html += `<span style="color:#94A3B8">→</span>`;
            html += `<span style="display:inline-flex;align-items:center;gap:5px;padding:3px 8px;background:#ECFEFF;color:#0E7490;border-radius:12px;font-size:12px">`;
            html += `<span>${this._bq_zhuanyi(jd.leixing)}</span><span>「${this._bq_zhuanyi(jd.zhi)}」</span><span style="color:#64748B">(${jd.cishu})</span>`;
            html += `</span>`;
            html += `<div style="display:flex;gap:6px;margin-left:auto">`;
            html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_jiandang_chakanbiaoqianribao('${lxJs}','${zhiJs}')" style="height:28px;min-height:28px;background:#ECFDF5;color:#15803D">看日报</button>`;
            if (shifou_ke_xiangqing) {
                html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_jiandang_xiangqing('${lxJs}','${zhiJs}')" style="height:28px;min-height:28px;background:#EEF2FF;color:#4338CA">看详情</button>`;
            }
            html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_jiandang_tupu('${lxJs}','${zhiJs}')" style="height:28px;min-height:28px;background:#F5F3FF;color:#7C3AED">图谱聚焦</button>`;
            html += `</div></div>`;
        }
        html += `</div>`;
        html += `</div>`;
        return html;
    }

    _jiandang_xuanran_xiangqing(xiangqing) {
        const { leixingmingcheng, mingcheng, ribaolie, biaoqianlie } = xiangqing;
        const guolv = lx => (biaoqianlie || []).filter(x => x.leixingmingcheng === lx).map(x => x.zhi);
        const guolvQuchong = lx => Array.from(new Set(guolv(lx)));
        const xiangmuming = guolvQuchong('项目名称');
        const gongsiming = guolvQuchong('客户公司');
        const lianxirenming = guolvQuchong('客户名字');
        const xiangguanGongsi = leixingmingcheng === '客户公司' ? [] : gongsiming;
        const xiangguanLianxiren = leixingmingcheng === '客户名字' ? [] : lianxirenming;
        const xiangguanXiangmu = leixingmingcheng === '项目名称' ? [] : xiangmuming;

        const tags = (lie, biaoti) => {
            if (!lie || lie.length === 0) return `<div style="font-size:12px;color:#94A3B8">${biaoti}：暂无</div>`;
            return `<div style="display:flex;align-items:center;gap:8px;flex-wrap:wrap"><span style="font-size:12px;color:#64748B">${biaoti}：</span>${lie.map(x => `<span style="padding:3px 8px;background:#EFF6FF;color:#1E40AF;border-radius:12px;font-size:12px">${this._bq_zhuanyi(x)}</span>`).join('')}</div>`;
        };

        let html = `<div style="margin-top:14px;padding:14px;border:1px solid #E2E8F0;border-radius:10px;background:#FFFFFF">`;
        html += `<div style="display:flex;justify-content:space-between;align-items:center;gap:10px;flex-wrap:wrap;margin-bottom:10px">`;
        html += `<div><div style="font-size:16px;font-weight:700;color:#0F172A">${this._bq_zhuanyi(mingcheng)}</div><div style="font-size:12px;color:#64748B;margin-top:2px">${this._bq_zhuanyi(leixingmingcheng)} 详情</div></div>`;
        html += `<div style="display:flex;gap:8px;flex-wrap:wrap">`;
        html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_jiandang_tupu('${this._jiandang_jszhuanyi(leixingmingcheng)}','${this._jiandang_jszhuanyi(mingcheng)}')" style="background:#F5F3FF;color:#7C3AED">跳转图谱</button>`;
        html += `</div></div>`;
        html += `<div style="display:flex;flex-direction:column;gap:8px;margin-bottom:12px">`;
        if (leixingmingcheng === '客户公司') html += tags(xiangguanLianxiren, '关联联系人');
        if (leixingmingcheng === '客户公司') html += tags(xiangguanXiangmu, '关联项目');
        if (leixingmingcheng === '客户名字') html += tags(xiangguanGongsi, '关联公司');
        if (leixingmingcheng === '客户名字') html += tags(xiangguanXiangmu, '关联项目');
        if (leixingmingcheng === '项目名称') html += tags(xiangguanGongsi, '关联公司');
        if (leixingmingcheng === '项目名称') html += tags(xiangguanLianxiren, '关联联系人');
        html += `</div>`;
        html += this._jiandang_xuanran_tupu_zitu(leixingmingcheng, mingcheng, biaoqianlie);
        html += this._jiandang_xuanran_tupu_wenzi(leixingmingcheng, mingcheng, ribaolie, biaoqianlie);

        html += `<div style="font-size:13px;font-weight:600;color:#334155;margin-bottom:8px">相关日报（${ribaolie.length}）</div>`;
        if (!ribaolie || ribaolie.length === 0) {
            html += `<div style="font-size:12px;color:#94A3B8">暂无相关日报</div>`;
        } else {
            html += `<div style="display:flex;flex-direction:column;gap:8px;max-height:300px;overflow:auto">`;
            for (const rb of ribaolie) {
                html += `<div style="padding:10px;border:1px solid #E2E8F0;border-radius:8px;background:#F8FAFC">`;
                html += `<div style="display:flex;justify-content:space-between;align-items:center;gap:8px">`;
                html += `<div style="font-size:13px;font-weight:600;color:#0F172A;word-break:break-all">${this._bq_zhuanyi(rb.biaoti || '无标题')}</div>`;
                html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_jiandang_chakanribao('${this._jiandang_jszhuanyi(rb.id || '')}')" style="height:30px;min-height:30px">查看</button>`;
                html += `</div>`;
                html += `<div style="margin-top:4px;font-size:12px;color:#64748B">${jiexishijian(rb.fabushijian || '')}</div>`;
                html += `</div>`;
            }
            html += `</div>`;
        }
        html += `</div>`;
        return html;
    }

    async shuaxinjiandangshitu() {
        const nr = document.getElementById('ribao_neirong');
        if (!nr) return;
        if (!this.shifouquanxian) {
            nr.innerHTML = '<p style="color:#F59E0B">建档区域仅管理员可用</p>';
            return;
        }
        const fenbiaopeizhi = {
            ribao: { biaoti: '日报管理', leixing: null, xinjianming: '' },
            gongsi: { biaoti: '公司表', leixing: '客户公司', xinjianming: '新建公司（标签）' },
            lianxiren: { biaoti: '联系人表', leixing: '客户名字', xinjianming: '新建联系人（标签）' },
            xiangmu: { biaoti: '项目表', leixing: '项目名称', xinjianming: '新建项目（标签）' },
        };
        const fenbiao = fenbiaopeizhi[this._jiandang_fenbiao] ? this._jiandang_fenbiao : 'gongsi';
        this._jiandang_fenbiao = fenbiao;
        const dangqian = fenbiaopeizhi[fenbiao];

        let html = `<div style="margin-bottom:14px;padding:12px;border:1px solid #E2E8F0;border-radius:10px;background:#FFFFFF">`;
        html += `<div style="display:flex;align-items:center;justify-content:space-between;gap:10px;flex-wrap:wrap">`;
        html += `<div style="font-size:15px;font-weight:700;color:#334155">建档区域</div>`;
        html += `<div style="display:flex;gap:8px;flex-wrap:wrap">`;
        html += `<button class="aq-btn aq-btn-xiao ${fenbiao === 'ribao' ? 'aq-btn-lv' : ''}" onclick="ribao_jiandang_qiehuan('ribao')">日报管理</button>`;
        html += `<button class="aq-btn aq-btn-xiao ${fenbiao === 'gongsi' ? 'aq-btn-lv' : ''}" onclick="ribao_jiandang_qiehuan('gongsi')">公司表</button>`;
        html += `<button class="aq-btn aq-btn-xiao ${fenbiao === 'lianxiren' ? 'aq-btn-lv' : ''}" onclick="ribao_jiandang_qiehuan('lianxiren')">联系人表</button>`;
        html += `<button class="aq-btn aq-btn-xiao ${fenbiao === 'xiangmu' ? 'aq-btn-lv' : ''}" onclick="ribao_jiandang_qiehuan('xiangmu')">项目表</button>`;
        html += `</div></div></div>`;

        if (fenbiao === 'ribao') {
            html += `<div style="padding:14px;border:1px dashed #CBD5E1;border-radius:10px;background:#F8FAFC">`;
            html += `<div style="font-size:14px;color:#334155;margin-bottom:8px">日报管理已在主视图实现，可直接进入日报页操作。</div>`;
            html += `<button class="aq-btn aq-btn-lv" onclick="ribao_qiehuanshitu('ribao')">进入日报管理</button>`;
            html += `</div>`;
            nr.innerHTML = html;
            return;
        }

        const liebiaojg = await this._jiandang_huoqu_liebiao(dangqian.leixing, true);
        if (!liebiaojg.chenggong) {
            nr.innerHTML = html + `<p style="color:#EF4444">加载${dangqian.biaoti}失败：${this._bq_zhuanyi(liebiaojg.xiaoxi || '请求错误')}</p>`;
            return;
        }
        const liebiao = liebiaojg.liebiao || [];
        html += `<div style="padding:14px;border:1px solid #E2E8F0;border-radius:10px;background:#FFFFFF">`;
        html += `<div style="display:flex;align-items:center;justify-content:space-between;gap:10px;flex-wrap:wrap;margin-bottom:10px">`;
        html += `<div style="font-size:14px;font-weight:700;color:#0F172A">${dangqian.biaoti}（${liebiao.length}）</div>`;
        html += `<div style="display:flex;gap:8px;flex-wrap:wrap">`;
        html += `<button class="aq-btn aq-btn-lv" onclick="ribao_jiandang_xinzeng('${this._jiandang_jszhuanyi(dangqian.leixing)}')">${dangqian.xinjianming}</button>`;
        html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_jiandang_qiehuan('${fenbiao}')">刷新</button>`;
        html += `</div></div>`;

        if (liebiao.length === 0) {
            html += `<p style="color:#94A3B8">暂无数据</p>`;
        } else {
            html += `<div style="display:grid;gap:8px">`;
            for (const xiang of liebiao) {
                const mc = xiang.zhi || '';
                const mcJs = this._jiandang_jszhuanyi(mc);
                const lxJs = this._jiandang_jszhuanyi(dangqian.leixing);
                const shu = xiang.ribao_shu || 0;
                html += `<div style="padding:10px;border:1px solid #E2E8F0;border-radius:8px;background:#F8FAFC;display:flex;align-items:center;gap:10px">`;
                html += `<div style="flex:1;min-width:0"><div style="font-size:14px;font-weight:600;color:#0F172A;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${this._bq_zhuanyi(mc)}</div><div style="font-size:12px;color:#64748B;margin-top:2px">关联日报：${shu} 篇</div></div>`;
                html += `<div style="display:flex;gap:6px;flex-wrap:wrap">`;
                html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_jiandang_xiangqing('${lxJs}','${mcJs}')">详情</button>`;
                html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_jiandang_bianji('${lxJs}','${mcJs}')" style="background:#F5F3FF;color:#7C3AED">编辑</button>`;
                html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_jiandang_shanchu('${lxJs}','${mcJs}')" style="background:#FEE2E2;color:#DC2626">删除</button>`;
                if (fenbiao === 'lianxiren') {
                    html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_jiandang_xiangguanribao('${lxJs}','${mcJs}')" style="background:#F5F3FF;color:#7C3AED">相关日报</button>`;
                }
                html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_jiandang_tupu('${lxJs}','${mcJs}')" style="background:#F5F3FF;color:#7C3AED">图谱</button>`;
                html += `</div></div>`;
            }
            html += `</div>`;
        }
        html += `</div>`;

        if (this._jiandang_xiangqing &&
            this._jiandang_xiangqing.leixingmingcheng === dangqian.leixing) {
            html += this._jiandang_xuanran_xiangqing(this._jiandang_xiangqing);
        }
        nr.innerHTML = html;
    }

    async jiandang_qiehuan(fenbiao) {
        const yunxu = ['ribao', 'gongsi', 'lianxiren', 'xiangmu'];
        this._jiandang_fenbiao = yunxu.includes(fenbiao) ? fenbiao : 'gongsi';
        if (this._jiandang_fenbiao === 'ribao') {
            await this.qiehuanshitu('ribao');
            return;
        }
        this._jiandang_xiangqing = null;
        await this.shuaxinjiandangshitu();
    }

    async jiandang_xinzeng(leixingmingcheng) {
        if (!this.shifouquanxian) {
            this.luoji.rizhi('仅管理员可新建', 'warn');
            return;
        }
        const biaoqian = huoqushitibiaoqian(leixingmingcheng);
        const zhi = await aqshuru(`新建${biaoqian}`, `将通过「标签新建」创建${biaoqian}`, '', `请输入${biaoqian}名称`);
        if (zhi === null || zhi === undefined) return;
        const zhiTrim = String(zhi).trim();
        if (!zhiTrim) {
            this.luoji.rizhi('名称不能为空', 'warn');
            return;
        }
        const leixingid = await this._jiandang_huoqu_leixingid(leixingmingcheng);
        if (!leixingid) {
            this.luoji.rizhi(`找不到类型「${leixingmingcheng}」`, 'err');
            return;
        }
        const jg = await this.luoji.biaoqian_xinzeng(leixingid, zhiTrim);
        if (jg?.zhuangtaima === 200) {
            this._jiandang_liebiao_huancun[leixingmingcheng] = null;
            this._jiandang_xiangqing = null;
            await this.shuaxinjiandangshitu();
            return;
        }
        this.luoji.rizhi(`新建${biaoqian}失败：${jg?.xiaoxi || '请求错误'}`, 'err');
    }

    async _jiandang_huoqu_mingcheng_idlie(leixingid, mingcheng) {
        const jg = await this.luoji.biaoqian_chaxun_leixingid(leixingid);
        if (jg?.zhuangtaima !== 200 || !Array.isArray(jg.shuju)) {
            return [];
        }
        const mubiao = String(mingcheng || '').trim();
        return jg.shuju
            .filter(x => String(x?.zhi || '').trim() === mubiao)
            .map(x => String(x?.id || ''))
            .filter(Boolean);
    }

    async jiandang_bianji(leixingmingcheng, mingcheng) {
        if (!this.shifouquanxian) {
            this.luoji.rizhi('仅管理员可编辑', 'warn');
            return;
        }
        const biaoqian = huoqushitibiaoqian(leixingmingcheng);
        const xinmingcheng = await aqshuru(`编辑${biaoqian}`, `修改${biaoqian}名称`, mingcheng, `请输入新的${biaoqian}名称`);
        if (xinmingcheng === null || xinmingcheng === undefined) return;
        const xinTrim = String(xinmingcheng || '').trim();
        const jiuTrim = String(mingcheng || '').trim();
        if (!xinTrim) {
            this.luoji.rizhi('名称不能为空', 'warn');
            return;
        }
        if (xinTrim === jiuTrim) {
            this.luoji.rizhi('名称未变化', 'warn');
            return;
        }

        const leixingid = await this._jiandang_huoqu_leixingid(leixingmingcheng);
        if (!leixingid) {
            this.luoji.rizhi(`找不到类型「${leixingmingcheng}」`, 'err');
            return;
        }
        const idlie = await this._jiandang_huoqu_mingcheng_idlie(leixingid, jiuTrim);
        if (idlie.length === 0) {
            this.luoji.rizhi(`未找到可编辑的${biaoqian}：${jiuTrim}`, 'warn');
            return;
        }

        const jieguolie = await Promise.all(idlie.map(id => this.luoji.biaoqian_gengxin(id, xinTrim)));
        const chenggongshu = jieguolie.filter(jg => jg?.zhuangtaima === 200).length;
        if (chenggongshu === idlie.length) {
            this._jiandang_liebiao_huancun[leixingmingcheng] = null;
            if (this._jiandang_xiangqing &&
                this._jiandang_xiangqing.leixingmingcheng === leixingmingcheng &&
                this._jiandang_xiangqing.mingcheng === jiuTrim) {
                this._jiandang_xiangqing = null;
            }
            await this.shuaxinjiandangshitu();
            return;
        }
        this.luoji.rizhi(`编辑${biaoqian}部分失败（成功 ${chenggongshu}/${idlie.length}）`, 'err');
    }

    async jiandang_shanchu(leixingmingcheng, mingcheng) {
        if (!this.shifouquanxian) {
            this.luoji.rizhi('仅管理员可删除', 'warn');
            return;
        }
        const biaoqian = huoqushitibiaoqian(leixingmingcheng);
        const queren = await aqqueren(`删除${biaoqian}`, `确认删除${biaoqian}「${this._bq_zhuanyi(mingcheng)}」？`);
        if (!queren) return;

        const leixingid = await this._jiandang_huoqu_leixingid(leixingmingcheng);
        if (!leixingid) {
            this.luoji.rizhi(`找不到类型「${leixingmingcheng}」`, 'err');
            return;
        }
        const idlie = await this._jiandang_huoqu_mingcheng_idlie(leixingid, mingcheng);
        if (idlie.length === 0) {
            this.luoji.rizhi(`未找到可删除的${biaoqian}：${mingcheng}`, 'warn');
            return;
        }

        let jg = null;
        if (idlie.length === 1) {
            jg = await this.luoji.biaoqian_shanchu(idlie[0]);
        } else {
            jg = await this.luoji.biaoqian_piliang_shanchu(idlie);
        }
        if (jg?.zhuangtaima === 200) {
            this._jiandang_liebiao_huancun[leixingmingcheng] = null;
            if (this._jiandang_xiangqing &&
                this._jiandang_xiangqing.leixingmingcheng === leixingmingcheng &&
                this._jiandang_xiangqing.mingcheng === String(mingcheng || '').trim()) {
                this._jiandang_xiangqing = null;
            }
            await this.shuaxinjiandangshitu();
            return;
        }
        this.luoji.rizhi(`删除${biaoqian}失败：${jg?.xiaoxi || '请求错误'}`, 'err');
    }

    async jiandang_chakanxiangqing(leixingmingcheng, mingcheng) {
        const fenbiaomap = {
            '客户公司': 'gongsi',
            '客户名字': 'lianxiren',
            '项目名称': 'xiangmu',
        };
        const mubiaoFenbiao = fenbiaomap[leixingmingcheng];
        if (mubiaoFenbiao && this._jiandang_fenbiao !== mubiaoFenbiao) {
            this._jiandang_fenbiao = mubiaoFenbiao;
        }
        const gui = await this._fenxi_api.shiti_ribao(leixingmingcheng, mingcheng);
        if (!gui.chenggong) {
            this.luoji.rizhi(`加载详情失败: ${gui.xiaoxi}，已展示空白详情`, 'warn');
            this._jiandang_xiangqing = {
                leixingmingcheng,
                mingcheng,
                ribaolie: [],
                biaoqianlie: [],
            };
            await this.shuaxinjiandangshitu();
            return;
        }
        this._jiandang_xiangqing = {
            leixingmingcheng,
            mingcheng,
            ribaolie: gui.shuju?.ribaolie || [],
            biaoqianlie: gui.shuju?.biaoqianlie || [],
        };
        await this.shuaxinjiandangshitu();
    }

    async jiandang_xiangguanribao(leixingmingcheng, mingcheng) {
        await this.jiandang_chakanxiangqing(leixingmingcheng, mingcheng);
    }

    async jiandang_tiaozhuan_tupu(leixingmingcheng, mingcheng) {
        await this._tupu_tiaozhuan_bingjihuomubiao({ leixingmingcheng, mingcheng });
    }

    async jiandang_chakanribao(ribaoid) {
        await this.qiehuanshitu('ribao');
        await this.xianshiribaozhidu(ribaoid);
    }

    async jiandang_chakanbiaoqianribao(leixingmingcheng, mingcheng) {
        await this.qiehuanshitu('ribao');
        this.dianjibibaoqian(leixingmingcheng, mingcheng);
    }

    sousuoshuru_huiche(shijian) {
        if (shijian.key === 'Enter') {
            this.sousuoguanjianci();
        }
    }

    async shuaxinrenwuliebiao() {
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = zhuangtai_html('jiazai', '加载中...');
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
        nr.innerHTML = zhuangtai_html('jiazai', '加载图谱数据...');
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
        const daohangmubiao = this._tupu_daohang_mubiao || (this._tupu_daohang_biaoqianid ? { biaoqianid: this._tupu_daohang_biaoqianid } : null);
        const mubiao = daohangmubiao?.biaoqianid || null;
        this._tupu_daohang_biaoqianid = null;
        this._tupu_daohang_mubiao = null;
        if (mubiao || daohangmubiao) {
            await this._tupu_jiazai_impl(null, mubiao, daohangmubiao);
        } else {
            await this._tupu_jiazai(null);
        }
    }

    async _tupu_jiazai_impl(leixingmingcheng, biaoqianid, daohangmubiao = null) {
        const rongqi = document.getElementById('tupu_rongqi');
        if (!rongqi) return;
        this._tupu_xuanranqi.tingzhi();
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
        console.log('[图谱数据]', '节点:', jiedianlie?.length, '边:', bianlie?.length, '关系边:', guanxi_bianlie?.length);
        console.log('[图谱节点]', JSON.parse(JSON.stringify(jiedianlie)));
        if (bianlie?.length) console.log('[图谱共现边]', JSON.parse(JSON.stringify(bianlie)));
        if (guanxi_bianlie?.length) console.log('[图谱关系边]', JSON.parse(JSON.stringify(guanxi_bianlie)));
        // 检查重复节点
        const _zhiCount = {};
        for (const j of jiedianlie || []) { const k = j.zhi || ''; _zhiCount[k] = (_zhiCount[k] || 0) + 1; }
        const _chongfu = Object.entries(_zhiCount).filter(([, c]) => c > 1);
        if (_chongfu.length) console.warn('[图谱重复节点]', _chongfu.map(([z, c]) => `"${z}" x${c}`).join(', '));
        if (!jiedianlie || jiedianlie.length === 0) {
            rongqi.innerHTML = '<p style="color:#94A3B8;padding:20px">暂无图谱数据</p>';
            return;
        }
        const xuanzeMubiao = daohangmubiao || (biaoqianid ? { biaoqianid } : null);
        await this._tupu_xuanran(rongqi, jiedianlie, bianlie || [], guanxi_bianlie || [], xuanzeMubiao);
    }

    async _tupu_jiazai(leixingmingcheng) {
        return this._tupu_jiazai_impl(leixingmingcheng, null);
    }

    async _tupu_jiazai_biaoqianid(biaoqianid) {
        return this._tupu_jiazai_impl(null, biaoqianid, { biaoqianid });
    }

    _tupu_moren_xuanzhong_jiedian(mubiao, changshi = 0) {
        if (!mubiao || !this._tupu_xuanranqi) return;
        const chenggong = this._tupu_xuanranqi.xuanze_jiedian(
            mubiao,
            (j, gx_lie) => this._tupu_xianshi_celan_jiedian(j, gx_lie)
        );
        if (chenggong) return;
        if (changshi >= 6) return;
        setTimeout(() => this._tupu_moren_xuanzhong_jiedian(mubiao, changshi + 1), 120);
    }

    async _tupu_xuanran(rongqi, jiedianlie, bianlie, guanxi_bianlie, zhongxinmubiao) {
        const zhongxinid = zhongxinmubiao?.biaoqianid || null;
        await this._tupu_xuanranqi.xuanran(rongqi, jiedianlie, bianlie, guanxi_bianlie, zhongxinid, {
            fanhui: () => this._tupu_jiazai(null),
            dianji_jiedian: (j, gx_lie) => this._tupu_xianshi_celan_jiedian(j, gx_lie),
            shuangji: (id) => this._tupu_jiazai_biaoqianid(id),
            dianji_bian: (yuan, mubiao, quanzhong) => this._tupu_xianshi_celan_bian(yuan, mubiao, quanzhong),
            dianji_guanxi_bian: (yuan, mubiao, gb) => this._tupu_xianshi_celan_guanxibian(yuan, mubiao, gb),
        });
        this._tupu_tingzhi = () => this._tupu_xuanranqi.tingzhi();
        if (zhongxinmubiao) {
            this._tupu_moren_xuanzhong_jiedian(zhongxinmubiao);
        }
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

    _tupu_celan_chongzhizhuangtai(huoqufn, fugai = {}) {
        this._tupu_celan_yeshu = 1;
        this._tupu_celan_huoqufn = huoqufn || null;
        this._tupu_celan_biaoqianid = fugai.biaoqianid ?? null;
        this._tupu_celan_shitimingcheng = fugai.shitimingcheng ?? null;
        this._tupu_celan_yuan_id = fugai.yuan_id ?? null;
        this._tupu_celan_mubiao_id = fugai.mubiao_id ?? null;
        this._tupu_celan_guanxi_ren1 = fugai.guanxi_ren1 ?? null;
        this._tupu_celan_guanxi_ren2 = fugai.guanxi_ren2 ?? null;
    }

    _tupu_xianshi_celan_jiedian(j, gx_lie) {
        const celan = document.getElementById('tupu_celan');
        if (!celan) return;
        celan.style.display = 'block';
        const tubiao = '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="white" stroke-width="2" stroke-linecap="round"><circle cx="8" cy="8" r="4.5"/></svg>';
        const shifou_xuni = String(j.id).startsWith('-');
        let gx_html = '';
        if (gx_lie && gx_lie.length > 0) {
            gx_html += `<div style="display:flex;align-items:center;gap:6px;margin-bottom:10px">
                <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="#7C3AED" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><path d="M4 8h8"/><circle cx="3" cy="8" r="2"/><circle cx="13" cy="8" r="2"/></svg>
                <span style="font-size:13px;font-weight:600;color:#475569">AI \u5173\u7cfb</span>
                <span style="display:inline-flex;align-items:center;padding:1px 7px;background:#F5F3FF;border-radius:8px;font-size:11px;color:#7C3AED;font-weight:600">${gx_lie.length}</span>
            </div>`;
            for (const gx of gx_lie) {
                const _esc_yuan_ming = j.zhi.replace(/\\/g, '\\\\').replace(/'/g, "\\'");
                const _esc_mb_ming = gx.duifang.zhi.replace(/\\/g, '\\\\').replace(/'/g, "\\'");
                gx_html += `<div onclick="ribao_tupu_celan_chakanGuanxi('${_esc_yuan_ming}','${_esc_mb_ming}')" style="border:1px solid #EDE9FE;border-radius:8px;padding:10px 12px;margin-bottom:8px;cursor:pointer;transition:all 150ms;border-left:3px solid ${gx.secai || '#8B5CF6'}" onmouseenter="this.style.background='#F5F3FF';this.style.boxShadow='0 2px 8px rgba(139,92,246,0.08)'" onmouseleave="this.style.background='';this.style.boxShadow='none'">
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
        const zitu_html = shifou_xuni ? '' : `<button class="aq-btn aq-btn-xiao" onclick="ribao_tupu_jiazai_biaoqianid_celan('${j.id}')" style="width:100%;display:flex;align-items:center;justify-content:center;gap:6px;background:transparent;color:#3B82F6;border:1.5px solid #BFDBFE;border-radius:8px;padding:9px;margin-bottom:16px;font-weight:500;transition:all 150ms" onmouseenter="this.style.background='#EFF6FF';this.style.borderColor='#3B82F6'" onmouseleave="this.style.background='transparent';this.style.borderColor='#BFDBFE'">
                        <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><circle cx="7" cy="7" r="4.5"/><line x1="10.5" y1="10.5" x2="14" y2="14"/><line x1="5.5" y1="7" x2="8.5" y2="7"/><line x1="7" y1="5.5" x2="7" y2="8.5"/></svg>
                        \u67e5\u770b\u5b50\u56fe
                    </button>`;
        celan.innerHTML = `
            <div style="display:flex;flex-direction:column;height:100%">
                ${this._tupu_celan_tou_html(tubiao, '\u8282\u70b9\u8be6\u60c5')}
                <div style="flex:1;overflow-y:auto;padding:16px">
                    <div style="background:linear-gradient(135deg,#F8FAFC,#F1F5F9);border:1px solid #E2E8F0;border-radius:10px;padding:14px;margin-bottom:14px;border-left:3px solid ${shifou_xuni ? '#F59E0B' : '#3B82F6'}">
                        <div style="display:inline-block;padding:2px 8px;background:${shifou_xuni ? '#FEF3C7' : '#EFF6FF'};color:${shifou_xuni ? '#D97706' : '#3B82F6'};border-radius:10px;font-size:11px;font-weight:600;letter-spacing:0.3px">${j.leixing}</div>
                        <div style="font-size:14px;font-weight:600;color:#0F172A;margin-top:8px;line-height:1.5">${j.zhi}</div>
                    </div>
                    ${zitu_html}
                    ${gx_html}
                    <div style="display:flex;align-items:center;gap:6px;margin-bottom:10px">
                        <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="#64748B" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="12" height="11" rx="1.5"/><line x1="2" y1="7" x2="14" y2="7"/><line x1="6" y1="3" x2="6" y2="7"/></svg>
                        <span style="font-size:13px;font-weight:600;color:#475569">\u5173\u8054\u65e5\u62a5</span>
                    </div>
                    <div id="tupu_celan_ribaolie"><p style="color:#94A3B8;font-size:13px">\u52a0\u8f7d\u4e2d...</p></div>
                </div>
            </div>`;
        const huoqufn = shifou_xuni
            ? (y, m) => this.luoji.tupu_guanxi_shiti_ribao_fenye(j.zhi, y, m)
            : (y, m) => this.luoji.tupu_ribao_fenye(j.id, y, m);
        this._tupu_celan_chongzhizhuangtai(huoqufn, {
            biaoqianid: shifou_xuni ? null : j.id,
            shitimingcheng: shifou_xuni ? j.zhi : null,
        });
        this._tupu_celan_jiazai_ribao(1);
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
        this._tupu_celan_chongzhizhuangtai(
            (y, m) => this.luoji.tupu_guanxi_bian_ribao_fenye(yuan.zhi, mubiao.zhi, y, m),
            { guanxi_ren1: yuan.zhi, guanxi_ren2: mubiao.zhi }
        );
        this._tupu_celan_jiazai_ribao(1);
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
        this._tupu_celan_chongzhizhuangtai(
            (y, m) => this.luoji.tupu_bian_ribao_fenye(yuan.id, mubiao.id, y, m),
            { yuan_id: yuan.id, mubiao_id: mubiao.id }
        );
        this._tupu_celan_jiazai_ribao(1);
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

    async _tupu_celan_jiazai_ribao(yeshu) {
        if (!this._tupu_celan_huoqufn) return;
        const meiyetiaoshu = 5;
        const jg = await this._tupu_celan_huoqufn(yeshu, meiyetiaoshu);
        if (!jg || jg.zhuangtaima !== 200) {
            const rongqi = document.getElementById('tupu_celan_ribaolie');
            if (rongqi) rongqi.innerHTML = '<p style="color:#EF4444;font-size:13px">加载失败</p>';
            return;
        }
        const { liebiao = [], zongshu = 0 } = jg.shuju || {};
        this._tupu_celan_xuanran_ribaolie(liebiao, zongshu, yeshu, meiyetiaoshu);
    }

    tupu_celan_chakanGuanxi(ren1, ren2) {
        this._tupu_celan_chongzhizhuangtai(
            (y, m) => this.luoji.tupu_guanxi_bian_ribao_fenye(ren1, ren2, y, m),
            { guanxi_ren1: ren1, guanxi_ren2: ren2 }
        );
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
        this._tupu_celan_jiazai_ribao(1);
    }

    _tupu_celan_fanyue(fangxiang) {
        const yeshu = this._tupu_celan_yeshu + fangxiang;
        if (yeshu < 1 || !this._tupu_celan_huoqufn) return;
        this._tupu_celan_jiazai_ribao(yeshu);
    }
    tupu_celan_shangyiye() { this._tupu_celan_fanyue(-1); }
    tupu_celan_xiayiye() { this._tupu_celan_fanyue(1); }

    async tupu_chakanribao(ribaoid) {
        await this.qiehuanshitu('ribao');
        await this.xianshiribaozhidu(ribaoid);
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
                html += `<div style="padding:8px 12px;cursor:pointer;border-bottom:1px solid #F1F5F9;font-size:13px" onmouseenter="this.style.background='#F8FAFC'" onmouseleave="this.style.background=''" onclick="ribao_tupu_sousuo_xuanze('${item.biaoqianid}')">
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
    .fenxi-tag{display:inline-flex;align-items:center;gap:6px;padding:4px 10px;background:#F1F5F9;border:1px solid #E2E8F0;border-radius:6px;font-size:12px;color:#475569}

    .fenxi-ribao-list{display:flex;flex-direction:column;gap:8px;max-height:420px;overflow:auto;padding:10px;background:#FFFFFF;border:1px solid #E2E8F0;border-radius:12px}
    .fenxi-ribao{border:1px solid #E2E8F0;border-radius:12px;overflow:hidden;background:#FFFFFF}
    .fenxi-ribao summary{list-style:none;padding:10px 12px;cursor:pointer;user-select:none;outline:none;display:flex;gap:10px;align-items:flex-start;justify-content:space-between}
    .fenxi-ribao summary::-webkit-details-marker{display:none}
    .fenxi-ribao:hover{border-color:#CBD5E1}
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
        label.style.cssText = 'display:inline-flex;align-items:center;gap:5px;padding:5px 12px;background:#F0FDF4;border:1px solid #16A34A;border-radius:999px;cursor:pointer;font-size:12px;color:#166534;transition:all 150ms;user-select:none';
        label.onmouseenter = function() { this.style.borderColor = '#86EFAC'; };
        label.onmouseleave = function() { this.style.borderColor = this.querySelector('input').checked ? '#16A34A' : '#BBF7D0'; };
        label.innerHTML = `<input type="checkbox" class="fenxi_weidu_xz" value="${zhi.replace(/"/g, '&quot;')}" checked onchange="this.parentElement.style.borderColor=this.checked?'#16A34A':'#BBF7D0'" style="width:14px;height:14px;accent-color:#16A34A;cursor:pointer;outline:none"><span style="display:inline-block;width:8px;height:8px;border-radius:50%;background:#16A34A;flex-shrink:0"></span>${zhi}<span onclick="this.parentElement.remove();event.stopPropagation()" style="margin-left:2px;color:#94A3B8;cursor:pointer;font-size:14px;line-height:1">×</span>`;
        qu.appendChild(label);
        shuru.value = '';
    }

    fenxi_quanxuan_weidu(xuanzhong) {
        const cblie = document.querySelectorAll('.fenxi_weidu_xz');
        for (const cb of cblie) {
            cb.checked = xuanzhong;
            const lbl = cb.closest('label');
            if (lbl) lbl.style.borderColor = xuanzhong ? '#3B82F6' : '#E2E8F0';
        }
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

    _fenxi_xianshi_guanlian_jiemian(biaoti, fubiaoti, tishiwen) {
        const jieguo_qu = document.getElementById('fenxi_jieguo_qu');
        if (!jieguo_qu) return;
        let html = `<div class="fenxi-title">${biaoti}</div>`;
        html += `<div class="fenxi-sub">${fubiaoti}</div>`;
        html += `<div style="margin:16px 0">`;
        html += `<div style="font-size:12px;font-weight:700;color:#475569;margin-bottom:8px">你想让 AI 重点分析什么？（可选）</div>`;
        html += `<textarea id="fenxi_guanlian_tishi" rows="3" placeholder="例如：${tishiwen}\n留空则使用默认全面分析" style="width:100%;padding:10px 12px;border:1px solid #E2E8F0;border-radius:10px;font-size:13px;outline:none;resize:vertical;box-sizing:border-box;font-family:inherit"></textarea>`;
        html += `</div>`;
        html += `<button class="aq-btn aq-btn-zhu" onclick="ribao_fenxi_guanlian_kaishi()" style="margin-right:8px">开始深度分析</button>`;
        html += `<span style="font-size:12px;color:#94A3B8">将基于日报原文进行深度关联分析</span>`;
        jieguo_qu.innerHTML = html;
    }

    fenxi_shiti_guanlian(leixingmingcheng) {
        const xuanzhong = this._fenxi_zt.huoquXuanzhonglie(leixingmingcheng);
        if (xuanzhong.length < 2) { this.luoji.rizhi('请至少勾选2个进行关联分析', 'warn'); return; }
        const lx_peizhi = this._fenxi_zt.chazhaoLeixingPeizhi(leixingmingcheng);
        const biaoti = lx_peizhi?.biaoti || leixingmingcheng;
        this._guanlian_huancun = { leixing: 'shiti', leixingmingcheng, xuanzhong, biaoti };
        this._fenxi_xianshi_guanlian_jiemian(`${biaoti}关联分析`, `已选：${xuanzhong.join(' / ')}`, `分析这些${biaoti}之间的关联关系、资源冲突、协作问题...`);
    }

    fenxi_zonghe_guanlian() {
        const suoyou = this._fenxi_zt.huoquSuoyouXuanzhong();
        if (suoyou.length < 2) { this.luoji.rizhi('请跨类型勾选至少2个实体进行关联分析', 'warn'); return; }
        this._guanlian_huancun = { leixing: 'zonghe', suoyou };
        const miaoshu = suoyou.map(s => `${s.leixing}:${s.zhi}`).join(' / ');
        this._fenxi_xianshi_guanlian_jiemian('综合关联分析', `已选实体：${miaoshu}`, '分析这些人员和项目之间的协作问题、风险点、资源冲突...');
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

