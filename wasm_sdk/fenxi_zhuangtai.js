// 分析视图 - 状态管理层

export class FenxiZhuangtai {
    constructor() {
        this.chongzhi();
    }

    // 重置所有状态
    chongzhi() {
        // 实体类型配置列表 [{mingcheng, biaoti, guanlianfenxi}]
        this.shiti_leixinglie = [];
        // 每个类型的实体数据 { [leixingmingcheng]: [{zhi, ribao_shu}] }
        this.shiti_shujumap = {};
        // 支持关联分析的类型的勾选集合 { [leixingmingcheng]: Set }
        this.xuanzhong_map = {};
        // 兼容旧字段
        this.xuanzhong_xiangmu = new Set();
        // 当前选中的实体 { leixing, mingcheng } | null
        this.dangqian_shiti = null;
        // AI 分析是否正在运行
        this.yunxingzhong = false;
    }

    // 初始化类型与数据
    shezhiPeizhi(shiti_leixinglie) {
        this.shiti_leixinglie = shiti_leixinglie || [];
        this.shiti_shujumap = {};
        this.xuanzhong_map = {};
        for (const lx of this.shiti_leixinglie) {
            if (lx.guanlianfenxi) {
                this.xuanzhong_map[lx.mingcheng] = new Set();
            }
        }
        // 兼容旧字段
        this.xuanzhong_xiangmu = this.xuanzhong_map['项目名称'] || new Set();
    }

    // 设置某类型的实体数据
    shezhiShujulie(leixingmingcheng, shujulie) {
        this.shiti_shujumap[leixingmingcheng] = shujulie || [];
    }

    // 获取某类型的实体数据
    huoquShujulie(leixingmingcheng) {
        return this.shiti_shujumap[leixingmingcheng] || [];
    }

    // 勾选/取消实体（关联分析用）
    gouxuanShiti(leixing, mingcheng, shifougouxuan) {
        if (!this.xuanzhong_map[leixing]) {
            this.xuanzhong_map[leixing] = new Set();
        }
        if (shifougouxuan) {
            this.xuanzhong_map[leixing].add(mingcheng);
        } else {
            this.xuanzhong_map[leixing].delete(mingcheng);
        }
        // 兼容旧字段
        if (leixing === '项目名称') {
            this.xuanzhong_xiangmu = this.xuanzhong_map[leixing];
        }
    }

    // 获取某类型的勾选列表
    huoquXuanzhonglie(leixingmingcheng) {
        return Array.from(this.xuanzhong_map[leixingmingcheng] || []);
    }

    // 设置当前实体，同时停止之前的分析
    shezhiDangqianShiti(leixing, mingcheng) {
        this.yunxingzhong = false;
        this.dangqian_shiti = { leixing, mingcheng };
    }

    // 开始 AI 分析
    kaishiFenxi() {
        this.yunxingzhong = true;
    }

    // 停止 AI 分析
    tingzhiFenxi() {
        this.yunxingzhong = false;
    }

    // 获取搜索占位符文本
    huoquSousuoTishi() {
        return this.shiti_leixinglie.map(lx => lx.biaoti).join('/') || '搜索';
    }

    // 获取支持关联分析的类型列表
    huoquGuanlianLeixinglie() {
        return this.shiti_leixinglie.filter(lx => lx.guanlianfenxi);
    }

    // 根据类型名查找配置
    chazhaoLeixingPeizhi(leixingmingcheng) {
        return this.shiti_leixinglie.find(l => l.mingcheng === leixingmingcheng) || null;
    }

    // 获取所有类型的勾选实体（跨类型关联分析用）
    // 返回 [{leixing, zhi}]
    huoquSuoyouXuanzhong() {
        const jieguo = [];
        for (const [leixing, jihe] of Object.entries(this.xuanzhong_map)) {
            for (const zhi of jihe) {
                jieguo.push({ leixing, zhi });
            }
        }
        return jieguo;
    }
}
