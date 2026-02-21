// 用户管理 - 逻辑层
export class Yonghuluoji {
    constructor(kehu, rizhifn) {
        this.kehu = kehu;
        this.rizhi = rizhifn;
    }

    async zhixing(caozuo, canshu) {
        if (!this.kehu) { this.rizhi('尚未初始化', 'warn'); return null; }
        if (!this.kehu.yidenglu()) { this.rizhi('请先登录', 'warn'); return null; }
        try {
            const canshujson = canshu ? JSON.stringify(canshu) : null;
            const jieguo = await this.kehu.yonghuguanliqingqiu(caozuo, canshujson);
            return JSON.parse(jieguo);
        } catch (e) {
            this.rizhi('请求失败: ' + e, 'err');
            return null;
        }
    }

    async fenye(dangqianyeshu, meiyeshuliang) {
        const jg = await this.zhixing('fenye', { dangqianyeshu, meiyeshuliang });
        if (jg) this.rizhi('分页查询: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async sousuo(guanjianci, dangqianyeshu, meiyeshuliang) {
        const jg = await this.zhixing('sousuo', { guanjianci, dangqianyeshu, meiyeshuliang });
        if (jg) this.rizhi('搜索用户: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async xiangqing(id) {
        const jg = await this.zhixing('xiangqing', { id });
        if (jg) this.rizhi('查询详情[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }
}
