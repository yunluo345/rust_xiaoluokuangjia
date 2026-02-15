import { ZHUANGTAIMA_CHENGGONG } from './changliang.js';
import { DOM, rizhi, zhixinganniu } from './gongyong.js';

/**
 * 注册登录相关的全局函数
 * @param {object} kehu - WASM 客户端实例
 * @param {Function} qudao_qiyonganniu - 渠道按钮启用回调
 */
export function zhuce_denglu(kehu, qudao_qiyonganniu) {

    window.dengluqingqiu = () => zhixinganniu('btn_denglu', async () => {
        const zhanghao = DOM.huoqu('denglu_zhanghao');
        const mima = DOM.huoqu('denglu_mima');
        if (!zhanghao || !mima) return rizhi('账号或密码不能为空', 'warn');

        const jieguo = await kehu.dengluqingqiu(zhanghao, mima);
        const xiangying = JSON.parse(jieguo);
        const chenggong = xiangying.zhuangtaima === ZHUANGTAIMA_CHENGGONG;

        rizhi(
            chenggong ? '登录成功' : '登录失败: ' + xiangying.xiaoxi,
            chenggong ? 'ok' : 'err'
        );

        if (chenggong) {
            qudao_qiyonganniu(true);
        }
    });
}
