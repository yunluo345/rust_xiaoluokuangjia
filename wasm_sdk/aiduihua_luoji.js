// AI对话管理 - 逻辑层
const zhongzhizhuangtai_yizhongzhi = 'yizhongzhi';
export class Aiduihualuoji {
    constructor(kehu, rizhifn) {
        this.kehu = kehu;
        this.rizhi = rizhifn;
        this.dangqianmoshi = 'feiliushi'; // 'feiliushi' 或 'liushi'
        this.abortController = null; // AbortController 引用，用于终止请求
        this.shuju = this.jiazaishuju();
        // 确保至少有一个会话
        if (this.shuju.huihualiebiao.length === 0) {
            this.xinjianhuihua();
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
    xinjianhuihua() {
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
                this.xinjianhuihua();
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
    tianjiaxiaoxi(juese, neirong, sikao) {
        const huihua = this.huoqudangqianhuihua();
        if (!huihua) return;
        const xiaoxi = { juese, neirong };
        if (sikao) xiaoxi.sikao = sikao;
        huihua.xiaoxilie.push(xiaoxi);
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
            delete huihua.aiui;
            delete huihua.xunwen; // 兼容旧字段
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

    // 删除最后一条用户消息（流式失败时回滚用）
    shanchuzuihouyonghuxiaoxi() {
        const huihua = this.huoqudangqianhuihua();
        if (huihua && huihua.xiaoxilie.length > 0 && huihua.xiaoxilie[huihua.xiaoxilie.length - 1].juese === 'user') {
            huihua.xiaoxilie.pop();
            this.baocunshuju();
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
    duihuaqianjianzha(neirong) {
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
        return true;
    }

    goujianxiaoxijson() {
        const xiaoxilie = this.huoqulishi().map(x => ({ juese: x.juese, neirong: x.neirong }));
        return JSON.stringify(xiaoxilie);
    }

    // 非流式对话
    async feiliushiduihua(neirong) {
        if (!this.duihuaqianjianzha(neirong)) return null;

        try {
            const xiaoxilie_json = this.goujianxiaoxijson();

            this.rizhi('发送非流式对话请求...', 'info');
            this.abortController = null;
            const jieguo_json = await this.kehu.aiduihuaqingqiu(xiaoxilie_json, 'aiduihua_duquqi_huidiao');
            const jieguo = JSON.parse(jieguo_json);

            if (jieguo.zhuangtaima === 200 && jieguo.shuju && jieguo.shuju.huifu) {
                // 检测 AIUI 响应（后端解析成功时 leixing 在 shuju 上）
                if (jieguo.shuju.leixing) {
                    return {
                        leixing: jieguo.shuju.leixing,
                        huifu: jieguo.shuju.huifu || '',
                        shuju: jieguo.shuju.shuju || null,
                    };
                }
                const huifu = jieguo.shuju.huifu;
                // 后备检测：huifu 本身是 AIUI JSON 字符串（后端解析失败时整串放在 huifu 里）
                try {
                    const obj = JSON.parse(huifu);
                    if (obj && obj.leixing && obj.huifu) {
                        return {
                            leixing: obj.leixing,
                            huifu: obj.huifu,
                            shuju: obj.shuju || null,
                        };
                    }
                } catch (e) {}
                const sikao = jieguo.shuju.sikao || null;
                const yitu_sikao = jieguo.shuju.yitu_sikao || null;
                if (yitu_sikao) {
                    this.tianjiaxiaoxi('assistant', `[意图思考] ${yitu_sikao}`);
                }
                this.tianjiaxiaoxi('assistant', huifu, sikao);
                this.rizhi('AI回复成功', 'ok');
                return huifu;
            } else {
                this.rizhi('AI回复失败: ' + jieguo.xiaoxi, 'err');
                this.shanchuzuihouyonghuxiaoxi();
                return null;
            }
        } catch (e) {
            if (this.abortController === zhongzhizhuangtai_yizhongzhi) {
                this.rizhi('非流式对话已终止', 'info');
                this.shanchuzuihouyonghuxiaoxi();
                return null;
            }
            this.rizhi('对话请求失败: ' + e, 'err');
            this.shanchuzuihouyonghuxiaoxi();
            return null;
        } finally {
            this.abortController = null;
        }
    }

    // 流式对话
    async liushiduihua(neirong, huidiaohanming, duquqi_huidiaohanming) {
        if (!this.duihuaqianjianzha(neirong)) return false;

        try {
            // 用户消息已由界面层提前添加

            // 构建请求
            const xiaoxilie = this.huoqulishi().map(x => ({ juese: x.juese, neirong: x.neirong }));
            const xiaoxilie_json = JSON.stringify(xiaoxilie);

            this.rizhi('发送流式对话请求...', 'info');
            this.abortController = null;
            await this.kehu.aiduihualiushiqingqiu(xiaoxilie_json, huidiaohanming, duquqi_huidiaohanming);
            this.rizhi('流式对话完成', 'ok');
            return true;
        } catch (e) {
            if (this.abortController === zhongzhizhuangtai_yizhongzhi) {
                this.rizhi('流式对话已终止', 'info');
                return true;
            }
            this.rizhi('流式对话失败: ' + e, 'err');
            return false;
        } finally {
            this.abortController = null;
        }
    }

    // 保存 AbortController（由 WASM 回调调用）
    baocunduquqi(controller) {
        this.abortController = controller;
        this.rizhi('已获取 AbortController，可以终止', 'info');
    }

    // 终止流式对话
    async zhongzhiliushi() {
        if (!this.abortController) {
            this.rizhi('AbortController 尚未就绪，无法终止', 'warn');
            return;
        }
        if (this.abortController === zhongzhizhuangtai_yizhongzhi) {
            this.rizhi('已经终止过了', 'warn');
            return;
        }
        try {
            this.rizhi('正在终止流式对话...', 'info');
            this.abortController.abort();
            this.abortController = zhongzhizhuangtai_yizhongzhi;
            this.rizhi('流式对话已终止', 'ok');
        } catch (e) {
            this.rizhi('终止失败: ' + e, 'err');
        }
    }

    // 导出历史记录
    huoqudaochushuju() {
        return JSON.stringify(this.shuju, null, 2);
    }
    daochulishi() {
        return this.huoqudaochushuju();
    }

    // 保存待处理 AIUI 状态到当前会话
    baocundaiui(shuju) {
        const huihua = this.huoqudangqianhuihua();
        if (huihua) {
            huihua.aiui = shuju;
            delete huihua.xunwen; // 兼容旧字段迁移
            this.baocunshuju();
        }
    }

    // 清除待处理 AIUI 状态
    qingchuaiui() {
        const huihua = this.huoqudangqianhuihua();
        if (huihua && (huihua.aiui || huihua.xunwen)) {
            delete huihua.aiui;
            delete huihua.xunwen; // 兼容旧字段
            this.baocunshuju();
        }
    }

    // 获取当前会话的待处理 AIUI 状态
    huoquaiui() {
        const huihua = this.huoqudangqianhuihua();
        return huihua ? (huihua.aiui || huihua.xunwen || null) : null;
    }

    // 兼容旧调用：询问工具专用方法转发到通用 AIUI
    baocunxunwen(shuju) {
        this.baocundaiui(shuju);
    }
    qingchuxunwen() {
        this.qingchuaiui();
    }
    huoquxunwen() {
        return this.huoquaiui();
    }
}
