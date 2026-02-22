// AI渠道管理 - 界面层
export class Aiqudaojiemian {
    constructor(luoji, rongqiid) {
        this.luoji = luoji;
        this.rongqi = document.getElementById(rongqiid);
        this.dangqianshitu = 'liebiao';
        this.xuanzhongid = null;
    }

    xuanran() {
        this.rongqi.innerHTML = '';
        const tou = document.createElement('div');
        tou.style.cssText = 'display:flex;justify-content:space-between;align-items:center;margin-bottom:12px';
        tou.innerHTML = `<h2 style="font-size:15px;color:#475569;margin:0">AI渠道管理</h2>
            <div><button class="aq-btn aq-btn-zhu" onclick="aiqudao_shuaxin()">刷新列表</button>
            <button class="aq-btn aq-btn-lv" onclick="aiqudao_xinzengshitu()">新增渠道</button></div>`;
        this.rongqi.appendChild(tou);
        const neirong = document.createElement('div');
        neirong.id = 'aiqudao_neirong';
        this.rongqi.appendChild(neirong);
    }

    async shuaxinliebiao() {
        const jg = await this.luoji.chaxunquanbu();
        const nr = document.getElementById('aiqudao_neirong');
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = `<p style="color:#EF4444">加载失败: ${jg ? jg.xiaoxi : '请求错误'}</p>`;
            return;
        }
        const liebiao = jg.shuju || [];
        if (liebiao.length === 0) {
            nr.innerHTML = '<p style="color:#64748B">暂无渠道数据</p>';
            return;
        }
        let html = '<div style="overflow-x:auto"><table class="aq-biao"><thead><tr>' +
            '<th>ID</th><th>名称</th><th>类型</th><th>模型</th><th>温度</th><th>最大Token</th><th>优先级</th><th>状态</th><th>操作</th>' +
            '</tr></thead><tbody>';
        for (const qd of liebiao) {
            const zt = qd.zhuangtai === '1';
            const zthtml = zt
                ? '<span style="color:#10B981;font-weight:600">启用</span>'
                : '<span style="color:#EF4444;font-weight:600">禁用</span>';
            html += `<tr>
                <td>${qd.id}</td><td>${qd.mingcheng}</td><td>${qd.leixing}</td>
                <td>${qd.moxing}</td><td>${qd.wendu}</td><td>${qd.zuida_token || 0}</td><td>${qd.youxianji}</td>
                <td>${zthtml}</td>
                <td style="white-space:nowrap">
                    <button class="aq-btn aq-btn-xiao" onclick="aiqudao_qiehuan('${qd.id}')">${zt ? '禁用' : '启用'}</button>
                    <button class="aq-btn aq-btn-xiao aq-btn-huang" onclick="aiqudao_bianji('${qd.id}')">编辑</button>
                    <button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="aiqudao_shanchu('${qd.id}')">删除</button>
                </td></tr>`;
        }
        html += '</tbody></table></div>';
        nr.innerHTML = html;
    }

    xuanranxinzengbiaodan() {
        const nr = document.getElementById('aiqudao_neirong');
        nr.innerHTML = `<div class="aq-biaodan">
            <div class="aq-hang"><label>名称</label><input id="aq_mingcheng" type="text" placeholder="渠道名称"></div>
            <div class="aq-hang"><label>类型</label>
                <select id="aq_leixing" style="border:1px solid #E2E8F0;border-radius:8px;padding:8px 12px;font-size:14px;outline:none;color:#1E293B;background:#fff;cursor:pointer">
                    <option value="">请选择类型</option>
                    <option value="openapi">openapi</option>
                    <option value="xiangliang">xiangliang</option>
                    <option value="yuyin">yuyin</option>
                </select>
            </div>
            <div class="aq-hang"><label>接口地址</label><input id="aq_jiekoudizhi" type="text" placeholder="API基础地址"></div>
            <div class="aq-hang"><label>密钥</label><input id="aq_miyao" type="text" placeholder="API密钥"></div>
            <div class="aq-hang"><label>模型</label><input id="aq_moxing" type="text" placeholder="模型名称"></div>
            <div class="aq-hang"><label>温度</label><input id="aq_wendu" type="text" value="0.7" placeholder="0.0-2.0"></div>
            <div class="aq-hang"><label>最大Token</label><input id="aq_zuida_token" type="text" value="0" placeholder="0表示不限制"></div>
            <div class="aq-hang"><label>备注</label><input id="aq_beizhu" type="text" placeholder="可选"></div>
            <div style="margin-top:12px">
                <button class="aq-btn aq-btn-lv" onclick="aiqudao_tijiaoxinzeng()">提交</button>
                <button class="aq-btn" onclick="aiqudao_shuaxin()">取消</button>
            </div></div>`;
    }

    async tijiaoxinzeng() {
        const hq = id => document.getElementById(id)?.value?.trim() || '';
        const shuju = {
            mingcheng: hq('aq_mingcheng'), leixing: hq('aq_leixing'),
            jiekoudizhi: hq('aq_jiekoudizhi'), miyao: hq('aq_miyao'),
            moxing: hq('aq_moxing'), wendu: hq('aq_wendu'),
            zuida_token: hq('aq_zuida_token') || '0', beizhu: hq('aq_beizhu') || undefined
        };
        if (!shuju.mingcheng || !shuju.leixing || !shuju.jiekoudizhi || !shuju.miyao || !shuju.moxing) {
            this.luoji.rizhi('请填写所有必填字段', 'warn');
            return;
        }
        const jg = await this.luoji.xinzeng(shuju);
        if (jg && jg.zhuangtaima === 200) await this.shuaxinliebiao();
    }

    async qiehuanzhuangtai(id) {
        const jg = await this.luoji.qiehuanzhuangtai(id);
        if (jg && jg.zhuangtaima === 200) await this.shuaxinliebiao();
    }

    async shanchuid(id) {
        if (!await aqqueren('删除渠道', '确定要删除该渠道吗？此操作不可恢复。')) return;
        const jg = await this.luoji.shanchu(id);
        if (jg && jg.zhuangtaima === 200) await this.shuaxinliebiao();
    }

    async bianji(id) {
        const jg = await this.luoji.chaxunid(id);
        if (!jg || jg.zhuangtaima !== 200) return;
        const qd = jg.shuju;
        this.xuanzhongid = id;
        const nr = document.getElementById('aiqudao_neirong');
        nr.innerHTML = `<div class="aq-biaodan">
            <div class="aq-hang"><label>ID</label><input value="${qd.id}" disabled></div>
            <div class="aq-hang"><label>名称</label><input id="aq_b_mingcheng" type="text" value="${qd.mingcheng}"></div>
            <div class="aq-hang"><label>类型</label>
                <select id="aq_b_leixing" style="border:1px solid #E2E8F0;border-radius:8px;padding:8px 12px;font-size:14px;outline:none;color:#1E293B;background:#fff;cursor:pointer">
                    <option value="openapi" ${qd.leixing === 'openapi' ? 'selected' : ''}>openapi</option>
                    <option value="xiangliang" ${qd.leixing === 'xiangliang' ? 'selected' : ''}>xiangliang</option>
                    <option value="yuyin" ${qd.leixing === 'yuyin' ? 'selected' : ''}>yuyin</option>
                </select>
            </div>
            <div class="aq-hang"><label>接口地址</label><input id="aq_b_jiekoudizhi" type="text" value="${qd.jiekoudizhi}"></div>
            <div class="aq-hang"><label>密钥</label><input id="aq_b_miyao" type="text" value="${qd.miyao}"></div>
            <div class="aq-hang"><label>模型</label><input id="aq_b_moxing" type="text" value="${qd.moxing}"></div>
            <div class="aq-hang"><label>温度</label><input id="aq_b_wendu" type="text" value="${qd.wendu}"></div>
            <div class="aq-hang"><label>最大Token</label><input id="aq_b_zuida_token" type="text" value="${qd.zuida_token || 0}"></div>
            <div class="aq-hang"><label>备注</label><input id="aq_b_beizhu" type="text" value="${qd.beizhu || ''}"></div>
            <div style="margin-top:12px">
                <button class="aq-btn aq-btn-huang" onclick="aiqudao_tijiaobian()">保存修改</button>
                <button class="aq-btn" onclick="aiqudao_shuaxin()">取消</button>
            </div></div>`;
    }

    async tijiaobian() {
        if (!this.xuanzhongid) return;
        const hq = id => document.getElementById(id)?.value?.trim() || '';
        const ziduanlie = [
            ['mingcheng', hq('aq_b_mingcheng')], ['leixing', hq('aq_b_leixing')],
            ['jiekoudizhi', hq('aq_b_jiekoudizhi')], ['miyao', hq('aq_b_miyao')],
            ['moxing', hq('aq_b_moxing')], ['wendu', hq('aq_b_wendu')],
            ['zuida_token', hq('aq_b_zuida_token')], ['beizhu', hq('aq_b_beizhu')]
        ].filter(([, v]) => v);
        if (ziduanlie.length === 0) { this.luoji.rizhi('没有需要更新的字段', 'warn'); return; }
        const jg = await this.luoji.gengxin(this.xuanzhongid, ziduanlie);
        if (jg && jg.zhuangtaima === 200) { this.xuanzhongid = null; await this.shuaxinliebiao(); }
    }
}
