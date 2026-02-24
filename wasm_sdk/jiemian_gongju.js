// 界面层公共工具

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
