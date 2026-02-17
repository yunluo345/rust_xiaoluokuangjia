// AI渠道管理 - 逻辑层
export class Aiqudaoluoji {
    constructor(kehu, rizhifn) {
        this.kehu = kehu;
        this.rizhi = rizhifn;
    }

    async zhixing(caozuo, canshu) {
        if (!this.kehu) { this.rizhi('尚未初始化', 'warn'); return null; }
        if (!this.kehu.yidenglu()) { this.rizhi('请先登录', 'warn'); return null; }
        try {
            const canshujson = canshu ? JSON.stringify(canshu) : null;
            const jieguo = await this.kehu.aiqudaoqingqiu(caozuo, canshujson);
            return JSON.parse(jieguo);
        } catch (e) {
            this.rizhi('请求失败: ' + e, 'err');
            return null;
        }
    }

    async chaxunquanbu() {
        const jg = await this.zhixing('chaxun_quanbu');
        if (jg) this.rizhi('查询所有渠道: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async chaxunqiyong() {
        const jg = await this.zhixing('chaxun_qiyong');
        if (jg) this.rizhi('查询启用渠道: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async chaxunid(id) {
        const jg = await this.zhixing('chaxun_id', { id });
        if (jg) this.rizhi('查询渠道[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async xinzeng(shuju) {
        const jg = await this.zhixing('xinzeng', shuju);
        if (jg) this.rizhi('新增渠道: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async gengxin(id, ziduanlie) {
        const jg = await this.zhixing('gengxin', { id, ziduanlie });
        if (jg) this.rizhi('更新渠道[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async shanchu(id) {
        const jg = await this.zhixing('shanchu', { id });
        if (jg) this.rizhi('删除渠道[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async qiehuanzhuangtai(id) {
        const jg = await this.zhixing('qiehuanzhuangtai', { id });
        if (jg) this.rizhi('切换状态[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async gengxinyouxianji(id, youxianji) {
        const jg = await this.zhixing('gengxinyouxianji', { id, youxianji });
        if (jg) this.rizhi('更新优先级[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }
}
