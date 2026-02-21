// 用户管理 - 界面层
export class Yonghujiemian {
    constructor(luoji, rongqiid) {
        this.luoji = luoji;
        this.rongqi = document.getElementById(rongqiid);
        this.dangqianyeshu = 1;
        this.meiyeshuliang = 10;
        this.sousuomoshi = false;
        this.guanjianci = '';
    }

    xuanran() {
        this.rongqi.innerHTML = '';
        const tou = document.createElement('div');
        tou.style.cssText = 'display:flex;justify-content:space-between;align-items:center;margin-bottom:16px;flex-wrap:wrap;gap:12px';
        tou.innerHTML = `<h2 style="font-size:18px;color:#0F172A;margin:0;font-weight:600">用户管理</h2>
            <div style="display:flex;gap:8px;flex-wrap:wrap">
                <div style="position:relative">
                    <svg style="position:absolute;left:12px;top:50%;transform:translateY(-50%);width:18px;height:18px;color:#64748B;pointer-events:none" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/></svg>
                    <input id="yh_sousuo" type="text" placeholder="搜索账号/昵称" style="border:1px solid #E2E8F0;border-radius:8px;padding:8px 12px 8px 38px;font-size:14px;outline:none;width:220px;transition:border-color 200ms;color:#1E293B" onfocus="this.style.borderColor='#3B82F6'" onblur="this.style.borderColor='#E2E8F0'">
                </div>
                <button class="aq-btn aq-btn-zhu" onclick="yonghu_sousuo()" style="cursor:pointer;transition:all 200ms">
                    <svg style="width:16px;height:16px;margin-right:4px;display:inline-block;vertical-align:middle" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/></svg>搜索
                </button>
                <button class="aq-btn" onclick="yonghu_qingkong()" style="cursor:pointer;transition:all 200ms">清空</button>
                <button class="aq-btn aq-btn-lv" onclick="yonghu_shuaxin()" style="cursor:pointer;transition:all 200ms">
                    <svg style="width:16px;height:16px;margin-right:4px;display:inline-block;vertical-align:middle" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>刷新
                </button>
            </div>`;
        this.rongqi.appendChild(tou);
        const neirong = document.createElement('div');
        neirong.id = 'yonghu_neirong';
        this.rongqi.appendChild(neirong);
    }

    async shuaxinliebiao() {
        const nr = document.getElementById('yonghu_neirong');
        nr.innerHTML = '<div style="display:flex;justify-content:center;align-items:center;padding:48px;color:#64748B"><svg style="width:24px;height:24px;margin-right:8px;animation:spin 1s linear infinite" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>加载中...</div><style>@keyframes spin{to{transform:rotate(360deg)}}</style>';
        const jg = this.sousuomoshi
            ? await this.luoji.sousuo(this.guanjianci, this.dangqianyeshu, this.meiyeshuliang)
            : await this.luoji.fenye(this.dangqianyeshu, this.meiyeshuliang);
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = `<div style="background:#FEF2F2;border:1px solid #FCA5A5;border-radius:12px;padding:16px;color:#991B1B;display:flex;align-items:center"><svg style="width:20px;height:20px;margin-right:8px;flex-shrink:0" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/></svg><span>加载失败: ${jg ? jg.xiaoxi : '请求错误'}</span></div>`;
            return;
        }
        const shuju = jg.shuju || {};
        const liebiao = shuju.liebiao || [];
        const zongshu = shuju.zongshu || 0;
        if (liebiao.length === 0) {
            nr.innerHTML = '<div style="text-align:center;padding:48px;color:#64748B"><svg style="width:64px;height:64px;margin:0 auto 16px;opacity:0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/></svg><p style="font-size:15px;margin:0">暂无用户数据</p></div>';
            return;
        }
        let html = '<div style="background:white;border:1px solid #E2E8F0;border-radius:12px;overflow:hidden;box-shadow:0 1px 3px rgba(0,0,0,0.05)"><table style="width:100%;border-collapse:collapse"><thead style="background:#F8FAFC;border-bottom:1px solid #E2E8F0"><tr>' +
            '<th style="padding:12px 16px;text-align:left;font-size:13px;font-weight:600;color:#475569">ID</th>' +
            '<th style="padding:12px 16px;text-align:left;font-size:13px;font-weight:600;color:#475569">账号</th>' +
            '<th style="padding:12px 16px;text-align:left;font-size:13px;font-weight:600;color:#475569">昵称</th>' +
            '<th style="padding:12px 16px;text-align:left;font-size:13px;font-weight:600;color:#475569">用户组</th>' +
            '<th style="padding:12px 16px;text-align:left;font-size:13px;font-weight:600;color:#475569">状态</th>' +
            '<th style="padding:12px 16px;text-align:left;font-size:13px;font-weight:600;color:#475569">创建时间</th>' +
            '<th style="padding:12px 16px;text-align:center;font-size:13px;font-weight:600;color:#475569">操作</th>' +
            '</tr></thead><tbody>';
        for (const yh of liebiao) {
            const zt = yh.zhuangtai === '1';
            const zthtml = zt
                ? '<span style="display:inline-flex;align-items:center;padding:4px 10px;background:#D1FAE5;color:#065F46;border-radius:6px;font-size:13px;font-weight:500"><svg style="width:14px;height:14px;margin-right:4px" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/></svg>正常</span>'
                : '<span style="display:inline-flex;align-items:center;padding:4px 10px;background:#FEE2E2;color:#991B1B;border-radius:6px;font-size:13px;font-weight:500"><svg style="width:14px;height:14px;margin-right:4px" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/></svg>禁用</span>';
            html += `<tr style="border-bottom:1px solid #F1F5F9;transition:background-color 150ms" onmouseover="this.style.backgroundColor='#F8FAFC'" onmouseout="this.style.backgroundColor='transparent'">
                <td style="padding:12px 16px;font-size:14px;color:#64748B">${yh.id}</td>
                <td style="padding:12px 16px;font-size:14px;color:#0F172A;font-weight:500">${yh.zhanghao}</td>
                <td style="padding:12px 16px;font-size:14px;color:#475569">${yh.nicheng || '-'}</td>
                <td style="padding:12px 16px;font-size:14px;color:#475569">${yh.yonghuzu || '-'}</td>
                <td style="padding:12px 16px">${zthtml}</td>
                <td style="padding:12px 16px;font-size:14px;color:#64748B">${yh.chuangjianshijian || '-'}</td>
                <td style="padding:12px 16px;text-align:center"><button class="aq-btn aq-btn-xiao aq-btn-zhu" onclick="yonghu_xiangqing('${yh.id}')" style="cursor:pointer;transition:all 200ms">详情</button></td>
            </tr>`;
        }
        html += '</tbody></table></div>';
        html += this.xuanranfenye(zongshu);
        nr.innerHTML = html;
    }

    xuanranfenye(zongshu) {
        const zongyeshu = Math.ceil(zongshu / this.meiyeshuliang);
        if (zongyeshu <= 1) return '';
        const shangyijinyong = this.dangqianyeshu <= 1;
        const xiayijinyong = this.dangqianyeshu >= zongyeshu;
        let html = '<div style="display:flex;justify-content:center;align-items:center;gap:12px;margin-top:20px;padding:16px;background:#F8FAFC;border-radius:12px">';
        html += `<button onclick="yonghu_shangyiye()" ${shangyijinyong ? 'disabled' : ''} style="display:inline-flex;align-items:center;gap:6px;padding:8px 16px;background:${shangyijinyong ? '#F1F5F9' : 'white'};color:${shangyijinyong ? '#94A3B8' : '#475569'};border:1px solid #E2E8F0;border-radius:8px;font-size:14px;font-weight:500;cursor:${shangyijinyong ? 'not-allowed' : 'pointer'};transition:all 200ms;box-shadow:0 1px 2px rgba(0,0,0,0.05)" ${shangyijinyong ? '' : 'onmouseover="this.style.borderColor=\'#3B82F6\';this.style.color=\'#3B82F6\'" onmouseout="this.style.borderColor=\'#E2E8F0\';this.style.color=\'#475569\'"'}><svg style="width:16px;height:16px" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/></svg>上一页</button>`;
        html += `<div style="display:flex;align-items:center;gap:8px"><span style="color:#64748B;font-size:14px">第</span><span style="display:inline-flex;align-items:center;justify-content:center;min-width:32px;height:32px;padding:0 8px;background:white;color:#3B82F6;border:1px solid #3B82F6;border-radius:8px;font-size:14px;font-weight:600">${this.dangqianyeshu}</span><span style="color:#64748B;font-size:14px">/ ${zongyeshu} 页</span><span style="color:#94A3B8;font-size:13px;margin-left:4px">(共 ${zongshu} 条)</span></div>`;
        html += `<button onclick="yonghu_xiayiye()" ${xiayijinyong ? 'disabled' : ''} style="display:inline-flex;align-items:center;gap:6px;padding:8px 16px;background:${xiayijinyong ? '#F1F5F9' : 'white'};color:${xiayijinyong ? '#94A3B8' : '#475569'};border:1px solid #E2E8F0;border-radius:8px;font-size:14px;font-weight:500;cursor:${xiayijinyong ? 'not-allowed' : 'pointer'};transition:all 200ms;box-shadow:0 1px 2px rgba(0,0,0,0.05)" ${xiayijinyong ? '' : 'onmouseover="this.style.borderColor=\'#3B82F6\';this.style.color=\'#3B82F6\'" onmouseout="this.style.borderColor=\'#E2E8F0\';this.style.color=\'#475569\'"'}>下一页<svg style="width:16px;height:16px" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg></button>`;
        html += '</div>';
        return html;
    }

    async xiangqing(id) {
        const nr = document.getElementById('yonghu_neirong');
        nr.innerHTML = '<div style="display:flex;justify-content:center;align-items:center;padding:48px;color:#64748B"><svg style="width:24px;height:24px;margin-right:8px;animation:spin 1s linear infinite" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>加载中...</div>';
        const jg = await this.luoji.xiangqing(id);
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = `<div style="background:#FEF2F2;border:1px solid #FCA5A5;border-radius:12px;padding:16px;color:#991B1B;display:flex;align-items:center"><svg style="width:20px;height:20px;margin-right:8px;flex-shrink:0" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/></svg><span>加载失败: ${jg ? jg.xiaoxi : '请求错误'}</span></div>`;
            return;
        }
        const yh = jg.shuju || {};
        const zt = yh.zhuangtai === '1';
        const zthtml = zt
            ? '<span style="display:inline-flex;align-items:center;padding:6px 12px;background:#D1FAE5;color:#065F46;border-radius:8px;font-size:14px;font-weight:500"><svg style="width:16px;height:16px;margin-right:6px" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/></svg>正常</span>'
            : '<span style="display:inline-flex;align-items:center;padding:6px 12px;background:#FEE2E2;color:#991B1B;border-radius:8px;font-size:14px;font-weight:500"><svg style="width:16px;height:16px;margin-right:6px" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/></svg>禁用</span>';
        let html = '<div style="background:white;border:1px solid #E2E8F0;border-radius:12px;padding:24px;box-shadow:0 1px 3px rgba(0,0,0,0.05);max-width:700px">';
        html += '<div style="display:flex;align-items:center;margin-bottom:24px;padding-bottom:16px;border-bottom:1px solid #E2E8F0"><svg style="width:24px;height:24px;color:#3B82F6;margin-right:12px" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/></svg><h3 style="font-size:18px;font-weight:600;color:#0F172A;margin:0">用户详情</h3></div>';
        html += '<div style="display:grid;gap:16px">';
        html += `<div style="display:grid;grid-template-columns:120px 1fr;gap:12px;align-items:center"><span style="font-size:14px;font-weight:500;color:#64748B">ID</span><span style="font-size:14px;color:#0F172A">${yh.id}</span></div>`;
        html += `<div style="display:grid;grid-template-columns:120px 1fr;gap:12px;align-items:center;padding:12px;background:#F8FAFC;border-radius:8px"><span style="font-size:14px;font-weight:500;color:#64748B">账号</span><span style="font-size:15px;color:#0F172A;font-weight:600">${yh.zhanghao}</span></div>`;
        html += `<div style="display:grid;grid-template-columns:120px 1fr;gap:12px;align-items:center"><span style="font-size:14px;font-weight:500;color:#64748B">昵称</span><span style="font-size:14px;color:#475569">${yh.nicheng || '-'}</span></div>`;
        html += `<div style="display:grid;grid-template-columns:120px 1fr;gap:12px;align-items:center;padding:12px;background:#F8FAFC;border-radius:8px"><span style="font-size:14px;font-weight:500;color:#64748B">用户组</span><span style="font-size:14px;color:#475569">${yh.yonghuzu || '-'}</span></div>`;
        html += `<div style="display:grid;grid-template-columns:120px 1fr;gap:12px;align-items:center"><span style="font-size:14px;font-weight:500;color:#64748B">状态</span>${zthtml}</div>`;
        html += `<div style="display:grid;grid-template-columns:120px 1fr;gap:12px;align-items:center;padding:12px;background:#F8FAFC;border-radius:8px"><span style="font-size:14px;font-weight:500;color:#64748B">创建时间</span><span style="font-size:14px;color:#475569">${yh.chuangjianshijian || '-'}</span></div>`;
        html += `<div style="display:grid;grid-template-columns:120px 1fr;gap:12px;align-items:center"><span style="font-size:14px;font-weight:500;color:#64748B">更新时间</span><span style="font-size:14px;color:#475569">${yh.gengxinshijian || '-'}</span></div>`;
        html += '</div>';
        html += '<div style="margin-top:24px;padding-top:16px;border-top:1px solid #E2E8F0"><button onclick="yonghu_shuaxin()" style="display:inline-flex;align-items:center;gap:6px;padding:10px 20px;background:white;color:#475569;border:1px solid #E2E8F0;border-radius:8px;font-size:14px;font-weight:500;cursor:pointer;transition:all 200ms;box-shadow:0 1px 2px rgba(0,0,0,0.05)" onmouseover="this.style.borderColor=\'#3B82F6\';this.style.color=\'#3B82F6\'" onmouseout="this.style.borderColor=\'#E2E8F0\';this.style.color=\'#475569\'"><svg style="width:16px;height:16px" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"/></svg>返回列表</button></div>';
        html += '</div>';
        nr.innerHTML = html;
    }

    async sousuo() {
        const shuru = document.getElementById('yh_sousuo');
        const gjc = shuru?.value?.trim() || '';
        if (!gjc) {
            this.luoji.rizhi('请输入搜索关键词', 'warn');
            return;
        }
        this.sousuomoshi = true;
        this.guanjianci = gjc;
        this.dangqianyeshu = 1;
        await this.shuaxinliebiao();
    }

    qingkong() {
        const shuru = document.getElementById('yh_sousuo');
        if (shuru) shuru.value = '';
        this.sousuomoshi = false;
        this.guanjianci = '';
        this.dangqianyeshu = 1;
        this.shuaxinliebiao();
    }

    async shangyiye() {
        if (this.dangqianyeshu > 1) {
            this.dangqianyeshu--;
            await this.shuaxinliebiao();
        }
    }

    async xiayiye() {
        this.dangqianyeshu++;
        await this.shuaxinliebiao();
    }
}
