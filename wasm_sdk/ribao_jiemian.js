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
            : ['ribao', 'quanburibao'];
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
        html += '<button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_ribao_piliangshanchu()" style="height:36px">批量删除</button>';
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
        html += '<button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_ribao_piliangshanchu()" style="height:36px">批量删除</button>';
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
                        ${rb.kuozhan ? `<button class="aq-btn aq-btn-xiao aq-btn-zhu" onclick="ribao_siweidaotu('${rb.id}')">导图</button>` : ''}
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

    siweidaotu(id) {
        const rb = this._ribaoshujuhuancun?.[id];
        if (!rb || !rb.kuozhan) return this.luoji.rizhi('无思维导图数据', 'warn');
        let shuju;
        try {
            shuju = typeof rb.kuozhan === 'string' ? JSON.parse(rb.kuozhan) : rb.kuozhan;
        } catch (e) {
            return this.luoji.rizhi('思维导图数据解析失败', 'err');
        }
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
