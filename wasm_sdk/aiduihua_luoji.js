// AI对话管理 - 逻辑层
export class Aiduihualuoji {
    constructor(kehu, rizhifn) {
        this.kehu = kehu;
        this.rizhi = rizhifn;
        this.lishijilu = this.jiazailishi();
        this.dangqianmoshi = 'feiliushi'; // 'feiliushi' 或 'liushi'
    }

    // 从localStorage加载历史记录
    jiazailishi() {
        try {
            const json = localStorage.getItem('ai_duihua_lishi');
            return json ? JSON.parse(json) : [];
        } catch (e) {
            this.rizhi('加载历史记录失败: ' + e, 'warn');
            return [];
        }
    }

    // 保存历史记录到localStorage
    baocunlishi() {
        try {
            localStorage.setItem('ai_duihua_lishi', JSON.stringify(this.lishijilu));
        } catch (e) {
            this.rizhi('保存历史记录失败: ' + e, 'warn');
        }
    }

    // 添加消息到历史
    tianjiaxiaoxi(juese, neirong) {
        this.lishijilu.push({ juese, neirong });
        this.baocunlishi();
    }

    // 清空历史记录
    qingkonglishi() {
        this.lishijilu = [];
        localStorage.removeItem('ai_duihua_lishi');
        this.rizhi('历史记录已清空', 'info');
    }

    // 删除指定索引的消息
    shanchuxiaoxi(suoyin) {
        if (suoyin >= 0 && suoyin < this.lishijilu.length) {
            this.lishijilu.splice(suoyin, 1);
            this.baocunlishi();
            this.rizhi('消息已删除', 'info');
        }
    }

    // 获取历史记录
    huoqulishi() {
        return this.lishijilu;
    }

    // 设置模式
    shezhimoshi(moshi) {
        if (moshi === 'feiliushi' || moshi === 'liushi') {
            this.dangqianmoshi = moshi;
            this.rizhi('切换到' + (moshi === 'liushi' ? '流式' : '非流式') + '模式', 'info');
        }
    }

    // 非流式对话
    async feiliushiduihua(neirong) {
        if (!this.kehu) {
            this.rizhi('尚未初始化', 'warn');
            return null;
        }
        if (!this.kehu.yidenglu()) {
            this.rizhi('请先登录', 'warn');
            return null;
        }
        if (!neirong || !neirong.trim()) {
            this.rizhi('消息内容不能为空', 'warn');
            return null;
        }

        try {
            // 添加用户消息到历史
            this.tianjiaxiaoxi('user', neirong);

            // 构建请求
            const xiaoxilie = this.lishijilu.map(x => ({ juese: x.juese, neirong: x.neirong }));
            const xiaoxilie_json = JSON.stringify(xiaoxilie);

            this.rizhi('发送非流式对话请求...', 'info');
            const jieguo_json = await this.kehu.aiduihuaqingqiu(xiaoxilie_json);
            const jieguo = JSON.parse(jieguo_json);

            if (jieguo.zhuangtaima === 200 && jieguo.shuju && jieguo.shuju.huifu) {
                const huifu = jieguo.shuju.huifu;
                this.tianjiaxiaoxi('assistant', huifu);
                this.rizhi('AI回复成功', 'ok');
                return huifu;
            } else {
                this.rizhi('AI回复失败: ' + jieguo.xiaoxi, 'err');
                // 失败时移除用户消息
                this.lishijilu.pop();
                this.baocunlishi();
                return null;
            }
        } catch (e) {
            this.rizhi('对话请求失败: ' + e, 'err');
            // 失败时移除用户消息
            if (this.lishijilu.length > 0 && this.lishijilu[this.lishijilu.length - 1].juese === 'user') {
                this.lishijilu.pop();
                this.baocunlishi();
            }
            return null;
        }
    }

    // 流式对话
    async liushiduihua(neirong, huidiaohanming) {
        if (!this.kehu) {
            this.rizhi('尚未初始化', 'warn');
            return false;
        }
        if (!this.kehu.yidenglu()) {
            this.rizhi('请先登录', 'warn');
            return false;
        }
        if (!neirong || !neirong.trim()) {
            this.rizhi('消息内容不能为空', 'warn');
            return false;
        }

        try {
            // 添加用户消息到历史
            this.tianjiaxiaoxi('user', neirong);

            // 构建请求
            const xiaoxilie = this.lishijilu.map(x => ({ juese: x.juese, neirong: x.neirong }));
            const xiaoxilie_json = JSON.stringify(xiaoxilie);

            this.rizhi('发送流式对话请求...', 'info');
            await this.kehu.aiduihualiushiqingqiu(xiaoxilie_json, huidiaohanming);
            this.rizhi('流式对话完成', 'ok');
            return true;
        } catch (e) {
            this.rizhi('流式对话失败: ' + e, 'err');
            // 失败时移除用户消息
            if (this.lishijilu.length > 0 && this.lishijilu[this.lishijilu.length - 1].juese === 'user') {
                this.lishijilu.pop();
                this.baocunlishi();
            }
            return false;
        }
    }

    // 导出历史记录
    daochulishi() {
        const json = JSON.stringify(this.lishijilu, null, 2);
        const blob = new Blob([json], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'ai_duihua_lishi_' + new Date().getTime() + '.json';
        a.click();
        URL.revokeObjectURL(url);
        this.rizhi('历史记录已导出', 'ok');
    }
}
