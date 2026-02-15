import { ZHUANGTAIMA_CHENGGONG } from './changliang.js';

/** DOM 操作工具 */
export const DOM = {
    get: (id) => document.getElementById(id),
    huoqu: (id) => document.getElementById(id)?.value?.trim() || '',
};

/**
 * 输出日志到 #rizhi 容器
 * @param {string} neirong - 日志内容
 * @param {string} leixing - 样式类名：ok / err / info / warn
 */
export function rizhi(neirong, leixing = '') {
    const shijian = new Date().toLocaleTimeString();
    const leixingclass = leixing ? ` class="${leixing}"` : '';
    const el = DOM.get('rizhi');
    el.innerHTML += `<span style="color:#64748B">[${shijian}]</span> <span${leixingclass}>${neirong}</span>\n`;
    el.scrollTop = el.scrollHeight;
}

/**
 * 更新状态标签
 * @param {string} id - 元素 ID
 * @param {string} wenben - 显示文本
 * @param {boolean} chenggong - 是否成功
 */
export function gengxinzhuangtai(id, wenben, chenggong) {
    const el = DOM.get(id);
    el.textContent = wenben;
    el.className = 'zhuangtai ' + (chenggong ? 'lianjiecheng' : 'weilianjiecheng');
}

/**
 * 解析并处理 WASM 返回的 JSON 响应
 * @param {string} jieguo - WASM 返回的 JSON 字符串
 * @param {string} chenggongxiaoxi - 成功时的自定义提示
 * @returns {{ xiangying: object, chenggong: boolean }}
 */
export function chulixiangying(jieguo, chenggongxiaoxi = '') {
    const xiangying = typeof jieguo === 'string' ? JSON.parse(jieguo) : jieguo;
    const chenggong = xiangying.zhuangtaima === ZHUANGTAIMA_CHENGGONG;
    const xiaoxi = chenggongxiaoxi && chenggong ? chenggongxiaoxi : xiangying.xiaoxi;
    rizhi(xiaoxi, chenggong ? 'ok' : 'err');
    return { xiangying, chenggong };
}

/**
 * 按钮防重复执行包装器
 * @param {string} anniuid - 按钮元素 ID
 * @param {Function} caozuo - 异步操作
 */
export async function zhixinganniu(anniuid, caozuo) {
    const btn = DOM.get(anniuid);
    btn.disabled = true;
    try {
        await caozuo();
    } catch (e) {
        rizhi('请求失败: ' + e, 'err');
    } finally {
        btn.disabled = false;
    }
}

/**
 * 安全执行异步操作（带错误日志）
 * @param {Function} caozuo - 异步操作
 * @param {string} cuowuxiaoxi - 失败时的提示前缀
 */
export async function anquanzhixing(caozuo, cuowuxiaoxi = '操作失败') {
    try {
        return await caozuo();
    } catch (e) {
        rizhi(`${cuowuxiaoxi}: ${e}`, 'err');
        throw e;
    }
}

/** HTML 转义，防止 XSS */
export function zhuanyiHTML(str) {
    const div = document.createElement('div');
    div.textContent = str;
    return div.innerHTML;
}

/** 尝试格式化 JSON 字符串，失败则原样转义 */
export function geshihuaJSON(str) {
    try {
        const obj = JSON.parse(str);
        return zhuanyiHTML(JSON.stringify(obj, null, 2));
    } catch (_) {
        return zhuanyiHTML(str);
    }
}

/** 向 #ai_shuchu 追加 HTML */
export function tianjiaHTML(html) {
    DOM.get('ai_shuchu').insertAdjacentHTML('beforeend', html);
}
