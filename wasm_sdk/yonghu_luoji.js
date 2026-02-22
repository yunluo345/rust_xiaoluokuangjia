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

    async xiugai_zhanghao(id, zhanghao) {
        const jg = await this.zhixing('xiugai_zhanghao', { id, zhanghao });
        if (jg) this.rizhi('修改账号[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async xiugai_nicheng(id, nicheng) {
        const jg = await this.zhixing('xiugai_nicheng', { id, nicheng });
        if (jg) this.rizhi('修改昵称[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async xiugai_mima(id, mima) {
        const jg = await this.zhixing('xiugai_mima', { id, mima });
        if (jg) this.rizhi('修改密码[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async xiugai_beizhu(id, beizhu) {
        const jg = await this.zhixing('xiugai_beizhu', { id, beizhu });
        if (jg) this.rizhi('修改备注[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async fengjin(id, yuanyin, jieshu) {
        const canshu = { id, yuanyin };
        if (jieshu) canshu.jieshu = jieshu;
        const jg = await this.zhixing('fengjin', canshu);
        if (jg) this.rizhi('封禁用户[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async jiefeng(id) {
        const jg = await this.zhixing('jiefeng', { id });
        if (jg) this.rizhi('解封用户[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async xinzeng(zhanghao, mima, nicheng, yonghuzuid, beizhu) {
        const jg = await this.zhixing('xinzeng', { zhanghao, mima, nicheng, yonghuzuid, beizhu });
        if (jg) this.rizhi('新增用户: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async shanchu(id) {
        const jg = await this.zhixing('shanchu', { id });
        if (jg) this.rizhi('删除用户[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async yonghuzu_fenye(dangqianyeshu, meiyeshuliang) {
        const jg = await this.zhixing('yonghuzu_fenye', { dangqianyeshu, meiyeshuliang });
        if (jg) this.rizhi('用户组分页: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async yonghuzu_sousuo(guanjianci, dangqianyeshu, meiyeshuliang) {
        const jg = await this.zhixing('yonghuzu_sousuo', { guanjianci, dangqianyeshu, meiyeshuliang });
        if (jg) this.rizhi('搜索用户组: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async yonghuzu_xiangqing(id) {
        const jg = await this.zhixing('yonghuzu_xiangqing', { id });
        if (jg) this.rizhi('用户组详情[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async yonghuzu_xinzeng(mingcheng, beizhu) {
        const jg = await this.zhixing('yonghuzu_xinzeng', { mingcheng, beizhu });
        if (jg) this.rizhi('新增用户组: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async yonghuzu_xiugai(id, mingcheng, beizhu) {
        const jg = await this.zhixing('yonghuzu_xiugai', { id, mingcheng, beizhu });
        if (jg) this.rizhi('修改用户组[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }

    async yonghuzu_shanchu(id) {
        const jg = await this.zhixing('yonghuzu_shanchu', { id });
        if (jg) this.rizhi('删除用户组[' + id + ']: ' + jg.xiaoxi, jg.zhuangtaima === 200 ? 'ok' : 'err');
        return jg;
    }
}
