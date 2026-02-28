// 界面层公共工具

// API 结果提取：直接取 shuju
export function tiqushuju(jg, morenzhi = []) {
    return jg?.zhuangtaima === 200 ? (jg.shuju || morenzhi) : morenzhi;
}

// API 分页结果提取：取 liebiao + zongshu
export function tiqufenyeshuju(jg) {
    if (!jg || jg.zhuangtaima !== 200) return { liebiao: [], zongshu: 0 };
    return { liebiao: jg.shuju?.liebiao || [], zongshu: jg.shuju?.zongshu || 0 };
}

// 状态 HTML（加载/错误/空/警告）
export function zhuangtai_html(leixing, xiaoxi) {
    const yansemap = { jiazai: '#64748B', cuowu: '#EF4444', kong: '#94A3B8', jinggao: '#F59E0B' };
    return `<p style="color:${yansemap[leixing] || '#64748B'}">${xiaoxi}</p>`;
}

// API 结果处理：成功返回 true，403 提示权限不足，其余返回 false
export function chuli_api_jieguo(luoji, jg) {
    if (jg?.zhuangtaima === 200) return true;
    if (jg?.zhuangtaima === 403) luoji.rizhi('权限不足：' + jg.xiaoxi, 'warn');
    return false;
}

// 高频共现统计
export function tongjigaopin(biaoqianlie, paichutiaojian = null, zuiduo = 8) {
    const map = new Map();
    for (const bq of (biaoqianlie || [])) {
        const lx = String(bq.leixingmingcheng || '').trim();
        const zhi = String(bq.zhi || '').trim();
        if (!lx || !zhi) continue;
        if (paichutiaojian && paichutiaojian(lx, zhi)) continue;
        const key = `${lx}::${zhi}`;
        const cishu = parseInt(bq.cishu, 10);
        const zengliang = (!isNaN(cishu) && cishu > 0) ? cishu : 1;
        map.set(key, (map.get(key) || 0) + zengliang);
    }
    return Array.from(map.entries())
        .map(([k, cishu]) => {
            const idx = k.indexOf('::');
            return { leixing: k.slice(0, idx), zhi: k.slice(idx + 2), cishu };
        })
        .sort((a, b) => b.cishu - a.cishu)
        .slice(0, zuiduo);
}

// 实体类型 → 中文简称映射
const SHITI_BIAOQIAN_MAP = { '客户公司': '公司', '客户名字': '联系人', '项目名称': '项目' };
export function huoqushitibiaoqian(leixingmingcheng) {
    return SHITI_BIAOQIAN_MAP[leixingmingcheng] || '标签';
}

// 全选/取消全选复选框
export function quanxuan(leibie, fuxuankuang) {
    document.querySelectorAll('.' + leibie).forEach(cb => cb.checked = fuxuankuang.checked);
}

// 批量删除通用流程：收集选中 → 确认 → 调用API → 刷新
export async function piliangshanchu(luoji, { leibie, mingcheng, shanchufn, shuaxinfn, tishi, houzhili }) {
    const xuanzhong = [...document.querySelectorAll('.' + leibie + ':checked')].map(cb => cb.dataset.id);
    if (!xuanzhong.length) { luoji.rizhi('请先勾选要删除的' + mingcheng, 'warn'); return; }
    if (!await window.aqqueren('批量删除' + mingcheng, `确定要删除选中的 ${xuanzhong.length} 个${mingcheng}吗？${tishi || ''}`)) return;
    const jg = await shanchufn(xuanzhong);
    if (jg?.zhuangtaima === 200) { houzhili?.(jg); await shuaxinfn(); }
}
