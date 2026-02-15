import { QUDAO_ZIDUAN } from './changliang.js';
import { DOM, rizhi, chulixiangying, zhixinganniu, anquanzhixing } from './gongyong.js';

/** 表单显示状态 */
let qudao_biaodanxianshi = false;

/**
 * 启用/禁用渠道相关按钮
 * @param {boolean} kai - 是否启用
 */
export function qudao_qiyonganniu(kai) {
    ['btn_qudao_liebiao', 'btn_qudao_xinzeng', 'btn_ai_fasong'].forEach(id => {
        DOM.get(id).disabled = !kai;
    });
}

/** 清空渠道表单 */
function qudao_qingkongbiaodan() {
    QUDAO_ZIDUAN.forEach(ziduan => {
        DOM.get('qudao_' + ziduan).value = '';
    });
    DOM.get('qudao_bianjid').value = '';
    DOM.get('qudao_biaodanbiaoti').textContent = '新增渠道';
}

/**
 * 渲染渠道列表表格
 * @param {Array} liebiao - 渠道数据数组
 */
function qudao_xuanranbiao(liebiao) {
    const rongqi = DOM.get('qudao_liebiaoku');
    if (!liebiao || !liebiao.length) {
        rongqi.innerHTML = '<div style="color:#94A3B8;font-size:13px;padding:20px 0;text-align:center">暂无渠道数据</div>';
        return;
    }
    const tou = '<tr><th>ID</th><th>名称</th><th>类型</th><th>模型</th><th>接口地址</th><th>温度</th><th>最大Token</th><th>优先级</th><th>状态</th><th>备注</th><th>操作</th></tr>';
    const hang = liebiao.map(q => {
        const zt = q.zhuangtai === '1';
        return `<tr>
            <td>${q.id}</td>
            <td title="${q.mingcheng || ''}">${q.mingcheng || ''}</td>
            <td>${q.leixing || ''}</td>
            <td title="${q.moxing || ''}">${q.moxing || ''}</td>
            <td title="${q.jiekoudizhi || ''}">${q.jiekoudizhi || ''}</td>
            <td>${q.wendu || '0'}</td>
            <td>${q.zuidatoken || '0'}</td>
            <td><input type="number" class="youxianji-input" value="${q.youxianji ?? 0}" onchange="qudao_gengxinyouxianji('${q.id}',this.value)" style="width:64px;padding:4px 8px;min-height:32px;font-size:13px;text-align:center"></td>
            <td><span class="${zt ? 'zt-qi' : 'zt-jin'}">${zt ? '启用' : '禁用'}</span></td>
            <td title="${q.beizhu || ''}">${q.beizhu || '-'}</td>
            <td style="white-space:nowrap">
                <button class="btn-sm btn-bian" onclick="qudao_bianji('${q.id}')">编辑</button>
                <button class="btn-sm btn-qie" onclick="qudao_qiehuanzhuangtai('${q.id}')">${zt ? '禁用' : '启用'}</button>
                <button class="btn-sm btn-shan" onclick="qudao_shanchu('${q.id}')">删除</button>
            </td></tr>`;
    }).join('');
    rongqi.innerHTML = `<table><thead>${tou}</thead><tbody>${hang}</tbody></table>`;
}

/**
 * 注册渠道管理相关的全局函数
 * @param {object} kehu - WASM 客户端实例
 */
export function zhuce_qudao(kehu) {

    /** 切换新增表单显示 */
    window.qudao_xinzengbiaodanqiehuan = function () {
        qudao_biaodanxianshi = !qudao_biaodanxianshi;
        DOM.get('qudao_xinzengqu').style.display = qudao_biaodanxianshi ? 'block' : 'none';
        if (!qudao_biaodanxianshi) qudao_qingkongbiaodan();
    };

    /** 刷新渠道列表 */
    window.qudao_liebiao = () => zhixinganniu('btn_qudao_liebiao', async () => {
        const jieguo = await kehu.aiqudao_liebiao();
        const { xiangying, chenggong } = chulixiangying(jieguo);
        if (chenggong) {
            qudao_xuanranbiao(xiangying.shuju);
            rizhi('渠道列表刷新成功，共 ' + (xiangying.shuju?.length || 0) + ' 条', 'ok');
        }
    });

    /** 提交新增/编辑渠道 */
    window.qudao_tijiao = async function () {
        const bianjid = DOM.get('qudao_bianjid').value;
        const huoquzhi = (ziduan) => DOM.huoqu('qudao_' + ziduan);

        const btn = DOM.get('btn_qudao_tijiao');
        btn.disabled = true;

        try {
            let jieguo;
            if (bianjid) {
                const gengxinti = { caozuo: 'gengxin', id: bianjid };
                QUDAO_ZIDUAN.forEach(ziduan => {
                    const zhi = huoquzhi(ziduan);
                    if (ziduan === 'beizhu' || zhi) {
                        gengxinti[ziduan] = zhi || '';
                    }
                });
                jieguo = await kehu.aiqudao_xiugai(JSON.stringify(gengxinti));
            } else {
                const bitian = ['mingcheng', 'leixing', 'jiekoudizhi', 'miyao', 'moxing'];
                const queshao = bitian.filter(z => !huoquzhi(z));
                if (queshao.length > 0) {
                    rizhi('名称、类型、接口地址、密钥、模型为必填项', 'warn');
                    return;
                }
                jieguo = await kehu.aiqudao_tianjia(
                    huoquzhi('mingcheng'), huoquzhi('leixing'), huoquzhi('jiekoudizhi'),
                    huoquzhi('miyao'), huoquzhi('moxing'),
                    huoquzhi('wendu') || undefined,
                    huoquzhi('beizhu') || undefined,
                    huoquzhi('zuidatoken') || undefined,
                    huoquzhi('youxianji') || undefined
                );
            }

            const { chenggong } = chulixiangying(jieguo, (bianjid ? '编辑' : '新增') + '渠道成功');
            if (chenggong) {
                qudao_biaodanxianshi = false;
                DOM.get('qudao_xinzengqu').style.display = 'none';
                qudao_qingkongbiaodan();
                window.qudao_liebiao();
            }
        } catch (e) {
            rizhi('提交失败: ' + e, 'err');
        } finally {
            btn.disabled = false;
        }
    };

    /** 删除渠道 */
    window.qudao_shanchu = async function (id) {
        if (!confirm('确认删除渠道 ID=' + id + ' ？')) return;
        await anquanzhixing(async () => {
            const jieguo = await kehu.aiqudao_shanchu(id);
            const { chenggong } = chulixiangying(jieguo, '删除渠道成功');
            if (chenggong) window.qudao_liebiao();
        }, '删除失败');
    };

    /** 切换渠道启用/禁用状态 */
    window.qudao_qiehuanzhuangtai = async function (id) {
        await anquanzhixing(async () => {
            const ti = JSON.stringify({ caozuo: 'qiehuanzhuangtai', id });
            const jieguo = await kehu.aiqudao_xiugai(ti);
            const { chenggong } = chulixiangying(jieguo, '切换状态成功');
            if (chenggong) window.qudao_liebiao();
        }, '切换状态失败');
    };

    /** 更新渠道优先级 */
    window.qudao_gengxinyouxianji = async function (id, zhi) {
        await anquanzhixing(async () => {
            const ti = JSON.stringify({ caozuo: 'gengxin', id, youxianji: zhi });
            const jieguo = await kehu.aiqudao_xiugai(ti);
            chulixiangying(jieguo, '优先级更新成功');
        }, '优先级更新失败');
    };

    /** 编辑渠道（加载详情到表单） */
    window.qudao_bianji = async function (id) {
        await anquanzhixing(async () => {
            const ti = JSON.stringify({ caozuo: 'xiangqing', id });
            const jieguo = await kehu.aiqudao_caozuo(ti);
            const { xiangying, chenggong } = chulixiangying(jieguo);

            if (!chenggong || !xiangying.shuju) {
                rizhi('查询渠道详情失败: ' + xiangying.xiaoxi, 'err');
                return;
            }

            const q = xiangying.shuju;
            DOM.get('qudao_bianjid').value = q.id;
            QUDAO_ZIDUAN.forEach(ziduan => {
                DOM.get('qudao_' + ziduan).value = q[ziduan] ?? '';
            });
            DOM.get('qudao_biaodanbiaoti').textContent = '编辑渠道 #' + q.id;
            qudao_biaodanxianshi = true;
            DOM.get('qudao_xinzengqu').style.display = 'block';
        }, '查询详情失败');
    };
}
