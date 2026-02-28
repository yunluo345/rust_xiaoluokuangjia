// 用户管理 - 界面层
import * as gj from './jiemian_gongju.js';

export class Yonghujiemian {
    constructor(luoji, rongqiid) {
        this.luoji = luoji;
        this.rongqi = document.getElementById(rongqiid);
        this.dangqianshitu = 'yonghu';
        this.yh_ye = 1;
        this.yh_liang = 10;
        this.yh_sousuomoshi = false;
        this.yh_gjc = '';
        this.zu_ye = 1;
        this.zu_liang = 10;
        this.zu_sousuomoshi = false;
        this.zu_gjc = '';
    }

    _zhuanyi(str) {
        if (!str) return '';
        return str.replace(/&/g, '&amp;').replace(/"/g, '&quot;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }

    _geshihuashijian(zhi) {
        if (!zhi) return '-';
        const shu = Number(zhi);
        if (isNaN(shu) || shu <= 0) return String(zhi);
        const rq = new Date(shu);
        if (isNaN(rq.getTime())) return String(zhi);
        const bu = (n) => String(n).padStart(2, '0');
        return `${rq.getFullYear()}-${bu(rq.getMonth() + 1)}-${bu(rq.getDate())} ${bu(rq.getHours())}:${bu(rq.getMinutes())}:${bu(rq.getSeconds())}`;
    }

    _jiazaizhong() {
        return '<div style="display:flex;justify-content:center;align-items:center;padding:48px;color:#64748B"><svg style="width:24px;height:24px;margin-right:8px;animation:spin 1s linear infinite" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>加载中...</div><style>@keyframes spin{to{transform:rotate(360deg)}}</style>';
    }

    _cuowuhtml(xiaoxi) {
        return `<div style="background:#FEF2F2;border:1px solid #FCA5A5;border-radius:12px;padding:16px;color:#991B1B;display:flex;align-items:center"><svg style="width:20px;height:20px;margin-right:8px;flex-shrink:0" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/></svg><span>${xiaoxi}</span></div>`;
    }

    _konghtml(wenben) {
        return `<div style="text-align:center;padding:48px;color:#64748B"><svg style="width:64px;height:64px;margin:0 auto 16px;opacity:0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/></svg><p style="font-size:15px;margin:0">${wenben}</p></div>`;
    }

    _hanghtml(biaoti, neirong, beijing) {
        const bg = beijing ? 'padding:12px;background:#F8FAFC;border-radius:8px' : 'padding:12px';
        return `<div style="display:grid;grid-template-columns:100px 1fr;gap:12px;align-items:center;min-height:42px;${bg}"><span style="font-size:14px;font-weight:500;color:#64748B">${biaoti}</span><div style="font-size:14px;color:#1E293B;line-height:30px">${neirong}</div></div>`;
    }

    _bianji_hang(biaoti, inputid, zhi, beijing, leixing) {
        const bg = beijing ? 'padding:12px;background:#F8FAFC;border-radius:8px' : 'padding:12px';
        const shuruleixing = leixing || 'text';
        return `<div style="display:grid;grid-template-columns:100px 1fr;gap:12px;align-items:center;min-height:42px;${bg}"><span style="font-size:14px;font-weight:500;color:#64748B">${biaoti}</span><div style="display:flex;gap:8px;align-items:center"><input id="${inputid}" type="${shuruleixing}" value="${this._zhuanyi(zhi)}" placeholder="输入${biaoti}" style="flex:1;border:1px solid #E2E8F0;border-radius:6px;padding:6px 10px;font-size:14px;outline:none;color:#1E293B;transition:border-color 200ms" onfocus="this.style.borderColor='#3B82F6'" onblur="this.style.borderColor='#E2E8F0'"></div></div>`;
    }

    _fanhuianniu(fn) {
        return `<button onclick="${fn}" style="display:inline-flex;align-items:center;gap:6px;padding:10px 20px;background:white;color:#475569;border:1px solid #E2E8F0;border-radius:8px;font-size:14px;font-weight:500;cursor:pointer;transition:all 200ms;box-shadow:0 1px 2px rgba(0,0,0,0.05)" onmouseenter="this.style.borderColor='#3B82F6';this.style.color='#3B82F6'" onmouseleave="this.style.borderColor='#E2E8F0';this.style.color='#475569'"><svg style="width:16px;height:16px" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"/></svg>返回列表</button>`;
    }

    _xuanranfenye(zongshu, liang, ye, shangyifn, xiayifn) {
        const zongyeshu = Math.ceil(zongshu / liang);
        if (zongyeshu <= 1) return '';
        const syjy = ye <= 1, xyjy = ye >= zongyeshu;
        const yang = (jy) => `display:inline-flex;align-items:center;gap:6px;padding:8px 16px;background:${jy ? '#F1F5F9' : 'white'};color:${jy ? '#94A3B8' : '#475569'};border:1px solid #E2E8F0;border-radius:8px;font-size:14px;font-weight:500;cursor:${jy ? 'not-allowed' : 'pointer'};transition:all 200ms;box-shadow:0 1px 2px rgba(0,0,0,0.05)`;
        const hv = `onmouseenter="this.style.borderColor='#3B82F6';this.style.color='#3B82F6'" onmouseleave="this.style.borderColor='#E2E8F0';this.style.color='#475569'"`;
        return `<div style="display:flex;justify-content:center;align-items:center;gap:12px;margin-top:20px;padding:16px;background:#F8FAFC;border-radius:12px"><button onclick="${shangyifn}" ${syjy ? 'disabled' : ''} style="${yang(syjy)}" ${syjy ? '' : hv}><svg style="width:16px;height:16px" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/></svg>上一页</button><div style="display:flex;align-items:center;gap:8px"><span style="color:#64748B;font-size:14px">第</span><span style="display:inline-flex;align-items:center;justify-content:center;min-width:32px;height:32px;padding:0 8px;background:white;color:#3B82F6;border:1px solid #3B82F6;border-radius:8px;font-size:14px;font-weight:600">${ye}</span><span style="color:#64748B;font-size:14px">/ ${zongyeshu} 页</span><span style="color:#94A3B8;font-size:13px;margin-left:4px">(共 ${zongshu} 条)</span></div><button onclick="${xiayifn}" ${xyjy ? 'disabled' : ''} style="${yang(xyjy)}" ${xyjy ? '' : hv}>下一页<svg style="width:16px;height:16px" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg></button></div>`;
    }

    _sousuolan(inputid, placeholder, sousuofn, qingkongfn, shuaxinfn, xinzengfn, xinzengwen) {
        return `<div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:16px;flex-wrap:wrap;gap:12px"><div style="display:flex;gap:8px;flex-wrap:wrap;align-items:center"><div style="position:relative"><svg style="position:absolute;left:12px;top:50%;transform:translateY(-50%);width:18px;height:18px;color:#64748B;pointer-events:none" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/></svg><input id="${inputid}" type="text" placeholder="${placeholder}" style="border:1px solid #E2E8F0;border-radius:8px;padding:8px 12px 8px 38px;font-size:14px;outline:none;width:220px;transition:border-color 200ms;color:#1E293B" onfocus="this.style.borderColor='#3B82F6'" onblur="this.style.borderColor='#E2E8F0'"></div><button class="aq-btn aq-btn-zhu" onclick="${sousuofn}" style="cursor:pointer;transition:all 200ms">搜索</button><button class="aq-btn" onclick="${qingkongfn}" style="cursor:pointer;transition:all 200ms">清空</button><button class="aq-btn aq-btn-lv" onclick="${shuaxinfn}" style="cursor:pointer;transition:all 200ms">刷新</button></div><button class="aq-btn aq-btn-zhu" onclick="${xinzengfn}" style="cursor:pointer;transition:all 200ms">${xinzengwen}</button></div>`;
    }

    // 视图切换渲染
    xuanran() {
        this.rongqi.innerHTML = '';
        const tou = document.createElement('div');
        tou.style.cssText = 'margin-bottom:16px';
        const shiyh = this.dangqianshitu === 'yonghu';
        const tabyang = (xuanzhong) => `padding:10px 24px;font-size:14px;font-weight:${xuanzhong ? '600' : '400'};color:${xuanzhong ? '#3B82F6' : '#64748B'};background:${xuanzhong ? 'white' : '#F1F5F9'};border:1px solid ${xuanzhong ? '#3B82F6' : '#E2E8F0'};cursor:pointer;transition:all 200ms`;
        tou.innerHTML = `<div style="display:flex;gap:0"><button onclick="yonghu_qiehuanshitu('yonghu')" style="${tabyang(shiyh)};border-radius:8px 0 0 8px">用户列表</button><button onclick="yonghu_qiehuanshitu('yonghuzu')" style="${tabyang(!shiyh)};border-radius:0 8px 8px 0">用户组列表</button></div>`;
        this.rongqi.appendChild(tou);
        const neirong = document.createElement('div');
        neirong.id = 'yonghu_neirong';
        const shiyonghu = this.dangqianshitu === 'yonghu';
        const sousuolan = shiyonghu
            ? this._sousuolan('yh_sousuo', '搜索账号/昵称', 'yonghu_sousuo()', 'yonghu_qingkong()', 'yonghu_shuaxin()', 'yonghu_xinzeng_shitu()', '新增用户')
            : this._sousuolan('zu_sousuo', '搜索组名称', 'yonghuzu_sousuo()', 'yonghuzu_qingkong()', 'yonghuzu_shuaxin()', 'yonghuzu_xinzeng_shitu()', '新增用户组');
         neirong.innerHTML = sousuolan + '<div id="' + (shiyonghu ? 'yh_lb' : 'zu_lb') + '"></div>';
         this.rongqi.appendChild(neirong);
     }

    qiehuanshitu(shitu) {
        this.dangqianshitu = shitu;
        this.xuanran();
    }

    // ========== 用户列表 ==========
    async shuaxinliebiao() {
        const nr = document.getElementById('yonghu_neirong');
        nr.innerHTML = this._sousuolan('yh_sousuo', '搜索账号/昵称', 'yonghu_sousuo()', 'yonghu_qingkong()', 'yonghu_shuaxin()', 'yonghu_xinzeng_shitu()', '新增用户') + '<div id="yh_lb">' + this._jiazaizhong() + '</div>';
        const jg = this.yh_sousuomoshi
            ? await this.luoji.sousuo(this.yh_gjc, this.yh_ye, this.yh_liang)
            : await this.luoji.fenye(this.yh_ye, this.yh_liang);
        const lb = document.getElementById('yh_lb');
        if (!lb) return;
        if (!jg || jg.zhuangtaima !== 200) { lb.innerHTML = this._cuowuhtml('加载失败: ' + (jg ? jg.xiaoxi : '请求错误')); return; }
        const liebiao = jg.shuju?.liebiao || [], zongshu = jg.shuju?.zongshu || 0;
        if (!liebiao.length) { lb.innerHTML = this._konghtml('暂无用户数据'); return; }
        const th = 'padding:12px 16px;text-align:left;font-size:13px;font-weight:600;color:#475569';
        let html = '<div style="display:flex;gap:8px;align-items:center;margin-bottom:10px"><button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="yonghu_piliangshanchu()" style="cursor:pointer">批量删除</button></div>';
        html += `<div style="background:white;border:1px solid #E2E8F0;border-radius:12px;overflow:hidden;box-shadow:0 1px 3px rgba(0,0,0,0.05)"><table style="width:100%;border-collapse:collapse"><thead style="background:#F8FAFC;border-bottom:1px solid #E2E8F0"><tr><th style="${th};width:40px"><input type="checkbox" onchange="yonghu_quanxuan(this)" style="width:16px;height:16px;cursor:pointer"></th><th style="${th}">ID</th><th style="${th}">账号</th><th style="${th}">昵称</th><th style="${th}">用户组</th><th style="${th}">状态</th><th style="${th}">创建时间</th><th style="${th};text-align:center">操作</th></tr></thead><tbody>`;
        for (const yh of liebiao) {
            const yifj = yh.fengjin === '1';
            const zthtml = yifj
                ? '<span style="display:inline-flex;align-items:center;padding:4px 10px;background:#FEE2E2;color:#991B1B;border-radius:6px;font-size:13px;font-weight:500">已封禁</span>'
                : '<span style="display:inline-flex;align-items:center;padding:4px 10px;background:#D1FAE5;color:#065F46;border-radius:6px;font-size:13px;font-weight:500">正常</span>';
            html += `<tr style="border-bottom:1px solid #F1F5F9;transition:background-color 150ms" onmouseenter="this.style.backgroundColor='#F8FAFC'" onmouseleave="this.style.backgroundColor='transparent'">
                <td style="padding:12px 16px"><input type="checkbox" class="yh_pl_xz" data-id="${yh.id}" style="width:16px;height:16px;cursor:pointer"></td>
                <td style="padding:12px 16px;font-size:14px;color:#64748B">${yh.id}</td>
                <td style="padding:12px 16px;font-size:14px;color:#0F172A;font-weight:500">${yh.zhanghao}</td>
                <td style="padding:12px 16px;font-size:14px;color:#475569">${yh.nicheng || '-'}</td>
                <td style="padding:12px 16px;font-size:14px;color:#475569">${yh.yonghuzu || yh.yonghuzuid || '-'}</td>
                <td style="padding:12px 16px">${zthtml}</td>
                <td style="padding:12px 16px;font-size:14px;color:#64748B">${this._geshihuashijian(yh.chuangjianshijian)}</td>
                <td style="padding:12px 16px;text-align:center"><button class="aq-btn aq-btn-xiao aq-btn-zhu" onclick="yonghu_xiangqing('${yh.id}')" style="cursor:pointer;transition:all 200ms">详情</button></td>
            </tr>`;
        }
        html += '</tbody></table></div>';
        html += this._xuanranfenye(zongshu, this.yh_liang, this.yh_ye, 'yonghu_shangyiye()', 'yonghu_xiayiye()');
        lb.innerHTML = html;
    }

    async xiangqing(id) {
        const nr = document.getElementById('yonghu_neirong');
        nr.innerHTML = this._jiazaizhong();
        const jg = await this.luoji.xiangqing(id);
        if (!jg || jg.zhuangtaima !== 200) { nr.innerHTML = this._cuowuhtml('加载失败: ' + (jg ? jg.xiaoxi : '请求错误')); return; }
        const yh = jg.shuju || {}, yifj = yh.fengjin === '1';
        let html = '<div style="background:white;border:1px solid #E2E8F0;border-radius:12px;padding:24px;box-shadow:0 1px 3px rgba(0,0,0,0.05);max-width:700px">';
        html += '<div style="display:flex;align-items:center;margin-bottom:24px;padding-bottom:16px;border-bottom:1px solid #E2E8F0"><svg style="width:24px;height:24px;color:#3B82F6;margin-right:12px" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/></svg><h3 style="font-size:18px;font-weight:600;color:#0F172A;margin:0">用户详情</h3></div>';
        html += '<div style="display:grid;gap:16px">';
        html += this._hanghtml('ID', `<span style="font-size:14px;color:#0F172A">${yh.id}</span>`, false);
        html += this._bianji_hang('账号', 'yh_xg_zhanghao', yh.zhanghao || '', true);
        html += this._bianji_hang('昵称', 'yh_xg_nicheng', yh.nicheng || '', false);
        html += this._bianji_hang('密码', 'yh_xg_mima', '', true, 'password');
        html += this._bianji_hang('备注', 'yh_xg_beizhu', yh.beizhu || '', false);
        html += this._hanghtml('用户组', `<span style="font-size:14px;color:#475569">${yh.yonghuzuid || '-'}</span>`, true);
        html += this._hanghtml('封禁状态', yifj ? '<span style="padding:6px 12px;background:#FEE2E2;color:#991B1B;border-radius:8px;font-size:14px;font-weight:500">已封禁</span>' : '<span style="padding:6px 12px;background:#D1FAE5;color:#065F46;border-radius:8px;font-size:14px;font-weight:500">正常</span>', false);
        if (yifj) {
            html += this._hanghtml('封禁原因', `<span style="font-size:14px;color:#991B1B">${this._zhuanyi(yh.fengjinyuanyin) || '-'}</span>`, true);
            html += this._hanghtml('封禁结束', `<span style="font-size:14px;color:#475569">${yh.fengjinjieshu ? this._geshihuashijian(yh.fengjinjieshu) : '永久'}</span>`, false);
        }
        html += this._hanghtml('创建时间', `<span style="font-size:14px;color:#475569">${this._geshihuashijian(yh.chuangjianshijian)}</span>`, true);
        html += this._hanghtml('更新时间', `<span style="font-size:14px;color:#475569">${this._geshihuashijian(yh.gengxinshijian)}</span>`, false);
        html += '</div>';
        html += `<div style="margin-top:20px;display:flex;justify-content:flex-end"><button class="aq-btn aq-btn-zhu" onclick="yonghu_tongyi_baocun('${yh.id}')" style="cursor:pointer;transition:all 200ms;padding:10px 28px;font-size:15px;font-weight:500;display:inline-flex;align-items:center;gap:6px"><svg style="width:18px;height:18px" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>保存修改</button></div>`;
        html += '<div style="margin-top:24px;padding-top:16px;border-top:1px solid #E2E8F0">';
        if (yifj) {
            html += `<div style="display:flex;align-items:center;gap:12px;margin-bottom:16px"><button class="aq-btn aq-btn-xiao aq-btn-lv" onclick="yonghu_jiefeng('${yh.id}')" style="cursor:pointer">解封用户</button></div>`;
        } else {
            html += '<div style="margin-bottom:16px"><div style="font-size:14px;font-weight:500;color:#64748B;margin-bottom:8px">封禁用户</div><div style="display:flex;gap:8px;align-items:center;flex-wrap:wrap">';
            html += '<input id="yh_fj_yuanyin" type="text" placeholder="封禁原因（必填）" style="flex:1;min-width:150px;border:1px solid #E2E8F0;border-radius:6px;padding:6px 10px;font-size:14px;outline:none;color:#1E293B;transition:border-color 200ms" onfocus="this.style.borderColor=\'#EF4444\'" onblur="this.style.borderColor=\'#E2E8F0\'">';
            html += '<input id="yh_fj_jieshu" type="datetime-local" style="width:200px;border:1px solid #E2E8F0;border-radius:6px;padding:6px 10px;font-size:14px;outline:none;color:#1E293B;transition:border-color 200ms" onfocus="this.style.borderColor=\'#EF4444\'" onblur="this.style.borderColor=\'#E2E8F0\'">';
            html += `<button class="aq-btn aq-btn-xiao" onclick="yonghu_fengjin('${yh.id}')" style="cursor:pointer;background:#FEE2E2;color:#991B1B;border-color:#FCA5A5">封禁</button></div></div>`;
        }
        html += `<div style="display:flex;gap:12px;align-items:center">${this._fanhuianniu('yonghu_shuaxin()')}<button class="aq-btn aq-btn-hong" onclick="yonghu_shanchu('${yh.id}')" style="cursor:pointer;transition:all 200ms">删除用户</button></div>`;
        html += '</div></div>';
        nr.innerHTML = html;
    }

    async xinzeng_shitu() {
        const nr = document.getElementById('yonghu_neirong');
        nr.innerHTML = this._jiazaizhong();
        const zuliebiao = await this._huoquzuliebiao();
        const xuanxiang = zuliebiao.map(z => `<option value="${z.id}">${this._zhuanyi(z.mingcheng)}</option>`).join('');
        let html = '<div style="background:white;border:1px solid #E2E8F0;border-radius:12px;padding:24px;box-shadow:0 1px 3px rgba(0,0,0,0.05);max-width:500px">';
        html += '<h3 style="font-size:18px;font-weight:600;color:#0F172A;margin:0 0 20px">新增用户</h3><div style="display:grid;gap:12px">';
        html += this._bianji_hang('账号', 'yh_xz_zhanghao', '', false);
        html += this._bianji_hang('密码', 'yh_xz_mima', '', true, 'password');
        html += this._bianji_hang('昵称', 'yh_xz_nicheng', '', false);
        html += `<div style="display:grid;grid-template-columns:100px 1fr;gap:12px;align-items:center;min-height:42px;padding:12px;background:#F8FAFC;border-radius:8px"><span style="font-size:14px;font-weight:500;color:#64748B">用户组</span><select id="yh_xz_yonghuzuid" style="flex:1;border:1px solid #E2E8F0;border-radius:6px;padding:6px 10px;font-size:14px;outline:none;color:#1E293B">${xuanxiang}</select></div>`;
        html += this._bianji_hang('备注', 'yh_xz_beizhu', '', false);
        html += `<div style="display:flex;gap:12px;margin-top:8px"><button class="aq-btn aq-btn-zhu" onclick="yonghu_tijiaoxinzeng()" style="cursor:pointer">提交</button>${this._fanhuianniu('yonghu_shuaxin()')}</div>`;
        html += '</div></div>';
        nr.innerHTML = html;
    }

    async tijiaoxinzeng() {
        const zhanghao = document.getElementById('yh_xz_zhanghao')?.value?.trim();
        const mima = document.getElementById('yh_xz_mima')?.value?.trim();
        const nicheng = document.getElementById('yh_xz_nicheng')?.value?.trim();
        const yonghuzuid = document.getElementById('yh_xz_yonghuzuid')?.value;
        const beizhu = document.getElementById('yh_xz_beizhu')?.value?.trim() || '';
        if (!zhanghao || !mima || !nicheng || !yonghuzuid) { this.luoji.rizhi('请填写所有必填项', 'warn'); return; }
        const jg = await this.luoji.xinzeng(zhanghao, mima, nicheng, yonghuzuid, beizhu);
        if (jg && jg.zhuangtaima === 200) await this.shuaxinliebiao();
    }

    async shanchu_yonghu(id) {
        const queren = await window.aqqueren('删除用户', `确定要删除用户 ID:${id} 吗？此操作不可撤销。`);
        if (!queren) return;
        const jg = await this.luoji.shanchu(id);
        if (jg && jg.zhuangtaima === 200) await this.shuaxinliebiao();
    }

    async tongyi_baocun(id) {
        const zhanghao = document.getElementById('yh_xg_zhanghao')?.value?.trim();
        const nicheng = document.getElementById('yh_xg_nicheng')?.value?.trim();
        const mima = document.getElementById('yh_xg_mima')?.value?.trim();
        const beizhu = document.getElementById('yh_xg_beizhu')?.value ?? '';
        if (!zhanghao) { this.luoji.rizhi('账号不能为空', 'warn'); return; }
        if (!nicheng) { this.luoji.rizhi('昵称不能为空', 'warn'); return; }
        let chenggong = true;
        const jg1 = await this.luoji.xiugai_zhanghao(id, zhanghao);
        if (!jg1 || jg1.zhuangtaima !== 200) chenggong = false;
        const jg2 = await this.luoji.xiugai_nicheng(id, nicheng);
        if (!jg2 || jg2.zhuangtaima !== 200) chenggong = false;
        if (mima) { const jg3 = await this.luoji.xiugai_mima(id, mima); if (!jg3 || jg3.zhuangtaima !== 200) chenggong = false; }
        const jg4 = await this.luoji.xiugai_beizhu(id, beizhu);
        if (!jg4 || jg4.zhuangtaima !== 200) chenggong = false;
        this.luoji.rizhi(chenggong ? '保存成功' : '部分保存失败，请检查', chenggong ? 'ok' : 'warn');
        await this.xiangqing(id);
    }

    async fengjin_yonghu(id) {
        const yuanyin = document.getElementById('yh_fj_yuanyin')?.value?.trim();
        if (!yuanyin) { this.luoji.rizhi('封禁原因不能为空', 'warn'); return; }
        const jieshuval = document.getElementById('yh_fj_jieshu')?.value;
        const jieshu = jieshuval ? String(new Date(jieshuval).getTime()) : undefined;
        const jg = await this.luoji.fengjin(id, yuanyin, jieshu);
        if (jg && jg.zhuangtaima === 200) await this.xiangqing(id);
    }

    async jiefeng_yonghu(id) {
        const jg = await this.luoji.jiefeng(id);
        if (jg && jg.zhuangtaima === 200) await this.xiangqing(id);
    }

    async sousuo() {
        const gjc = document.getElementById('yh_sousuo')?.value?.trim() || '';
        if (!gjc) { this.luoji.rizhi('请输入搜索关键词', 'warn'); return; }
        this.yh_sousuomoshi = true; this.yh_gjc = gjc; this.yh_ye = 1;
        await this.shuaxinliebiao();
    }

    qingkong() { this.yh_sousuomoshi = false; this.yh_gjc = ''; this.yh_ye = 1; this.shuaxinliebiao(); }
    async shangyiye() { if (this.yh_ye > 1) { this.yh_ye--; await this.shuaxinliebiao(); } }
    async xiayiye() { this.yh_ye++; await this.shuaxinliebiao(); }

    async _huoquzuliebiao() {
        const jg = await this.luoji.yonghuzu_fenye(1, 100);
        return (jg && jg.zhuangtaima === 200) ? (jg.shuju?.liebiao || []) : [];
    }

    // ========== 用户组列表 ==========
    async shuaxinyonghuzuliebiao() {
        const nr = document.getElementById('yonghu_neirong');
        nr.innerHTML = this._sousuolan('zu_sousuo', '搜索组名称', 'yonghuzu_sousuo()', 'yonghuzu_qingkong()', 'yonghuzu_shuaxin()', 'yonghuzu_xinzeng_shitu()', '新增用户组') + '<div id="zu_lb">' + this._jiazaizhong() + '</div>';
        const jg = this.zu_sousuomoshi
            ? await this.luoji.yonghuzu_sousuo(this.zu_gjc, this.zu_ye, this.zu_liang)
            : await this.luoji.yonghuzu_fenye(this.zu_ye, this.zu_liang);
        const lb = document.getElementById('zu_lb');
        if (!lb) return;
        if (!jg || jg.zhuangtaima !== 200) { lb.innerHTML = this._cuowuhtml('加载失败: ' + (jg ? jg.xiaoxi : '请求错误')); return; }
        const liebiao = jg.shuju?.liebiao || [], zongshu = jg.shuju?.zongshu || 0;
        if (!liebiao.length) { lb.innerHTML = this._konghtml('暂无用户组数据'); return; }
        const th = 'padding:12px 16px;text-align:left;font-size:13px;font-weight:600;color:#475569';
        let html = '<div style="display:flex;gap:8px;align-items:center;margin-bottom:10px"><button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="yonghuzu_piliangshanchu()" style="cursor:pointer">批量删除</button></div>';
        html += `<div style="background:white;border:1px solid #E2E8F0;border-radius:12px;overflow:hidden;box-shadow:0 1px 3px rgba(0,0,0,0.05)"><table style="width:100%;border-collapse:collapse"><thead style="background:#F8FAFC;border-bottom:1px solid #E2E8F0"><tr><th style="${th};width:40px"><input type="checkbox" onchange="yonghuzu_quanxuan(this)" style="width:16px;height:16px;cursor:pointer"></th><th style="${th}">ID</th><th style="${th}">名称</th><th style="${th}">默认组</th><th style="${th}">备注</th><th style="${th}">创建时间</th><th style="${th};text-align:center">操作</th></tr></thead><tbody>`;
        for (const zu of liebiao) {
            const morenzhu = zu.morenzhu === '1'
                ? '<span style="padding:4px 10px;background:#DBEAFE;color:#1E40AF;border-radius:6px;font-size:13px;font-weight:500">是</span>'
                : '<span style="padding:4px 10px;background:#F1F5F9;color:#64748B;border-radius:6px;font-size:13px">否</span>';
            html += `<tr style="border-bottom:1px solid #F1F5F9;transition:background-color 150ms" onmouseenter="this.style.backgroundColor='#F8FAFC'" onmouseleave="this.style.backgroundColor='transparent'">
                <td style="padding:12px 16px"><input type="checkbox" class="zu_pl_xz" data-id="${zu.id}" style="width:16px;height:16px;cursor:pointer"></td>
                <td style="padding:12px 16px;font-size:14px;color:#64748B">${zu.id}</td>
                <td style="padding:12px 16px;font-size:14px;color:#0F172A;font-weight:500">${this._zhuanyi(zu.mingcheng)}</td>
                <td style="padding:12px 16px">${morenzhu}</td>
                <td style="padding:12px 16px;font-size:14px;color:#475569">${this._zhuanyi(zu.beizhu) || '-'}</td>
                <td style="padding:12px 16px;font-size:14px;color:#64748B">${this._geshihuashijian(zu.chuangjianshijian)}</td>
                <td style="padding:12px 16px;text-align:center"><button class="aq-btn aq-btn-xiao aq-btn-zhu" onclick="yonghuzu_bianji('${zu.id}')" style="cursor:pointer">编辑</button> <button class="aq-btn aq-btn-xiao aq-btn-hong" onclick="yonghuzu_shanchu('${zu.id}')" style="cursor:pointer">删除</button></td>
            </tr>`;
        }
        html += '</tbody></table></div>';
        html += this._xuanranfenye(zongshu, this.zu_liang, this.zu_ye, 'yonghuzu_shangyiye()', 'yonghuzu_xiayiye()');
        lb.innerHTML = html;
    }

    async yonghuzu_xinzeng_shitu() {
        const nr = document.getElementById('yonghu_neirong');
        nr.innerHTML = this._jiazaizhong();
        const zuliebiao = await this._huoquzuliebiao();
        const jichengxuanxiang = zuliebiao.map(z => `<option value="${z.id}">${this._zhuanyi(z.mingcheng)}</option>`).join('');
        let html = '<div style="background:white;border:1px solid #E2E8F0;border-radius:12px;padding:24px;box-shadow:0 1px 3px rgba(0,0,0,0.05);max-width:500px">';
        html += '<h3 style="font-size:18px;font-weight:600;color:#0F172A;margin:0 0 20px">新增用户组</h3><div style="display:grid;gap:12px">';
        html += this._bianji_hang('组名称', 'zu_xz_mingcheng', '', false);
        html += this._bianji_hang('备注', 'zu_xz_beizhu', '', true);
        html += `<div style="display:grid;grid-template-columns:100px 1fr;gap:12px;align-items:center;min-height:42px;padding:12px;border-radius:8px"><span style="font-size:14px;font-weight:500;color:#64748B">继承自</span><select id="zu_xz_jicheng" style="flex:1;border:1px solid #E2E8F0;border-radius:6px;padding:6px 10px;font-size:14px;outline:none;color:#1E293B"><option value="">不继承</option>${jichengxuanxiang}</select></div>`;
        html += `<div style="font-size:12px;color:#94A3B8;padding:0 12px">选择继承后，新用户组将复制该组的禁用接口权限配置</div>`;
        html += `<div style="display:flex;gap:12px;margin-top:8px"><button class="aq-btn aq-btn-zhu" onclick="yonghuzu_tijiaoxinzeng()" style="cursor:pointer">提交</button>${this._fanhuianniu('yonghuzu_shuaxin()')}</div>`;
        html += '</div></div>';
        nr.innerHTML = html;
    }

    async yonghuzu_tijiaoxinzeng() {
        const mingcheng = document.getElementById('zu_xz_mingcheng')?.value?.trim();
        if (!mingcheng) { this.luoji.rizhi('组名称不能为空', 'warn'); return; }
        const beizhu = document.getElementById('zu_xz_beizhu')?.value?.trim() || '';
        const jichengyonghuzuid = document.getElementById('zu_xz_jicheng')?.value || '';
        const jg = await this.luoji.yonghuzu_xinzeng(mingcheng, beizhu);
        if (!jg || jg.zhuangtaima !== 200) return;
        if (jichengyonghuzuid && jg.shuju?.id) {
            await this.luoji.yonghuzu_jicheng(String(jg.shuju.id), jichengyonghuzuid);
        }
        await this.shuaxinyonghuzuliebiao();
    }

    async yonghuzu_bianji(id) {
        const nr = document.getElementById('yonghu_neirong');
        nr.innerHTML = this._jiazaizhong();
        const [jg, jkjg, jnjg] = await Promise.all([
            this.luoji.yonghuzu_xiangqing(id),
            this.luoji.yonghuzu_jiekouliebiao(),
            this.luoji.yonghuzu_huoqujinjiekou(id),
        ]);
        if (!jg || jg.zhuangtaima !== 200) { nr.innerHTML = this._cuowuhtml('加载失败: ' + (jg ? jg.xiaoxi : '请求错误')); return; }
        const zu = jg.shuju || {};
        const shifouroot = zu.mingcheng === 'root';
        const jiekoulie = (jkjg && jkjg.zhuangtaima === 200) ? (jkjg.shuju || []) : [];
        const jinjiekoulie = (jnjg && jnjg.zhuangtaima === 200) ? (jnjg.shuju || []) : [];
        const jinjiekouji = new Set(jinjiekoulie);
        let html = '<div style="background:white;border:1px solid #E2E8F0;border-radius:12px;padding:24px;box-shadow:0 1px 3px rgba(0,0,0,0.05);max-width:700px">';
        html += '<h3 style="font-size:18px;font-weight:600;color:#0F172A;margin:0 0 20px">编辑用户组</h3><div style="display:grid;gap:12px">';
        html += this._hanghtml('ID', `<span style="font-size:14px;color:#0F172A">${zu.id}</span>`, false);
        html += this._bianji_hang('组名称', 'zu_bj_mingcheng', zu.mingcheng || '', true);
        html += this._bianji_hang('备注', 'zu_bj_beizhu', zu.beizhu || '', false);
        html += `<div style="display:flex;gap:12px;margin-top:8px"><button class="aq-btn aq-btn-zhu" onclick="yonghuzu_tijiaobian('${zu.id}')" style="cursor:pointer">保存基本信息</button>${this._fanhuianniu('yonghuzu_shuaxin()')}</div>`;
        html += '</div>';
        html += '<div style="margin-top:24px;padding-top:20px;border-top:1px solid #E2E8F0">';
        html += '<div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:16px"><h4 style="font-size:16px;font-weight:600;color:#0F172A;margin:0">接口权限管理</h4>';
        if (!shifouroot) {
            html += `<button class="aq-btn aq-btn-zhu" onclick="yonghuzu_baocunquanxian('${zu.id}')" style="cursor:pointer;padding:8px 20px">保存权限</button>`;
        }
        html += '</div>';
        if (shifouroot) {
            html += '<div style="padding:12px 16px;background:#F0FDF4;border:1px solid #BBF7D0;border-radius:8px;color:#166534;font-size:14px">root用户组拥有所有权限，无法修改</div>';
        } else if (!jiekoulie.length) {
            html += '<div style="padding:12px 16px;background:#FEF2F2;border:1px solid #FCA5A5;border-radius:8px;color:#991B1B;font-size:14px">暂无接口数据</div>';
        } else {
            html += '<div style="font-size:13px;color:#64748B;margin-bottom:12px">勾选表示<span style="color:#EF4444;font-weight:600">禁用</span>该接口，取消勾选表示允许访问</div>';
            html += '<div style="display:grid;gap:8px">';
            for (const jk of jiekoulie) {
                const lujing = jk.lujing || '';
                const nicheng = jk.nicheng || '';
                const fangshi = jk.fangshi || '';
                const xudenglu = jk.xudenglu === '1';
                if (!xudenglu) continue;
                const yijin = jinjiekouji.has(lujing);
                const cbid = 'zu_qx_' + lujing.replace(/\//g, '_');
                html += `<label for="${cbid}" style="display:flex;align-items:center;gap:10px;padding:10px 14px;background:${yijin ? '#FEF2F2' : '#F8FAFC'};border:1px solid ${yijin ? '#FCA5A5' : '#E2E8F0'};border-radius:8px;cursor:pointer;transition:all 200ms" onmouseenter="this.style.borderColor='#3B82F6'" onmouseleave="this.style.borderColor='${yijin ? '#FCA5A5' : '#E2E8F0'}'">`;
                html += `<input type="checkbox" id="${cbid}" data-lujing="${this._zhuanyi(lujing)}" ${yijin ? 'checked' : ''} style="width:18px;height:18px;accent-color:#EF4444;cursor:pointer">`;
                html += `<div style="flex:1;min-width:0"><div style="display:flex;align-items:center;gap:8px"><span style="font-size:14px;font-weight:500;color:#0F172A">${this._zhuanyi(nicheng)}</span><span style="padding:2px 8px;background:#E0E7FF;color:#3730A3;border-radius:4px;font-size:12px;font-weight:500">${fangshi}</span></div>`;
                html += `<div style="font-size:12px;color:#64748B;margin-top:2px;font-family:monospace">${this._zhuanyi(lujing)}</div></div></label>`;
            }
            html += '</div>';
        }
        html += '</div></div>';
        nr.innerHTML = html;
    }

    async yonghuzu_tijiaobian(id) {
        const mingcheng = document.getElementById('zu_bj_mingcheng')?.value?.trim();
        if (!mingcheng) { this.luoji.rizhi('组名称不能为空', 'warn'); return; }
        const beizhu = document.getElementById('zu_bj_beizhu')?.value?.trim() || '';
        const jg = await this.luoji.yonghuzu_xiugai(id, mingcheng, beizhu);
        if (jg && jg.zhuangtaima === 200) await this.yonghuzu_bianji(id);
    }

    async yonghuzu_baocunquanxian(id) {
        const checkboxes = document.querySelectorAll('input[type="checkbox"][data-lujing]');
        const jinjiekou = [];
        checkboxes.forEach(cb => { if (cb.checked) jinjiekou.push(cb.dataset.lujing); });
        const jg = await this.luoji.yonghuzu_gengxinjinjiekou(id, jinjiekou);
        if (jg && jg.zhuangtaima === 200) await this.yonghuzu_bianji(id);
    }

    async yonghuzu_shanchu(id) {
        const queren = await window.aqqueren('删除用户组', `确定要删除用户组 ID:${id} 吗？`);
        if (!queren) return;
        const jg = await this.luoji.yonghuzu_shanchu(id);
        if (jg && jg.zhuangtaima === 200) await this.shuaxinyonghuzuliebiao();
    }

    async yonghuzu_sousuo() {
        const gjc = document.getElementById('zu_sousuo')?.value?.trim() || '';
        if (!gjc) { this.luoji.rizhi('请输入搜索关键词', 'warn'); return; }
        this.zu_sousuomoshi = true; this.zu_gjc = gjc; this.zu_ye = 1;
        await this.shuaxinyonghuzuliebiao();
    }

    yonghuzu_qingkong() { this.zu_sousuomoshi = false; this.zu_gjc = ''; this.zu_ye = 1; this.shuaxinyonghuzuliebiao(); }
    async yonghuzu_shangyiye() { if (this.zu_ye > 1) { this.zu_ye--; await this.shuaxinyonghuzuliebiao(); } }
    async yonghuzu_xiayiye() { this.zu_ye++; await this.shuaxinyonghuzuliebiao(); }

    quanxuan_yonghu(cb) { gj.quanxuan('yh_pl_xz', cb); }
    quanxuan_yonghuzu(cb) { gj.quanxuan('zu_pl_xz', cb); }

    async piliangshanchu_yonghu() {
        await gj.piliangshanchu(this.luoji, { leibie: 'yh_pl_xz', mingcheng: '用户', shanchufn: id => this.luoji.piliang_shanchu(id), shuaxinfn: () => this.shuaxinliebiao(), tishi: '此操作不可撤销。' });
    }
    async piliangshanchu_yonghuzu() {
        await gj.piliangshanchu(this.luoji, {
            leibie: 'zu_pl_xz', mingcheng: '用户组',
            shanchufn: id => this.luoji.yonghuzu_piliang_shanchu(id),
            shuaxinfn: () => this.shuaxinyonghuzuliebiao(),
            houzhili: jg => { if (jg.shuju?.tiaoguo?.length) this.luoji.rizhi('部分跳过: ' + jg.shuju.tiaoguo.join('、'), 'warn'); }
        });
    }
}
