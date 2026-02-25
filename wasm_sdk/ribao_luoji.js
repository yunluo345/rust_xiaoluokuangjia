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

    async ribao_xinzeng(yonghuid, neirong, fabushijian) {
        let jg;
        if (this.shifouquanxian) {
            jg = await this.zhixing('ribao_xinzeng', { yonghuid, neirong, fabushijian });
        } else {
            jg = await this.yonghuzhixing('xinzeng', { neirong, fabushijian });
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
}
