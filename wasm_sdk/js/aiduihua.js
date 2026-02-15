import { DOM, rizhi, zhuanyiHTML, geshihuaJSON, tianjiaHTML } from './gongyong.js';

/** AI 对话状态 */
let duihua_lishi = [];
let dangqian_huifu = '';
let dangqian_zhongduanqi = null;
let ai_zhengzaichuli = false;

/** 工具调用渲染状态 */
let gongju_jishu = 0;
let gongju_canshu_map = {};
let suoyin_id_map = {};
let id_dom_map = {};

/** 切换发送/停止按钮状态 */
function ai_anniuzhuangtai(zhengzai) {
    ai_zhengzaichuli = zhengzai;
    const btnFasong = DOM.get('btn_ai_fasong');
    const btnTingzhi = DOM.get('btn_ai_tingzhi');
    btnFasong.disabled = zhengzai;
    btnTingzhi.disabled = !zhengzai;
    btnFasong.style.background = zhengzai ? '#94A3B8' : '#3B82F6';
    btnFasong.style.cursor = zhengzai ? 'not-allowed' : 'pointer';
    btnTingzhi.style.background = zhengzai ? '#EF4444' : '#94A3B8';
    btnTingzhi.style.cursor = zhengzai ? 'pointer' : 'not-allowed';
}

/**
 * 注册 AI 对话相关的全局函数
 * @param {object} kehu - WASM 客户端实例
 * @param {Function} Kehuduanjiami - WASM 类（用于创建中断器）
 */
export function zhuce_aiduihua(kehu, Kehuduanjiami) {

    /** SSE 流式回调 — 处理各类事件 */
    window.ai_huidiao = function (shuju) {
        const shuchu = DOM.get('ai_shuchu');
        const qianzhui = 'data: ';
        for (const hang of shuju.split('\n').filter(s => s.startsWith(qianzhui))) {
            try {
                const shijian = JSON.parse(hang.slice(qianzhui.length));
                const { leixing } = shijian;

                switch (leixing) {
                    case 'sikao_guocheng':
                        tianjiaHTML(`<div class="ai-sikao">💭 ${zhuanyiHTML(shijian.neirong)}</div>`);
                        break;
                    case 'yitu_fenxi':
                        tianjiaHTML(`<div class="ai-yitu">🎯 意图：${zhuanyiHTML(shijian.yitu)}<br>🔑 关键词：${shijian.guanjianci.map(k => zhuanyiHTML(k)).join('、')}</div>`);
                        break;
                    case 'gongju_faxian': {
                        const liebiao = shijian.jieguo.map(g =>
                            `<span style="color:#FCD34D">${zhuanyiHTML(g.mingcheng)}</span><span style="color:#94A3B8">(${zhuanyiHTML(g.miaoshu)}, ${(g.defen * 100).toFixed(0)}%)</span>`
                        ).join('、');
                        tianjiaHTML(`<div class="ai-gongju-faxian">🔍 发现工具：${liebiao}</div>`);
                        break;
                    }
                    case 'wenben_kuai':
                        dangqian_huifu += shijian.neirong;
                        tianjiaHTML(`<span class="ai-wenben">${zhuanyiHTML(shijian.neirong)}</span>`);
                        break;
                    case 'gongju_kaishi': {
                        gongju_jishu++;
                        const domId = gongju_jishu;
                        const sy = shijian.suoyin;
                        suoyin_id_map[sy] = shijian.gongjuid;
                        id_dom_map[shijian.gongjuid] = domId;
                        gongju_canshu_map[sy] = '';
                        dangqian_huifu += `\n[工具调用: ${shijian.gongjuming}]\n参数: `;
                        tianjiaHTML(
                            `<div class="ai-gongju-kuai" id="gongju_${domId}">` +
                                `<div class="ai-gongju-tou" onclick="gongju_zhedie(${domId})">` +
                                    `<span class="zhuangtai-deng deng-yunxing"></span>` +
                                    `<span class="mingcheng">⚙️ ${zhuanyiHTML(shijian.gongjuming)}</span>` +
                                    `<span class="gongju-tishi" style="color:#64748B;font-weight:400">参数接收中...</span>` +
                                    `<span class="zhedie-jiantou">▼</span>` +
                                `</div>` +
                                `<div class="ai-gongju-neirong" id="gongju_nr_${domId}">` +
                                    `<div class="ai-gongju-canshu" id="gongju_cs_${domId}"><span style="color:#64748B">参数加载中...</span></div>` +
                                `</div>` +
                            `</div>`
                        );
                        break;
                    }
                    case 'gongju_canshu': {
                        const sy = shijian.suoyin;
                        dangqian_huifu += shijian.bufen_json;
                        if (gongju_canshu_map[sy] !== undefined) {
                            gongju_canshu_map[sy] += shijian.bufen_json;
                        }
                        break;
                    }
                    case 'gongju_wancheng': {
                        const domId = id_dom_map[shijian.gongjuid];
                        if (domId) {
                            const csEl = document.getElementById(`gongju_cs_${domId}`);
                            const sy = shijian.suoyin;
                            if (csEl && gongju_canshu_map[sy] !== undefined) {
                                csEl.innerHTML = geshihuaJSON(gongju_canshu_map[sy]);
                                delete gongju_canshu_map[sy];
                            }
                            const tishi = document.querySelector(`#gongju_${domId} .gongju-tishi`);
                            if (tishi) tishi.textContent = '执行中...';
                        }
                        break;
                    }
                    case 'gongju_jieguo': {
                        dangqian_huifu += `\n结果: ${shijian.jieguo}\n`;
                        const domId = id_dom_map[shijian.gongjuid];
                        if (domId) {
                            const kuaiEl = document.getElementById(`gongju_${domId}`);
                            if (kuaiEl) {
                                const deng = kuaiEl.querySelector('.zhuangtai-deng');
                                const tishi = kuaiEl.querySelector('.gongju-tishi');
                                if (deng) { deng.classList.remove('deng-yunxing'); deng.classList.add('deng-chenggong'); }
                                if (tishi) tishi.textContent = '完成';
                                const nrEl = document.getElementById(`gongju_nr_${domId}`);
                                if (nrEl) nrEl.insertAdjacentHTML('beforeend', `<div class="ai-gongju-jieguo">${geshihuaJSON(shijian.jieguo)}</div>`);
                            }
                        }
                        break;
                    }
                    case 'yasuo_wancheng':
                        duihua_lishi = [{ jiaose: 'yonghu', neirong: `【历史对话总结】\n${shijian.zongjie}` }];
                        tianjiaHTML(`<div class="ai-yasuo">📦 对话已压缩</div>`);
                        break;
                    case 'wancheng':
                        if (dangqian_huifu) {
                            duihua_lishi.push({ jiaose: 'zhushou', neirong: dangqian_huifu });
                            dangqian_huifu = '';
                        }
                        dangqian_zhongduanqi = null;
                        ai_anniuzhuangtai(false);
                        tianjiaHTML(`<div class="ai-wancheng">✅ 完成</div>`);
                        break;
                    case 'cuowu':
                        dangqian_huifu = '';
                        dangqian_zhongduanqi = null;
                        ai_anniuzhuangtai(false);
                        tianjiaHTML(`<div class="ai-cuowu">❌ ${zhuanyiHTML(shijian.xinxi)}</div>`);
                        break;
                }
            } catch (_) {}
        }
        shuchu.scrollTop = shuchu.scrollHeight;
    };

    /** 工具块折叠切换 */
    window.gongju_zhedie = function (id) {
        const nrEl = document.getElementById(`gongju_nr_${id}`);
        const jiantou = document.querySelector(`#gongju_${id} .zhedie-jiantou`);
        if (nrEl) nrEl.classList.toggle('yincang');
        if (jiantou) jiantou.classList.toggle('shouqi');
    };

    /** 发送 AI 对话 */
    window.aifasong = function () {
        if (!kehu) return rizhi('尚未初始化', 'warn');
        if (ai_zhengzaichuli) return rizhi('正在处理中，请等待或点击停止', 'warn');

        const neirong = DOM.huoqu('ai_shuru');
        if (!neirong) return rizhi('请输入消息内容', 'warn');

        ai_anniuzhuangtai(true);

        try {
            const leixing = DOM.get('ai_leixing').value;
            const xitongtishici = DOM.huoqu('ai_xitongtishici') || undefined;
            duihua_lishi.push({ jiaose: 'yonghu', neirong });
            const xiaoxilie = JSON.stringify(duihua_lishi);
            const shuchu = DOM.get('ai_shuchu');
            shuchu.insertAdjacentHTML('beforeend',
                `<div class="ai-yonghu">👤 你：${zhuanyiHTML(neirong)}</div><div class="ai-zhushou-biaoji">🤖 AI：</div>`
            );
            shuchu.scrollTop = shuchu.scrollHeight;

            rizhi('AI对话请求中...', 'info');

            dangqian_zhongduanqi = Kehuduanjiami.chuangjian_zhongduanqi();
            kehu.aiduihua(leixing, xiaoxilie, 'ai_huidiao', xitongtishici, dangqian_zhongduanqi);
        } catch (e) {
            const cuowuXinxi = e.toString();
            rizhi('请求失败: ' + cuowuXinxi, 'err');
            const shuchu = DOM.get('ai_shuchu');
            shuchu.insertAdjacentHTML('beforeend', `<div class="ai-cuowu">❌ ${zhuanyiHTML(cuowuXinxi)}</div>`);
            shuchu.scrollTop = shuchu.scrollHeight;
            dangqian_zhongduanqi = null;
            ai_anniuzhuangtai(false);
        }
    };

    /** 停止 AI 输出 */
    window.aitingzhi = function () {
        if (!ai_zhengzaichuli) return;
        if (dangqian_zhongduanqi) {
            dangqian_zhongduanqi.abort();
            dangqian_zhongduanqi = null;
            rizhi('正在停止AI输出...', 'warn');
        }
        ai_anniuzhuangtai(false);
    };

    /** 继续对话（快捷发送"继续"） */
    window.aijixu = function () {
        if (!kehu) return rizhi('尚未初始化', 'warn');
        if (ai_zhengzaichuli) return rizhi('正在处理中，请等待', 'warn');
        DOM.get('ai_shuru').value = '继续';
        window.aifasong();
    };

    /** 清空对话 */
    window.aiqingkong = function () {
        DOM.get('ai_shuchu').innerHTML = '';
        DOM.get('ai_shuru').value = '';
        duihua_lishi = [];
        dangqian_huifu = '';
        gongju_jishu = 0;
        gongju_canshu_map = {};
        suoyin_id_map = {};
        id_dom_map = {};
    };
}
