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
        const jg = await this.zhixing('ribao_xinzeng', { yonghuid, neirong, fabushijian });
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

    async chaxunfenye_shipei(yeshu, meiyetiaoshu) {
        return this.shifouquanxian
            ? this.ribao_chaxun_fenye(yeshu, meiyetiaoshu)
            : this.ribao_chaxun_yonghuid_fenye('', yeshu, meiyetiaoshu);
    }

    async guanjiancichaxunfenye_shipei(guanjianci, yeshu, meiyetiaoshu) {
        return this.shifouquanxian
            ? this.ribao_chaxun_guanjianci_fenye(guanjianci, yeshu, meiyetiaoshu)
            : this.ribao_chaxun_yonghuid_fenye('', yeshu, meiyetiaoshu);
    }

    async tongjizongshu_shipei() {
        return this.shifouquanxian
            ? this.ribao_tongji_zongshu()
            : this.ribao_tongji_yonghuid_zongshu('');
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
}
