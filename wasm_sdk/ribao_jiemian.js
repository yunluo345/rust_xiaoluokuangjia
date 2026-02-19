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
    }

    async xuanran() {
        this.rongqi.innerHTML = '';
        const tou = document.createElement('div');
        tou.style.cssText = 'display:flex;justify-content:space-between;align-items:center;margin-bottom:12px';
        const anniulie = this.shifouquanxian
            ? `<button class="aq-btn aq-btn-zhu" onclick="ribao_qiehuanshitu('ribao')">日报</button>
               <button class="aq-btn aq-btn-zhu" onclick="ribao_qiehuanshitu('biaoqian')">标签</button>
               <button class="aq-btn aq-btn-zhu" onclick="ribao_qiehuanshitu('leixing')">类型</button>
               <button class="aq-btn aq-btn-lv" onclick="ribao_shuaxin()">刷新数据</button>`
            : `<button class="aq-btn aq-btn-lv" onclick="ribao_shuaxin()">刷新数据</button>`;
        tou.innerHTML = `<h2 style="font-size:15px;color:#475569;margin:0">日报管理</h2><div>${anniulie}</div>`;
        this.rongqi.appendChild(tou);
        const neirong = document.createElement('div');
        neirong.id = 'ribao_neirong';
        neirong.innerHTML = '<p style="color:#94A3B8;font-size:14px">点击「刷新数据」加载日报信息</p>';
        this.rongqi.appendChild(neirong);
    }

    async qiehuanshitu(shitu) {
        this.dangqianshitu = this.shifouquanxian ? shitu : 'ribao';
        await this.shuaxindangqianshitu();
    }

    async shuaxindangqianshitu() {
        const shitumap = {
            'ribao': () => this.shuaxinribaoliebiao(),
            'biaoqian': () => this.shuaxinbiaoqianliebiao(),
            'leixing': () => this.shuaxinleixingliebiao()
        };
        const shitu = this.shifouquanxian ? this.dangqianshitu : 'ribao';
        const fn = shitumap[shitu];
        if (fn) await fn();
    }

    async shuaxinribaoliebiao() {
        const nr = document.getElementById('ribao_neirong');
        const jg = await this.luoji.chaxunfenye_shipei(this.dangqianyeshu, this.meiyetiaoshu);
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = `<p style="color:#EF4444">加载失败: ${jg ? jg.xiaoxi : '请求错误'}</p>`;
            return;
        }
        const liebiao = jg.shuju?.liebiao || [];
        const zongshu = jg.shuju?.zongshu || 0;
        if (liebiao.length === 0) {
            nr.innerHTML = '<p style="color:#64748B">暂无日报数据</p><button class="aq-btn aq-btn-lv" onclick="ribao_xinzengshitu()">新增日报</button>';
            return;
        }
        let html = '<div style="margin-bottom:12px"><button class="aq-btn aq-btn-lv" onclick="ribao_xinzengshitu()">新增日报</button></div>';
        html += '<div style="overflow-x:auto"><table class="aq-biao"><thead><tr>' +
            '<th>ID</th><th>内容</th><th>发布时间</th>' + (this.shifouquanxian ? '<th>用户ID</th>' : '') + '<th>操作</th>' +
            '</tr></thead><tbody>';
        for (const rb of liebiao) {
            const neirong = rb.neirong.length > 50 ? rb.neirong.substring(0, 50) + '...' : rb.neirong;
            html += `<tr>
                <td>${rb.id}</td><td>${neirong}</td><td>${rb.fabushijian}</td>
                ${this.shifouquanxian ? `<td>${rb.yonghuid}</td>` : ''}
                <td style="white-space:nowrap">
                    <button class="aq-btn aq-btn-xiao aq-btn-huang" onclick="ribao_bianji('${rb.id}')">编辑</button>
                    <button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="ribao_shanchu('${rb.id}')">删除</button>
                    <button class="aq-btn aq-btn-xiao" onclick="ribao_guanlianguanlian('${rb.id}')">标签</button>
                </td>
            </tr>`;
        }
        html += '</tbody></table></div>';
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
        if (!confirm('确认删除此日报？')) return;
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
        if (!confirm('确认删除此标签？')) return;
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
        if (!confirm('确认删除此类型？')) return;
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
        const nr = document.getElementById('ribao_neirong');
        nr.innerHTML = '<p style="color:#64748B">加载中...</p>';
        const guanlianjg = await this.luoji.guanlian_chaxun_ribaoid(ribaoid);
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
            html += '<div style="display:flex;flex-wrap:wrap;gap:8px">';
            for (const gl of yiguanlian) {
                const bq = suoyoubiaoqian.find(b => b.id === gl.biaoqianid);
                const mingcheng = bq ? bq.zhi : gl.biaoqianid;
                html += `<span style="display:inline-flex;align-items:center;gap:6px;padding:6px 12px;background:#EFF6FF;color:#1E40AF;border-radius:20px;font-size:13px;line-height:1.2">
                    ${mingcheng}
                    <button onclick="ribao_shanchuguanlian('${ribaoid}','${gl.biaoqianid}')" style="background:none;border:none;color:#DC2626;cursor:pointer;padding:0;font-size:14px;line-height:1;display:flex;align-items:center">×</button>
                </span>`;
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

    quxiao() {
        this.shuaxindangqianshitu();
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

    shezhiquanxian(shifouquanxian) {
        this.shifouquanxian = !!shifouquanxian;
        this.dangqianshitu = 'ribao';
        this.dangqianyeshu = 1;
        this.shuaxindangqianshitu();
    }
}
