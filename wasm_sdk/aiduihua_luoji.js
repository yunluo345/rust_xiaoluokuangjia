// AI对话管理 - 逻辑层
export class Aiduihualuoji {
    constructor(kehu, rizhifn) {
        this.kehu = kehu;
        this.rizhi = rizhifn;
        this.dangqianmoshi = 'feiliushi'; // 'feiliushi' 或 'liushi'
        this.shuju = this.jiazaishuju();
        // 确保至少有一个会话
        if (this.shuju.huihualiebiao.length === 0) {
            this.xinjianhiuhua();
        }
    }

    // 从localStorage加载全部数据
    jiazaishuju() {
        try {
            const json = localStorage.getItem('ai_duihua_shuju');
            if (json) {
                const d = JSON.parse(json);
                if (d.huihualiebiao && d.dangqianid) return d;
            }
            // 兼容旧数据迁移
            const laoJson = localStorage.getItem('ai_duihua_lishi');
            if (laoJson) {
                const laoLishi = JSON.parse(laoJson);
                if (Array.isArray(laoLishi) && laoLishi.length > 0) {
                    const id = this.shengchengid();
                    const huihua = { id, mingcheng: '历史对话', xiaoxilie: laoLishi, chuangjianshijian: Date.now() };
                    localStorage.removeItem('ai_duihua_lishi');
                    return { huihualiebiao: [huihua], dangqianid: id };
                }
            }
        } catch (e) {
            this.rizhi('加载数据失败: ' + e, 'warn');
        }
        return { huihualiebiao: [], dangqianid: null };
    }

    // 保存全部数据到localStorage
    baocunshuju() {
        try {
            localStorage.setItem('ai_duihua_shuju', JSON.stringify(this.shuju));
        } catch (e) {
            this.rizhi('保存数据失败: ' + e, 'warn');
        }
    }

    // 生成简单唯一id
    shengchengid() {
        return Date.now().toString(36) + Math.random().toString(36).substring(2, 7);
    }

    // 获取当前会话
    huoqudangqianhuihua() {
        return this.shuju.huihualiebiao.find(h => h.id === this.shuju.dangqianid) || null;
    }

    // 获取会话列表
    huoquhuihualiebiao() {
        return this.shuju.huihualiebiao;
    }

    // 获取当前会话id
    huoqudangqianid() {
        return this.shuju.dangqianid;
    }

    // 新建会话
    xinjianhiuhua() {
        const id = this.shengchengid();
        const huihua = { id, mingcheng: '新对话', xiaoxilie: [], chuangjianshijian: Date.now() };
        this.shuju.huihualiebiao.unshift(huihua);
        this.shuju.dangqianid = id;
        this.baocunshuju();
        return id;
    }

    // 切换会话
    qiehuanhuihua(id) {
        const huihua = this.shuju.huihualiebiao.find(h => h.id === id);
        if (huihua) {
            this.shuju.dangqianid = id;
            this.baocunshuju();
        }
    }

    // 删除会话
    shanchuhuihua(id) {
        this.shuju.huihualiebiao = this.shuju.huihualiebiao.filter(h => h.id !== id);
        if (this.shuju.dangqianid === id) {
            if (this.shuju.huihualiebiao.length > 0) {
                this.shuju.dangqianid = this.shuju.huihualiebiao[0].id;
            } else {
                this.xinjianhiuhua();
                return;
            }
        }
        this.baocunshuju();
    }

    // 重命名会话
    chongmingminghuihua(id, mingcheng) {
        const huihua = this.shuju.huihualiebiao.find(h => h.id === id);
        if (huihua) {
            huihua.mingcheng = mingcheng;
            this.baocunshuju();
        }
    }

    // 添加消息到当前会话
    tianjiaxiaoxi(juese, neirong) {
        const huihua = this.huoqudangqianhuihua();
        if (!huihua) return;
        huihua.xiaoxilie.push({ juese, neirong });
        // 第一条用户消息时自动命名
        if (juese === 'user' && huihua.mingcheng === '新对话') {
            huihua.mingcheng = neirong.substring(0, 20) + (neirong.length > 20 ? '...' : '');
        }
        this.baocunshuju();
    }

    // 清空当前会话历史
    qingkonglishi() {
        const huihua = this.huoqudangqianhuihua();
        if (huihua) {
            huihua.xiaoxilie = [];
            huihua.mingcheng = '新对话';
            this.baocunshuju();
            this.rizhi('当前对话已清空', 'info');
        }
    }

    // 删除指定索引的消息
    shanchuxiaoxi(suoyin) {
        const huihua = this.huoqudangqianhuihua();
        if (huihua && suoyin >= 0 && suoyin < huihua.xiaoxilie.length) {
            huihua.xiaoxilie.splice(suoyin, 1);
            this.baocunshuju();
            this.rizhi('消息已删除', 'info');
        }
    }

    // 获取当前会话历史记录
    huoqulishi() {
        const huihua = this.huoqudangqianhuihua();
        return huihua ? huihua.xiaoxilie : [];
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
            const xiaoxilie = this.huoqulishi().map(x => ({ juese: x.juese, neirong: x.neirong }));
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
                const huihua = this.huoqudangqianhuihua();
                if (huihua) { huihua.xiaoxilie.pop(); this.baocunshuju(); }
                return null;
            }
        } catch (e) {
            this.rizhi('对话请求失败: ' + e, 'err');
            // 失败时移除用户消息
            const huihua = this.huoqudangqianhuihua();
            if (huihua && huihua.xiaoxilie.length > 0 && huihua.xiaoxilie[huihua.xiaoxilie.length - 1].juese === 'user') {
                huihua.xiaoxilie.pop();
                this.baocunshuju();
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
            const xiaoxilie = this.huoqulishi().map(x => ({ juese: x.juese, neirong: x.neirong }));
            const xiaoxilie_json = JSON.stringify(xiaoxilie);

            this.rizhi('发送流式对话请求...', 'info');
            await this.kehu.aiduihualiushiqingqiu(xiaoxilie_json, huidiaohanming);
            this.rizhi('流式对话完成', 'ok');
            return true;
        } catch (e) {
            this.rizhi('流式对话失败: ' + e, 'err');
            // 失败时移除用户消息
            const huihua = this.huoqudangqianhuihua();
            if (huihua && huihua.xiaoxilie.length > 0 && huihua.xiaoxilie[huihua.xiaoxilie.length - 1].juese === 'user') {
                huihua.xiaoxilie.pop();
                this.baocunshuju();
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
