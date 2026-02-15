import { DOM, rizhi, zhixinganniu } from './gongyong.js';

/** 注册请求测试相关的全局函数 */
export function zhuce_qingqiu(kehu) {

    window.chongzhihuihua = function () {
        kehu.chongzhihuihua();
        rizhi('会话已重置，下次加密请求将自动重新协商', 'warn');
    };

    window.jiankangqingqiu = () => zhixinganniu('btn_jiankang', async () => {
        const jieguo = await kehu.jiankangqingqiu();
        rizhi('健康检查: ' + jieguo, 'ok');
    });

    window.jiamijiankangqingqiu = () => zhixinganniu('btn_jiamiqingqiu', async () => {
        const jieguo = await kehu.jiamijiankangqingqiu('你好加密传输');
        rizhi('加密响应: ' + jieguo, 'ok');
    });

    window.sseceshiqingqiu = () => zhixinganniu('btn_sseceshi', async () => {
        rizhi('--- 普通SSE开始 ---', 'info');
        await kehu.sseceshiqingqiu('sse_huidiao');
        rizhi('--- 普通SSE结束 ---', 'info');
    });

    window.jiamisseceshiqingqiu = () => zhixinganniu('btn_jiamisseceshi', async () => {
        rizhi('--- 加密SSE开始 ---', 'info');
        await kehu.jiamisseceshiqingqiu('sse_huidiao');
        rizhi('--- 加密SSE结束 ---', 'info');
    });

    window.qingkongrizhi = function () {
        DOM.get('rizhi').innerHTML = '';
    };

    window.sse_huidiao = function (shuju) {
        rizhi('SSE: ' + shuju, 'info');
    };
}
