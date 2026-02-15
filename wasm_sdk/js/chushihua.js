import { FUWUQI_DIZHI } from './changliang.js';
import { DOM, rizhi, gengxinzhuangtai } from './gongyong.js';
import { zhuce_qingqiu } from './qingqiu.js';
import { zhuce_denglu } from './denglu.js';
import { zhuce_aiduihua } from './aiduihua.js';
import { qudao_qiyonganniu, zhuce_qudao } from './qudao.js';

/**
 * SDK 自动初始化
 * 加载 WASM 模块 → 创建客户端 → 注册各模块全局函数
 */
(async () => {
    try {
        const sdk = await import('../pkg/wasm_sdk.js?v=' + Date.now());
        const init = sdk.default;
        const Kehuduanjiami = sdk.Kehuduanjiami;

        await init({ module_or_path: '../pkg/wasm_sdk_bg.wasm?v=' + Date.now() });
        const kehu = new Kehuduanjiami(FUWUQI_DIZHI);

        gengxinzhuangtai('sdk_zhuangtai', '已就绪', true);
        rizhi('SDK 自动加载成功，服务器: ' + FUWUQI_DIZHI, 'ok');

        // 启用基础请求按钮
        ['btn_jiankang', 'btn_jiamiqingqiu', 'btn_sseceshi', 'btn_jiamisseceshi', 'btn_chongzhi', 'btn_denglu']
            .forEach(id => { DOM.get(id).disabled = false; });

        // 注册各模块
        zhuce_qingqiu(kehu);
        zhuce_denglu(kehu, qudao_qiyonganniu);
        zhuce_aiduihua(kehu, Kehuduanjiami);
        zhuce_qudao(kehu);

        // 恢复登录状态
        if (kehu.yidenglu()) {
            qudao_qiyonganniu(true);
            rizhi('已恢复登录状态（令牌已缓存）', 'ok');
        }
    } catch (e) {
        gengxinzhuangtai('sdk_zhuangtai', '加载失败', false);
        rizhi('SDK 加载失败: ' + e, 'err');
    }
})();
