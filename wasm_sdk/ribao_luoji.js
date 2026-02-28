// 日报管理 - 逻辑层
export class Ribaoluoji {
    constructor(kehu, rizhifn, shifouquanxian = false) {
        this.kehu = kehu;
        this.rizhi = rizhifn;
        this.shifouquanxian = !!shifouquanxian;
    }

    async zhixing(caozuo, canshu) {
        if (!this.kehu) { this.rizhi('尚未初始化', 'warn'); return null; }
        if (!this.kehu.yidenglu()) { this.rizhi('请先登录', 'warn'); return null; }
        try {
            const canshujson = canshu ? JSON.stringify(canshu) : null;
            const jieguo = await this.kehu.ribaoqingqiu(caozuo, canshujson);
            return JSON.parse(jieguo);
        } catch (e) {
            this.rizhi('请求失败: ' + e, 'err');
            return null;
        }
    }

    async zhixing_ai_jiekou(fangfa, lujing, qingqiuti = null, fanhui_json = true) {
        if (!this.kehu) { this.rizhi('尚未初始化', 'warn'); return null; }
        if (!this.kehu.yidenglu()) { this.rizhi('请先登录', 'warn'); return null; }
        const lingpai = this.kehu.huoqulingpai?.();
        const fuwuqidizhi = this.kehu.huoqufuwuqidizhi?.();
        if (!lingpai || !fuwuqidizhi) {
            this.rizhi('登录信息异常，无法请求AI调度器', 'err');
            return null;
        }
        try {
            const qingqiutou = { Authorization: 'Bearer ' + lingpai };
            const canshu = { method: fangfa, headers: qingqiutou };
            if (qingqiuti !== null && qingqiuti !== undefined) {
                qingqiutou['Content-Type'] = 'application/json';
                canshu.body = JSON.stringify(qingqiuti);
            }
            const xiangying = await fetch(fuwuqidizhi + lujing, canshu);
            const wenben = await xiangying.text();
            if (!fanhui_json) {
                return { zhuangtaima: xiangying.ok ? 200 : xiangying.status, xiaoxi: xiangying.statusText || '', shuju: wenben };
            }
            return JSON.parse(wenben);
        } catch (e) {
            this.rizhi('AI调度器请求失败: ' + e, 'err');
            return null;
        }
    }

    async diaoduqi_chaxun_zhuangtai() {
        const jg = await this.zhixing_ai_jiekou('GET', '/jiekou/ai/diaoduqi');
        if (jg) this.rizhi('查询AI调度器状态: ' + (jg.xiaoxi || ''), jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async diaoduqi_chaxun_jiankong() {
        const jg = await this.zhixing_ai_jiekou('GET', '/jiekou/ai/diaoduqi/jiankong', null, false);
        if (!jg || jg.zhuangtaima !== 200) {
            this.rizhi('查询AI调度器监控失败', 'err');
            return jg;
        }
        const zhibiao = {};
        for (const hang of String(jg.shuju || '').split('\n')) {
            const trim = hang.trim();
            if (!trim || trim.startsWith('#')) continue;
            const [mingcheng, yuanshi] = trim.split(/\s+/, 2);
            if (!mingcheng || yuanshi === undefined) continue;
            const shuzi = Number(yuanshi);
            zhibiao[mingcheng] = Number.isNaN(shuzi) ? yuanshi : shuzi;
        }
        return { zhuangtaima: 200, xiaoxi: '查询成功', shuju: zhibiao };
    }

    async diaoduqi_gengxin(quanju_shangxian, paidui_chaoshi_miao) {
        const qingqiuti = {};
        if (Number.isInteger(quanju_shangxian) && quanju_shangxian > 0) qingqiuti.quanju_shangxian = quanju_shangxian;
        if (Number.isInteger(paidui_chaoshi_miao) && paidui_chaoshi_miao > 0) qingqiuti.paidui_chaoshi_miao = paidui_chaoshi_miao;
        const jg = await this.zhixing_ai_jiekou('POST', '/jiekou/ai/diaoduqi/gengxin', qingqiuti);
        if (jg) this.rizhi('更新AI调度器配置: ' + (jg.xiaoxi || ''), jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async yonghuzhixing(caozuo, canshu) {
        if (!this.kehu) { this.rizhi('尚未初始化', 'warn'); return null; }
        if (!this.kehu.yidenglu()) { this.rizhi('请先登录', 'warn'); return null; }
        try {
            const canshujson = canshu ? JSON.stringify(canshu) : null;
            const jieguo = await this.kehu.ribaoyonghuqingqiu(caozuo, canshujson);
            return JSON.parse(jieguo);
        } catch (e) {
            this.rizhi('请求失败: ' + e, 'err');
            return null;
        }
    }

    async leixing_xinzeng(mingcheng) {
        const jg = await this.zhixing('leixing_xinzeng', { mingcheng });
        if (jg) this.rizhi('新增标签类型: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async leixing_shanchu(id) {
        const jg = await this.zhixing('leixing_shanchu', { id });
        if (jg) this.rizhi('删除标签类型[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async leixing_gengxin(id, mingcheng) {
        const jg = await this.zhixing('leixing_gengxin', { id, mingcheng });
        if (jg) this.rizhi('更新标签类型[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async leixing_chaxun_id(id) {
        const jg = await this.zhixing('leixing_chaxun_id', { id });
        if (jg) this.rizhi('查询标签类型[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async leixing_chaxun_mingcheng(mingcheng) {
        const jg = await this.zhixing('leixing_chaxun_mingcheng', { mingcheng });
        if (jg) this.rizhi('查询标签类型[' + mingcheng + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async leixing_chaxun_quanbu() {
        const jg = await this.zhixing('leixing_chaxun_quanbu');
        if (jg) this.rizhi('查询所有标签类型: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async biaoqian_xinzeng(leixingid, zhi) {
        const jg = await this.zhixing('biaoqian_xinzeng', { leixingid, zhi });
        if (jg) this.rizhi('新增标签: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async biaoqian_shanchu(id) {
        const jg = await this.zhixing('biaoqian_shanchu', { id });
        if (jg) this.rizhi('删除标签[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async biaoqian_gengxin(id, zhi) {
        const jg = await this.zhixing('biaoqian_gengxin', { id, zhi });
        if (jg) this.rizhi('更新标签[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async biaoqian_chaxun_id(id) {
        const jg = await this.zhixing('biaoqian_chaxun_id', { id });
        if (jg) this.rizhi('查询标签[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async biaoqian_chaxun_leixingid(leixingid) {
        const jg = await this.zhixing('biaoqian_chaxun_leixingid', { leixingid });
        if (jg) this.rizhi('查询类型标签[' + leixingid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async biaoqian_chaxun_leixingid_zhi(leixingid, zhi) {
        const jg = await this.zhixing('biaoqian_chaxun_leixingid_zhi', { leixingid, zhi });
        if (jg) this.rizhi('查询标签: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async biaoqian_chaxun_quanbu() {
        const jg = await this.zhixing('biaoqian_chaxun_quanbu');
        if (jg) this.rizhi('查询所有标签: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async biaoqian_chaxun_leixing(biaoqianid) {
        const jg = await this.zhixing('biaoqian_chaxun_leixing', { biaoqianid });
        if (jg) this.rizhi('查询标签类型[' + biaoqianid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async ribao_xinzeng(yonghuid, biaoti, neirong, fabushijian) {
        let jg;
        if (this.shifouquanxian) {
            jg = await this.zhixing('ribao_xinzeng', { yonghuid, biaoti, neirong, fabushijian });
        } else {
            jg = await this.yonghuzhixing('xinzeng', { biaoti, neirong, fabushijian });
        }
        if (jg) this.rizhi('新增日报: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async ribao_shanchu(id) {
        const jg = await this.zhixing('ribao_shanchu', { id });
        if (jg) this.rizhi('删除日报[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async ribao_gengxin(id, ziduanlie) {
        const jg = await this.zhixing('ribao_gengxin', { id, ziduanlie });
        if (jg) this.rizhi('更新日报[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async ribao_chaxun_id(id) {
        const jg = await this.zhixing('ribao_chaxun_id', { id });
        if (jg) this.rizhi('查询日报[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async ribao_chaxun_yonghuid(yonghuid) {
        const jg = await this.zhixing('ribao_chaxun_yonghuid', { yonghuid });
        if (jg) this.rizhi('查询用户日报[' + yonghuid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async ribao_chaxun_quanbu() {
        const jg = await this.zhixing('ribao_chaxun_quanbu');
        if (jg) this.rizhi('查询所有日报: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async ribao_chaxun_fenye(yeshu, meiyetiaoshu) {
        const jg = await this.zhixing('ribao_chaxun_fenye', { yeshu, meiyetiaoshu });
        if (jg) this.rizhi('分页查询日报: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async ribao_chaxun_yonghuid_fenye(yonghuid, yeshu, meiyetiaoshu) {
        const jg = await this.zhixing('ribao_chaxun_yonghuid_fenye', { yonghuid, yeshu, meiyetiaoshu });
        if (jg) this.rizhi('分页查询用户日报: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async ribao_chaxun_guanjianci_fenye(guanjianci, yeshu, meiyetiaoshu) {
        const jg = await this.zhixing('ribao_chaxun_guanjianci_fenye', { guanjianci, yeshu, meiyetiaoshu });
        if (jg) this.rizhi('关键词查询日报: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async ribao_tongji_zongshu() {
        const jg = await this.zhixing('ribao_tongji_zongshu');
        if (jg) this.rizhi('统计日报总数: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async ribao_tongji_yonghuid_zongshu(yonghuid) {
        const jg = await this.zhixing('ribao_tongji_yonghuid_zongshu', { yonghuid });
        if (jg) this.rizhi('统计用户日报总数: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async chaxunfenye_shipei(yeshu, meiyetiaoshu, quanbu = false) {
        if (this.shifouquanxian) {
            return this.ribao_chaxun_fenye(yeshu, meiyetiaoshu);
        }
        const caozuo = quanbu ? 'chaxun_quanbu_fenye' : 'chaxun_fenye';
        const jg = await this.yonghuzhixing(caozuo, { yeshu, meiyetiaoshu });
        if (jg) this.rizhi('分页查询日报: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanjiancichaxunfenye_shipei(guanjianci, yeshu, meiyetiaoshu) {
        if (this.shifouquanxian) {
            return this.ribao_chaxun_guanjianci_fenye(guanjianci, yeshu, meiyetiaoshu);
        }
        const jg = await this.yonghuzhixing('chaxun_fenye', { yeshu, meiyetiaoshu });
        if (jg) this.rizhi('分页查询日报: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async fabushijianchaxunfenye_shipei(kaishi, jieshu, yeshu, meiyetiaoshu) {
        const canshu = { kaishi, jieshu, yeshu, meiyetiaoshu };
        const jg = this.shifouquanxian
            ? await this.zhixing('ribao_chaxun_fabushijian_fenye', canshu)
            : await this.yonghuzhixing('chaxun_fabushijian_fenye', canshu);
        if (jg) this.rizhi('时间范围查询日报: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async tongjizongshu_shipei(quanbu = false) {
        if (this.shifouquanxian) {
            return this.ribao_tongji_zongshu();
        }
        const caozuo = quanbu ? 'tongji_quanbu_zongshu' : 'tongji_zongshu';
        const jg = await this.yonghuzhixing(caozuo);
        if (jg) this.rizhi('统计日报总数: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanlian_xinzeng(ribaoid, biaoqianid) {
        const jg = await this.zhixing('guanlian_xinzeng', { ribaoid, biaoqianid });
        if (jg) this.rizhi('新增关联: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanlian_shanchu_ribaoid(ribaoid) {
        const jg = await this.zhixing('guanlian_shanchu_ribaoid', { ribaoid });
        if (jg) this.rizhi('删除日报关联[' + ribaoid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanlian_shanchu(ribaoid, biaoqianid) {
        const jg = await this.zhixing('guanlian_shanchu', { ribaoid, biaoqianid });
        if (jg) this.rizhi('删除关联: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanlian_chaxun_ribaoid(ribaoid) {
        const jg = await this.zhixing('guanlian_chaxun_ribaoid', { ribaoid });
        if (jg) this.rizhi('查询日报关联[' + ribaoid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanlian_chaxun_biaoqianid(biaoqianid) {
        const jg = await this.zhixing('guanlian_chaxun_biaoqianid', { biaoqianid });
        if (jg) this.rizhi('查询标签关联[' + biaoqianid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanlian_chaxun_leixingmingcheng_zhi(leixingmingcheng, zhi) {
        const jg = await this.zhixing('guanlian_chaxun_leixingmingcheng_zhi', { leixingmingcheng, zhi });
        if (jg) this.rizhi('查询类型标签关联[' + leixingmingcheng + ',' + zhi + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanlian_chaxun_ribaoid_daixinxi(ribaoid) {
        const jg = await this.zhixing('guanlian_chaxun_ribaoid_daixinxi', { ribaoid });
        if (jg) this.rizhi('查询日报关联详情[' + ribaoid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanlian_chaxun_ribaoid_daixinxi_shipei(ribaoid) {
        if (this.shifouquanxian) {
            return this.guanlian_chaxun_ribaoid_daixinxi(ribaoid);
        }
        const jg = await this.yonghuzhixing('chaxun_ribao_biaoqian', { ribaoid });
        if (jg) this.rizhi('查询自己日报标签[' + ribaoid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanlian_chaxun_xiangguanbiaoqian(biaoqianid, leixingmingcheng) {
        const jg = await this.zhixing('guanlian_chaxun_xiangguanbiaoqian', { biaoqianid, leixingmingcheng });
        if (jg) this.rizhi('查询相关标签[' + biaoqianid + ',' + leixingmingcheng + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanlian_piliang_xinzeng(ribaoid, biaoqianidlie) {
        const jg = await this.zhixing('guanlian_piliang_xinzeng', { ribaoid, biaoqianidlie });
        if (jg) this.rizhi('批量新增关联: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_chaxun_id(id) {
        const jg = await this.zhixing('renwu_chaxun_id', { id });
        if (jg) this.rizhi('查询任务[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_chaxun_ribaoid(ribaoid) {
        const jg = await this.zhixing('renwu_chaxun_ribaoid', { ribaoid });
        if (jg) this.rizhi('查询日报任务[' + ribaoid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_chaxun_yonghuid(yonghuid, shuliang) {
        const jg = await this.zhixing('renwu_chaxun_yonghuid', { yonghuid, shuliang });
        if (jg) this.rizhi('查询用户任务[' + yonghuid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_chaxun_dengdai(shuliang) {
        const jg = await this.zhixing('renwu_chaxun_dengdai', { shuliang });
        if (jg) this.rizhi('查询待处理任务: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_chaxun_fenye(zhuangtai, yeshu, meiyetiaoshu) {
        const canshu = { yeshu, meiyetiaoshu };
        if (zhuangtai !== null && zhuangtai !== undefined) canshu.zhuangtai = zhuangtai;
        const jg = await this.zhixing('renwu_chaxun_fenye', canshu);
        if (jg) this.rizhi('查询任务列表: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_tongji_zhuangtai(zhuangtai) {
        const jg = await this.zhixing('renwu_tongji_zhuangtai', { zhuangtai });
        if (jg) this.rizhi('统计任务[' + zhuangtai + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_tongji_kechuli() {
        const jg = await this.zhixing('renwu_tongji_kechuli');
        if (jg) this.rizhi('统计可处理任务: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_chongxin_ruidui(id) {
        const jg = await this.zhixing('renwu_chongxin_ruidui', { id });
        if (jg) this.rizhi('重新入队[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_chongxin_ruidui_ribaoid(ribaoid) {
        const jg = await this.zhixing('renwu_chongxin_ruidui_ribaoid', { ribaoid });
        if (jg) this.rizhi('按日报重新入队[' + ribaoid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_shanchu(id) {
        const jg = await this.zhixing('renwu_shanchu', { id });
        if (jg) this.rizhi('删除任务[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_xinzeng(ribaoid) {
        const jg = await this.zhixing('renwu_xinzeng', { ribaoid });
        if (jg) this.rizhi('新增任务[日报' + ribaoid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_biaoqian_ai_chuli() {
        const jg = await this.zhixing('renwu_biaoqian_ai_chuli');
        if (jg) this.rizhi('标签任务处理: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_dange_chuli(id) {
        const jg = await this.zhixing('renwu_dange_chuli', { id });
        if (jg) this.rizhi('单任务处理[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_biaoqian_ai_tingzhi() {
        const jg = await this.zhixing('renwu_biaoqian_ai_tingzhi');
        if (jg) this.rizhi('标签任务停止: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_biaoqian_ai_zhuangtai() {
        const jg = await this.zhixing('renwu_biaoqian_ai_zhuangtai');
        return jg;
    }

    async leixing_piliang_shanchu(idlie) {
        const jg = await this.zhixing('leixing_piliang_shanchu', { idlie });
        if (jg) this.rizhi('批量删除标签类型: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async biaoqian_piliang_shanchu(idlie) {
        const jg = await this.zhixing('biaoqian_piliang_shanchu', { idlie });
        if (jg) this.rizhi('批量删除标签: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async ribao_piliang_shanchu(idlie) {
        const jg = await this.zhixing('ribao_piliang_shanchu', { idlie });
        if (jg) this.rizhi('批量删除日报: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanlian_piliang_shanchu_ribaoidlie(idlie) {
        const jg = await this.zhixing('guanlian_piliang_shanchu_ribaoidlie', { idlie });
        if (jg) this.rizhi('批量删除关联: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_piliang_shanchu(idlie) {
        const jg = await this.zhixing('renwu_piliang_shanchu', { idlie });
        if (jg) this.rizhi('批量删除任务: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async renwu_piliang_xinzeng_quanbu() {
        const jg = await this.zhixing('renwu_piliang_xinzeng_quanbu');
        if (jg) this.rizhi('批量创建任务: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async huoqu_guanxifenxi_leixing() {
        const jg = await this.zhixing('huoqu_guanxifenxi_leixing');
        return jg;
    }

    async tupu_chaxun_quanbu() {
        const jg = await this.zhixing('tupu_quanbu');
        if (jg) this.rizhi('查询全量图谱: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async tupu_chaxun_biaoqianid(biaoqianid) {
        const jg = await this.zhixing('tupu_biaoqianid', { biaoqianid });
        if (jg) this.rizhi('查询标签图谱[' + biaoqianid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async tupu_chaxun_leixingmingcheng(mingcheng) {
        const jg = await this.zhixing('tupu_leixingmingcheng', { mingcheng });
        if (jg) this.rizhi('查询类型图谱[' + mingcheng + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async tupu_sousuo(guanjianci, leixingmingcheng) {
        const canshu = { guanjianci };
        if (leixingmingcheng) canshu.leixingmingcheng = leixingmingcheng;
        const jg = await this.zhixing('tupu_sousuo', canshu);
        return jg;
    }

    async tupu_ribao_fenye(biaoqianid, yeshu, meiyetiaoshu) {
        const jg = await this.zhixing('tupu_ribao_fenye', { biaoqianid, yeshu, meiyetiaoshu });
        return jg;
    }

    async tupu_bian_ribao_fenye(yuan_biaoqianid, mubiao_biaoqianid, yeshu, meiyetiaoshu) {
        const jg = await this.zhixing('tupu_bian_ribao_fenye', { yuan_biaoqianid, mubiao_biaoqianid, yeshu, meiyetiaoshu });
        return jg;
    }

    async guanxi_chaxun_ribaoid(ribaoid) {
        const jg = await this.zhixing('guanxi_chaxun_ribaoid', { ribaoid });
        if (jg) this.rizhi('查询日报关系边[' + ribaoid + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async guanxi_piliang_shanchu_ribaoidlie(idlie) {
        const jg = await this.zhixing('guanxi_piliang_shanchu_ribaoidlie', { idlie });
        if (jg) this.rizhi('批量删除关系边: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async tupu_guanxi_shiti_ribao_fenye(shitimingcheng, yeshu, meiyetiaoshu) {
        const jg = await this.zhixing('tupu_guanxi_shiti_ribao_fenye', { shitimingcheng, yeshu, meiyetiaoshu });
        return jg;
    }

    async tupu_guanxi_bian_ribao_fenye(ren1, ren2, yeshu, meiyetiaoshu) {
        const jg = await this.zhixing('tupu_guanxi_bian_ribao_fenye', { ren1, ren2, yeshu, meiyetiaoshu });
        return jg;
    }

    async tupu_ribao_duobiaoqian_fenye(biaoqianidlie, yeshu, meiyetiaoshu) {
        const jg = await this.zhixing('tupu_ribao_duobiaoqian_fenye', { biaoqianidlie, yeshu, meiyetiaoshu });
        return jg;
    }

    // ========== 跨日报分析 ==========

    async fenxi_huoqu_shiti_leixing() {
        const jg = await this.zhixing('fenxi_huoqu_shiti_leixing');
        return jg;
    }

    async fenxi_shiti_liebiao(leixingmingcheng) {
        const jg = await this.zhixing('fenxi_shiti_liebiao', { leixingmingcheng });
        if (jg) this.rizhi('查询[' + leixingmingcheng + ']列表: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async fenxi_xiangmu_liebiao() {
        const jg = await this.zhixing('fenxi_xiangmu_liebiao');
        if (jg) this.rizhi('查询项目列表: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async fenxi_kehu_liebiao() {
        const jg = await this.zhixing('fenxi_kehu_liebiao');
        if (jg) this.rizhi('查询客户列表: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async fenxi_chaxun_jiaoliu(shiti_leixing, shiti_mingcheng) {
        const jg = await this.zhixing('fenxi_chaxun_jiaoliu', { shiti_leixing, shiti_mingcheng });
        if (jg) this.rizhi('查询交流内容[' + shiti_mingcheng + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async fenxi_jiaoliu_neirong(shiti_leixing, shiti_mingcheng) {
        const jg = await this.zhixing('fenxi_jiaoliu_neirong', { shiti_leixing, shiti_mingcheng });
        if (jg) this.rizhi('AI交流分析[' + shiti_mingcheng + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async fenxi_shiti_ribao(shiti_leixing, shiti_mingcheng) {
        const jg = await this.zhixing('fenxi_shiti_ribao', { shiti_leixing, shiti_mingcheng });
        if (jg) this.rizhi('查询实体日报[' + shiti_mingcheng + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async fenxi_ai_shendu(shiti_leixing, shiti_mingcheng, weidu) {
        const jg = await this.zhixing('fenxi_ai_shendu', { shiti_leixing, shiti_mingcheng, weidu });
        if (jg) this.rizhi('AI深度分析[' + weidu + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async fenxi_shiti_guanlian(leixingmingcheng, zhi_liebiao) {
        const jg = await this.zhixing('fenxi_shiti_guanlian', { leixingmingcheng, zhi_liebiao });
        if (jg) this.rizhi('实体关联分析[' + leixingmingcheng + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async fenxi_xiangmu_guanlian(xiangmu_liebiao) {
        const jg = await this.zhixing('fenxi_xiangmu_guanlian', { xiangmu_liebiao });
        if (jg) this.rizhi('项目关联分析: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async fenxi_zonghe_guanlian(shiti_liebiao) {
        const jg = await this.zhixing('fenxi_zonghe_guanlian', { shiti_liebiao });
        if (jg) this.rizhi('综合关联分析: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }
}

// ========== 分析 API 统一适配器 ==========
export class FenxiApiClient {
    constructor(luoji) {
        this._luoji = luoji;
    }

    // 统一请求包装：错误捕获 + 日志
    async _qingqiu(caozuo, canshu, rizhimingcheng) {
        try {
            const jg = await this._luoji.zhixing(caozuo, canshu);
            if (jg && rizhimingcheng) {
                this._luoji.rizhi(`${rizhimingcheng}: ${jg.xiaoxi}`, jg.zhuangtaima === 200 ? 'ok' : 'err');
            }
            return jg;
        } catch (e) {
            this._luoji.rizhi(`${rizhimingcheng || caozuo} 失败: ${e}`, 'err');
            return null;
        }
    }

    // 响应规范化：统一返回 { chenggong, xiaoxi, shuju }
    _guifanhua(jg, morenShuju = null) {
        if (!jg || jg.zhuangtaima !== 200) {
            return { chenggong: false, xiaoxi: jg?.xiaoxi || '请求错误', shuju: morenShuju };
        }
        return { chenggong: true, xiaoxi: jg.xiaoxi || '', shuju: jg.shuju ?? morenShuju };
    }

    // 获取实体类型配置，确保每项含必要字段
    async huoqu_shiti_leixing() {
        const jg = await this._qingqiu('fenxi_huoqu_shiti_leixing');
        const gui = this._guifanhua(jg, []);
        if (gui.chenggong && Array.isArray(gui.shuju)) {
            gui.shuju = gui.shuju.map(lx => ({
                mingcheng: lx.mingcheng || '',
                biaoti: lx.biaoti || lx.mingcheng || '',
                guanlianfenxi: !!lx.guanlianfenxi,
            }));
        }
        return gui;
    }

    // 获取某类型的实体列表
    async shiti_liebiao(leixingmingcheng) {
        const jg = await this._qingqiu('fenxi_shiti_liebiao', { leixingmingcheng }, `查询[${leixingmingcheng}]列表`);
        const gui = this._guifanhua(jg, []);
        if (gui.chenggong && Array.isArray(gui.shuju)) {
            gui.shuju = gui.shuju.map(x => ({
                zhi: x.zhi || '',
                ribao_shu: parseInt(x.ribao_shu, 10) || 0,
            }));
        }
        return gui;
    }

    // 获取实体关联日报与标签
    async shiti_ribao(shiti_leixing, shiti_mingcheng) {
        const jg = await this._qingqiu('fenxi_shiti_ribao', { shiti_leixing, shiti_mingcheng }, `查询实体日报[${shiti_mingcheng}]`);
        const gui = this._guifanhua(jg, { ribaolie: [], biaoqianlie: [] });
        if (gui.chenggong && gui.shuju) {
            gui.shuju.ribaolie = Array.isArray(gui.shuju.ribaolie) ? gui.shuju.ribaolie : [];
            gui.shuju.biaoqianlie = Array.isArray(gui.shuju.biaoqianlie) ? gui.shuju.biaoqianlie : [];
        }
        return gui;
    }

    // AI 深度分析
    async ai_shendu(shiti_leixing, shiti_mingcheng, weidu) {
        const jg = await this._qingqiu('fenxi_ai_shendu', { shiti_leixing, shiti_mingcheng, weidu }, `AI深度分析[${weidu}]`);
        const gui = this._guifanhua(jg);
        if (gui.chenggong && gui.shuju) {
            gui.shuju.ai_fenxi = gui.shuju.ai_fenxi ?? null;
        }
        return gui;
    }

    // 实体关联分析（通用）
    async shiti_guanlian(leixingmingcheng, zhi_liebiao, yonghu_tishi = '') {
        const canshu = { leixingmingcheng, zhi_liebiao };
        if (yonghu_tishi) canshu.yonghu_tishi = yonghu_tishi;
        const jg = await this._qingqiu('fenxi_shiti_guanlian', canshu, `实体关联分析[${leixingmingcheng}]`);
        const gui = this._guifanhua(jg);
        if (gui.chenggong && gui.shuju) {
            gui.shuju.xiangmu_shuju = Array.isArray(gui.shuju.xiangmu_shuju) ? gui.shuju.xiangmu_shuju : [];
            gui.shuju.ai_fenxi = gui.shuju.ai_fenxi ?? null;
        }
        return gui;
    }

    // 项目关联分析（兼容旧接口）
    async xiangmu_guanlian(xiangmu_liebiao) {
        const jg = await this._qingqiu('fenxi_xiangmu_guanlian', { xiangmu_liebiao }, '项目关联分析');
        return this._guifanhua(jg);
    }

    // 综合关联分析（跨类型）
    async zonghe_guanlian(shiti_liebiao, yonghu_tishi = '') {
        const canshu = { shiti_liebiao };
        if (yonghu_tishi) canshu.yonghu_tishi = yonghu_tishi;
        const jg = await this._qingqiu('fenxi_zonghe_guanlian', canshu, '综合关联分析');
        const gui = this._guifanhua(jg);
        if (gui.chenggong && gui.shuju) {
            gui.shuju.xiangmu_shuju = Array.isArray(gui.shuju.xiangmu_shuju) ? gui.shuju.xiangmu_shuju : [];
            gui.shuju.ai_fenxi = gui.shuju.ai_fenxi ?? null;
        }
        return gui;
    }
}
