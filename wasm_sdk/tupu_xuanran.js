// 图谱渲染模块 — 基于 force-graph 库
// CDN: https://esm.sh/force-graph@1 (自动解析 d3-force 等依赖)

let _ForceGraph = null;

async function _jiazaiKu() {
    if (_ForceGraph) return _ForceGraph;
    const mod = await import('https://esm.sh/force-graph@1');
    _ForceGraph = mod.default;
    return _ForceGraph;
}

const ZHUTI = [
    { zhu: '#6366F1', qian: '#EEF2FF', bian: '#C7D2FE' },
    { zhu: '#06B6D4', qian: '#ECFEFF', bian: '#A5F3FC' },
    { zhu: '#10B981', qian: '#ECFDF5', bian: '#A7F3D0' },
    { zhu: '#F59E0B', qian: '#FFFBEB', bian: '#FDE68A' },
    { zhu: '#EC4899', qian: '#FDF2F8', bian: '#FBCFE8' },
    { zhu: '#8B5CF6', qian: '#F5F3FF', bian: '#DDD6FE' },
    { zhu: '#EF4444', qian: '#FEF2F2', bian: '#FECACA' },
    { zhu: '#14B8A6', qian: '#F0FDFA', bian: '#99F6E4' },
];

const GUANXI_SECAI = ['#8B5CF6', '#EC4899', '#F59E0B', '#06B6D4', '#10B981', '#EF4444', '#6366F1', '#14B8A6'];

export class TupuXuanran {
    constructor() {
        this._graph = null;
        this._rongqi = null;
        this._xuanzhong = null;
        this._xuanzhong_link = null;
        this._linjiSet = new Set();
        this._zuihou_dianji = { shijian: 0, id: '' };
        this._fullscreen_handler = null;
        this._jiedian_shu = 0;
        this._du = {};
        this._zuidade = 1;
        this._leixingmap = {};
    }

    async xuanran(rongqi, jiedianlie, bianlie, guanxi_bianlie, zhongxinid, huidiao) {
        this.tingzhi();
        const ForceGraph = await _jiazaiKu();
        this._rongqi = rongqi;
        this._xuanzhong = null;
        this._xuanzhong_link = null;
        this._linjiSet.clear();

        const kuan = rongqi.clientWidth || 900;
        const gao = Math.max(700, window.innerHeight - 260);
        rongqi.innerHTML = '';
        rongqi.style.height = gao + 'px';
        rongqi.style.position = 'relative';

        // 准备数据
        const { nodes, links, leixingmap, guanxi_secai_map } = this._zhunbeishuju(jiedianlie, bianlie, guanxi_bianlie, zhongxinid);
        this._leixingmap = leixingmap;
        this._jiedian_shu = nodes.length;

        // 构建图谱
        const graph = new ForceGraph(rongqi)
            .width(kuan)
            .height(gao)
            .backgroundColor('#FAFBFC')
            .graphData({ nodes, links })
            .nodeId('id')
            .nodeVal(n => n._banjing * n._banjing * 0.5)
            .nodeLabel(() => null)
            .linkSource('source')
            .linkTarget('target')
            .linkCurvature('_qulv')
            .linkColor(l => this._lianjieyanse(l))
            .linkWidth(l => this._lianjiekuandu(l))
            .linkLineDash(l => l._leixing === 'guanxi' ? [4, 3] : null)
            .linkDirectionalArrowLength(0)
            .nodeCanvasObjectMode(() => 'replace')
            .nodeCanvasObject((node, ctx, globalScale) => this._huajiedian(node, ctx, globalScale))
            .nodePointerAreaPaint((node, color, ctx) => {
                ctx.beginPath();
                ctx.arc(node.x, node.y, node._banjing + 4, 0, Math.PI * 2);
                ctx.fillStyle = color;
                ctx.fill();
            })
            .linkCanvasObjectMode(l => l._leixing === 'guanxi' && l._guanxi ? 'after' : undefined)
            .linkCanvasObject((link, ctx, globalScale) => this._hualianjiebiaoji(link, ctx, globalScale))
            .warmupTicks(50)
            .cooldownTicks(1000)
            .d3AlphaDecay(0.02)
            .d3VelocityDecay(0.3)
            .enableNodeDrag(true)
            .enableZoomPanInteraction(true)
            .onNodeHover(node => {
                this._xuanzhong = node;
                this._gengxin_linji(node, links);
                this._gengxin_xinxi_jiedian(node);
                rongqi.style.cursor = node ? 'pointer' : 'grab';
            })
            .onLinkHover(link => {
                this._xuanzhong_link = link;
                this._gengxin_xinxi_lianjie(link);
                if (link && !this._xuanzhong) rongqi.style.cursor = 'pointer';
            })
            .onNodeClick((node) => {
                const xianzai = Date.now();
                if (xianzai - this._zuihou_dianji.shijian < 400 && node.id === this._zuihou_dianji.id) {
                    this._zuihou_dianji = { shijian: 0, id: '' };
                    if (huidiao.shuangji) huidiao.shuangji(node.id);
                } else {
                    this._zuihou_dianji = { shijian: xianzai, id: node.id };
                    if (huidiao.dianji_jiedian) {
                        const gx_lie = links
                            .filter(l => l._leixing === 'guanxi' && (l.source.id === node.id || l.target.id === node.id))
                            .map(l => ({
                                duifang: l.source.id === node.id ? l.target : l.source,
                                guanxi: l._guanxi, miaoshu: l._miaoshu, cishu: l._cishu, secai: l._secai
                            }));
                        huidiao.dianji_jiedian(node, gx_lie);
                    }
                }
            })
            .onLinkClick((link) => {
                if (link._leixing === 'guanxi' && huidiao.dianji_guanxi_bian) {
                    huidiao.dianji_guanxi_bian(link.source, link.target, link);
                } else if (link._leixing === 'gongxian' && huidiao.dianji_bian) {
                    huidiao.dianji_bian(link.source, link.target, link._quanzhong);
                }
            })
            .onBackgroundClick(() => {
                const celan = document.getElementById('tupu_celan');
                if (celan) celan.style.display = 'none';
            });

        // 配置力模型
        try {
            const chargeLi = graph.d3Force('charge');
            if (chargeLi) chargeLi.strength(-Math.max(200, nodes.length * 4));
            const linkLi = graph.d3Force('link');
            if (linkLi) linkLi.distance(Math.max(60, Math.sqrt(nodes.length) * 15));
        } catch (_) {}

        this._graph = graph;

        // 初始布局稳定后自适应缩放，确保所有节点可见
        setTimeout(() => { if (this._graph === graph) graph.zoomToFit(400, 40); }, 500);

        // 创建覆盖层元素
        this._chuangjian_gongju(rongqi, zhongxinid, huidiao, graph);
        this._chuangjian_tuli(rongqi, leixingmap, guanxi_secai_map);
        this._chuangjian_xinxi(rongqi);
        this._chuangjian_celan(rongqi);

        // 全屏处理
        this._fullscreen_handler = () => this._tiaozheng_quanping(rongqi, graph);
        document.addEventListener('fullscreenchange', this._fullscreen_handler);
    }

    tingzhi() {
        if (this._graph) {
            try {
                this._graph.pauseAnimation();
                if (typeof this._graph._destructor === 'function') this._graph._destructor();
            } catch (_) {}
            this._graph = null;
        }
        if (this._fullscreen_handler) {
            document.removeEventListener('fullscreenchange', this._fullscreen_handler);
            this._fullscreen_handler = null;
        }
        this._xuanzhong = null;
        this._xuanzhong_link = null;
        this._linjiSet.clear();
    }

    // ========== 数据准备 ==========

    _zhunbeishuju(jiedianlie, bianlie, guanxi_bianlie, zhongxinid) {
        const leixingmap = {};
        let leixingxuhao = 0;
        const guanxi_secai_map = {};
        let guanxi_secai_idx = 0;

        // 节点（先收集，再去重）
        const idset = new Set(jiedianlie.map(j => String(j.id ?? '')));
        const yuanshi_nodes = jiedianlie.map(j => {
            const id = String(j.id ?? '');
            const lx = j.leixingmingcheng || '';
            if (!(lx in leixingmap)) leixingmap[lx] = leixingxuhao++;
            return { id, zhi: j.zhi || '', leixing: lx, _banjing: 18 };
        });

        // 同名节点去重：优先保留真实标签（正ID），合并重复
        const _zhiMap = new Map();  // zhi → 保留的节点
        const _idRemap = {};        // 被合并的id → 保留的id
        for (const n of yuanshi_nodes) {
            const yiyou = _zhiMap.get(n.zhi);
            if (!yiyou) { _zhiMap.set(n.zhi, n); continue; }
            // 正ID（真实标签）优先于负ID（虚拟节点）
            const yiyou_zhenshi = !yiyou.id.startsWith('-');
            const n_zhenshi = !n.id.startsWith('-');
            if (n_zhenshi && !yiyou_zhenshi) {
                _idRemap[yiyou.id] = n.id;
                _zhiMap.set(n.zhi, n);
            } else {
                _idRemap[n.id] = yiyou.id;
            }
        }
        const nodes = [..._zhiMap.values()];
        const nodeIdSet = new Set(nodes.map(n => n.id));

        // 合并边（重映射被合并的ID，过滤无效边）
        const _remap = id => _idRemap[id] || id;
        const links = [];
        for (const b of bianlie) {
            const yuan = _remap(String(b.yuan)); const mubiao = _remap(String(b.mubiao));
            if (!nodeIdSet.has(yuan) || !nodeIdSet.has(mubiao) || yuan === mubiao) continue;
            links.push({
                source: yuan, target: mubiao,
                _leixing: 'gongxian',
                _quanzhong: parseInt(b.quanzhong) || 1,
                quanzhong: parseInt(b.quanzhong) || 1,
                _qulv: 0,
            });
        }
        for (const b of guanxi_bianlie) {
            const yuan = _remap(String(b.yuan)); const mubiao = _remap(String(b.mubiao));
            if (!nodeIdSet.has(yuan) || !nodeIdSet.has(mubiao) || yuan === mubiao) continue;
            const gx = b.guanxi || '';
            if (gx && !(gx in guanxi_secai_map)) guanxi_secai_map[gx] = guanxi_secai_idx++;
            const secai = gx ? GUANXI_SECAI[(guanxi_secai_map[gx] || 0) % GUANXI_SECAI.length] : GUANXI_SECAI[0];
            links.push({
                source: yuan, target: mubiao,
                _leixing: 'guanxi',
                _guanxi: gx, guanxi: gx,
                _miaoshu: b.miaoshu || '', miaoshu: b.miaoshu || '',
                _cishu: parseInt(b.cishu) || 1, cishu: parseInt(b.cishu) || 1,
                _secai: secai,
                _qulv: 0,
            });
        }

        // 多边曲率计算
        this._jisuanqulv(links);

        // 节点度数 → 半径
        const du = {};
        for (const n of nodes) du[n.id] = 0;
        for (const l of links) { du[l.source]++; du[l.target]++; }
        const zuidade = Math.max(1, ...Object.values(du));
        this._du = du;
        this._zuidade = zuidade;
        for (const n of nodes) {
            const jichu = zhongxinid && n.id === String(zhongxinid) ? 24 : 18;
            n._banjing = Math.round(jichu * (0.65 + 0.55 * (du[n.id] || 0) / zuidade));
        }

        return { nodes, links, leixingmap, guanxi_secai_map };
    }

    _jisuanqulv(links) {
        const suoyin = new Map();
        for (let i = 0; i < links.length; i++) {
            const a = links[i].source, b = links[i].target;
            const jian = a < b ? a + '_' + b : b + '_' + a;
            const zu = suoyin.get(jian) || [];
            zu.push(i);
            suoyin.set(jian, zu);
        }
        const step = 0.25;
        for (const [, zu] of suoyin) {
            if (zu.length === 1) { links[zu[0]]._qulv = 0; continue; }
            const n = zu.length;
            zu.forEach((idx, xuhao) => {
                links[idx]._qulv = (xuhao - (n - 1) / 2) * step;
            });
        }
    }

    // ========== 自定义渲染 ==========

    _huajiedian(node, ctx, globalScale) {
        if (!isFinite(node.x) || !isFinite(node.y)) return;
        const isXz = this._xuanzhong && this._xuanzhong.id === node.id;
        const isLinji = this._linjiSet.has(node.id);
        const t = ZHUTI[(this._leixingmap[node.leixing] || 0) % ZHUTI.length];
        const r = node._banjing;
        const jianhua = this._jiedian_shu > 25;
        const duoJiedian = this._jiedian_shu > 12;

        // 高亮光环
        if (isXz || isLinji) {
            ctx.beginPath();
            ctx.arc(node.x, node.y, r * (isXz ? 1.5 : 1.3), 0, Math.PI * 2);
            ctx.fillStyle = t.zhu + (isXz ? '18' : '0C');
            ctx.fill();
        }

        // 节点圆
        ctx.beginPath();
        ctx.arc(node.x, node.y, r, 0, Math.PI * 2);
        if (jianhua && !isXz && !isLinji) {
            ctx.fillStyle = t.qian;
        } else {
            const nG = ctx.createRadialGradient(node.x, node.y - r * 0.3, r * 0.1, node.x, node.y, r);
            nG.addColorStop(0, '#FFFFFF');
            nG.addColorStop(0.5, t.qian);
            nG.addColorStop(1, t.bian);
            ctx.fillStyle = nG;
        }
        ctx.fill();

        // 描边
        ctx.strokeStyle = t.zhu + (isXz ? 'DD' : isLinji ? 'AA' : '66');
        ctx.lineWidth = (isXz ? 2.5 : isLinji ? 2 : 1.2) / globalScale;
        ctx.stroke();

        // 标签
        let labelAlpha = 1;
        if (duoJiedian) {
            labelAlpha = this._xuanzhong
                ? (isXz ? 1 : isLinji ? 0.9 : 0.08)
                : ((this._du[node.id] || 0) >= Math.max(2, this._zuidade * 0.25) ? 0.95 : 0.12);
        }
        if (globalScale >= 0.4 && r > 3 / globalScale && labelAlpha > 0.08) {
            const fs = Math.max(10, 12) / globalScale;
            ctx.font = `500 ${fs}px -apple-system,"Microsoft YaHei",sans-serif`;
            const maxLW = Math.max(60, 140) / globalScale;
            let wenzi = node.zhi;
            if (wenzi.length > 10) wenzi = wenzi.slice(0, 10);
            let tw0 = ctx.measureText(wenzi).width;
            if (tw0 > maxLW) wenzi = wenzi.slice(0, Math.max(1, Math.floor(wenzi.length * (maxLW - 8 / globalScale) / tw0))) + '…';
            const tw = ctx.measureText(wenzi).width;
            const ppx = 7 / globalScale, pw = tw + ppx * 2, ph = fs + 6 / globalScale, prad = ph / 2;
            const ly = node.y + r + ph / 2 + 5 / globalScale;
            const pl = node.x - pw / 2, pt = ly - ph / 2;
            ctx.beginPath();
            ctx.roundRect(pl, pt, pw, ph, prad);
            ctx.fillStyle = `rgba(255,255,255,${(0.92 * labelAlpha).toFixed(2)})`;
            ctx.fill();
            if (labelAlpha > 0.5) {
                ctx.strokeStyle = `rgba(226,232,240,${(0.45 * labelAlpha).toFixed(2)})`;
                ctx.lineWidth = 0.5 / globalScale;
                ctx.stroke();
            }
            ctx.fillStyle = `rgba(15,23,42,${labelAlpha.toFixed(2)})`;
            ctx.textAlign = 'center';
            ctx.textBaseline = 'middle';
            ctx.fillText(wenzi, node.x, ly);
        }
    }

    _hualianjiebiaoji(link, ctx, globalScale) {
        if (globalScale < 0.6 || !link._guanxi) return;
        const a = link.source, c = link.target;
        if (!isFinite(a.x) || !isFinite(a.y) || !isFinite(c.x) || !isFinite(c.y)) return;
        const mx = (a.x + c.x) / 2, my = (a.y + c.y) / 2;
        // 曲线中点偏移
        const dx = c.x - a.x, dy = c.y - a.y;
        const elen = Math.sqrt(dx * dx + dy * dy) || 1;
        const qulv = link._qulv || 0;
        const lx = mx + (-dy / elen) * qulv * elen * 0.5;
        const ly = my + (dx / elen) * qulv * elen * 0.5;

        const lfs = Math.max(9, 10) / globalScale;
        ctx.font = `500 ${lfs}px -apple-system,"Microsoft YaHei",sans-serif`;
        const tw = ctx.measureText(link._guanxi).width;
        const lpp = 6 / globalScale, lph = lfs + 6 / globalScale;
        const rrx = lx - tw / 2 - lpp, rry = ly - lph / 2;

        const secai = link._secai || '#6B7280';
        const gaoliang = this._xuanzhong_link === link ||
            (this._xuanzhong && (this._xuanzhong.id === link.source.id || this._xuanzhong.id === link.target.id));

        ctx.beginPath();
        ctx.roundRect(rrx, rry, tw + lpp * 2, lph, lph / 2);
        ctx.fillStyle = gaoliang ? secai + '18' : 'rgba(255,255,255,0.9)';
        ctx.fill();
        ctx.strokeStyle = gaoliang ? secai + '40' : 'rgba(226,232,240,0.5)';
        ctx.lineWidth = 0.5 / globalScale;
        ctx.stroke();
        ctx.fillStyle = gaoliang ? secai : '#6B7280';
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillText(link._guanxi, lx, ly);
    }

    _lianjieyanse(link) {
        const gaoliang = this._xuanzhong &&
            (this._xuanzhong.id === (typeof link.source === 'object' ? link.source.id : link.source) ||
             this._xuanzhong.id === (typeof link.target === 'object' ? link.target.id : link.target));
        if (link._leixing === 'guanxi') {
            const secai = link._secai || GUANXI_SECAI[0];
            if (this._xuanzhong_link === link) return secai;
            if (gaoliang) return secai + '88';
            const ha = Math.round(Math.min(130, 50 + (link._cishu || 1) * 22)).toString(16).padStart(2, '0');
            return secai + ha;
        }
        // 共现边
        if (this._xuanzhong_link === link) return '#6366F1';
        const yuan_id = typeof link.source === 'object' ? link.source.id : link.source;
        if (gaoliang) {
            const xt = this._xuanzhong ? ZHUTI[(this._leixingmap[this._xuanzhong.leixing] || 0) % ZHUTI.length] : null;
            return xt ? xt.zhu + '66' : 'rgba(100,116,139,0.45)';
        }
        // 默认颜色：源节点类型色 + 透明度
        const yuanNode = typeof link.source === 'object' ? link.source : null;
        const t1 = yuanNode ? ZHUTI[(this._leixingmap[yuanNode.leixing] || 0) % ZHUTI.length] : ZHUTI[0];
        const ha = Math.round(Math.min(100, 35 + (link._quanzhong || 1) * 18)).toString(16).padStart(2, '0');
        return t1.zhu + ha;
    }

    _lianjiekuandu(link) {
        if (link._leixing === 'guanxi') {
            if (this._xuanzhong_link === link) return Math.min(4, 1.5 + (link._cishu || 1) * 0.3);
            return Math.min(2.5, 1.2 + (link._cishu || 1) * 0.2);
        }
        if (this._xuanzhong_link === link) return Math.min(4, 1.2 + (link._quanzhong || 1) * 0.6);
        const gaoliang = this._xuanzhong &&
            (this._xuanzhong.id === (typeof link.source === 'object' ? link.source.id : link.source) ||
             this._xuanzhong.id === (typeof link.target === 'object' ? link.target.id : link.target));
        if (gaoliang) return Math.min(3, 1 + (link._quanzhong || 1) * 0.4);
        return Math.min(2.5, 0.6 + (link._quanzhong || 1) * 0.3);
    }

    // ========== 交互 ==========

    _gengxin_linji(node, links) {
        this._linjiSet.clear();
        if (!node) return;
        for (const l of links) {
            const sid = typeof l.source === 'object' ? l.source.id : l.source;
            const tid = typeof l.target === 'object' ? l.target.id : l.target;
            if (sid === node.id) this._linjiSet.add(tid);
            if (tid === node.id) this._linjiSet.add(sid);
        }
    }

    _gengxin_xinxi_jiedian(node) {
        const xinxi = document.getElementById('tupu_xinxi');
        if (!xinxi) return;
        if (node) {
            xinxi.style.display = 'block';
            xinxi.innerHTML = `<b>${node.leixing}</b>: ${node.zhi}`;
        } else if (!this._xuanzhong_link) {
            xinxi.style.display = 'none';
        }
    }

    _gengxin_xinxi_lianjie(link) {
        const xinxi = document.getElementById('tupu_xinxi');
        if (!xinxi) return;
        if (link && !this._xuanzhong) {
            xinxi.style.display = 'block';
            const yuan = link.source, mubiao = link.target;
            if (link._leixing === 'guanxi') {
                xinxi.innerHTML = `<b>${yuan.zhi}</b> — <span style="color:${link._secai}">${link._guanxi}</span> — <b>${mubiao.zhi}</b> (${link._cishu}篇日报)`;
            } else {
                xinxi.innerHTML = `<b>${yuan.zhi}</b> ↔ <b>${mubiao.zhi}</b> (共现${link._quanzhong}次)`;
            }
        } else if (!this._xuanzhong) {
            xinxi.style.display = 'none';
        }
    }

    // ========== 覆盖层元素 ==========

    _chuangjian_gongju(rongqi, zhongxinid, huidiao, graph) {
        const _svg = {
            fangda: '<svg width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="8" y1="3.5" x2="8" y2="12.5"/><line x1="3.5" y1="8" x2="12.5" y2="8"/></svg>',
            suoxiao: '<svg width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="3.5" y1="8" x2="12.5" y2="8"/></svg>',
            chongzhi: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M2.5 8a5.5 5.5 0 1 1 1.1 3.3"/><polyline points="0.5 7.5 2.5 11.3 5.2 8.8"/></svg>',
            quanping: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><polyline points="2 6 2 2 6 2"/><polyline points="10 2 14 2 14 6"/><polyline points="14 10 14 14 10 14"/><polyline points="6 14 2 14 2 10"/></svg>',
            tuiquanping: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 2 6 6 2 6"/><polyline points="10 6 14 6 10 2"/><polyline points="10 14 10 10 14 10"/><polyline points="2 10 6 10 6 14"/></svg>',
            fanhui: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><line x1="13" y1="8" x2="3" y2="8"/><polyline points="7 4 3 8 7 12"/></svg>'
        };
        const mkBtn = (html, title, extra) => {
            const btn = document.createElement('button');
            btn.innerHTML = html;
            if (title) btn.title = title;
            btn.style.cssText = 'width:32px;height:32px;padding:0;display:inline-flex;align-items:center;justify-content:center;border:none;background:transparent;color:#64748B;border-radius:8px;cursor:pointer;transition:all 150ms ease;margin:0;min-height:0;font-size:12px';
            if (extra) Object.assign(btn.style, extra);
            btn.onmouseenter = () => { btn.style.background = '#F1F5F9'; btn.style.color = '#0F172A'; };
            btn.onmouseleave = () => { btn.style.background = 'transparent'; btn.style.color = '#64748B'; btn.style.transform = ''; };
            btn.onmousedown = () => { btn.style.transform = 'scale(0.9)'; };
            btn.onmouseup = () => { btn.style.transform = ''; };
            return btn;
        };
        const sep = () => {
            const d = document.createElement('div');
            d.style.cssText = 'width:1px;height:18px;background:rgba(148,163,184,0.2);margin:0 1px;flex-shrink:0';
            return d;
        };

        const bar = document.createElement('div');
        bar.style.cssText = 'position:absolute;top:12px;left:12px;display:flex;gap:2px;z-index:2;align-items:center;background:rgba(255,255,255,0.92);backdrop-filter:blur(20px);-webkit-backdrop-filter:blur(20px);border:1px solid rgba(226,232,240,0.45);border-radius:14px;padding:4px 5px;box-shadow:0 4px 24px rgba(0,0,0,0.06),0 1px 2px rgba(0,0,0,0.03)';

        if (zhongxinid) {
            const fb = mkBtn(_svg.fanhui + '<span style="margin-left:4px;font-weight:500">返回</span>', '返回全局视图');
            fb.style.width = 'auto';
            fb.style.padding = '0 10px';
            fb.onclick = () => { if (huidiao.fanhui) huidiao.fanhui(); };
            bar.append(fb, sep());
        }

        const btnSuoxiao = mkBtn(_svg.suoxiao, '缩小');
        btnSuoxiao.onclick = () => { const z = graph.zoom(); graph.zoom(z / 1.3, 200); };
        const zoomLabel = document.createElement('span');
        zoomLabel.style.cssText = 'font-size:11px;color:#94A3B8;min-width:38px;text-align:center;font-weight:600;font-variant-numeric:tabular-nums;user-select:none;letter-spacing:-0.3px';
        zoomLabel.textContent = '100%';
        const btnFangda = mkBtn(_svg.fangda, '放大');
        btnFangda.onclick = () => { const z = graph.zoom(); graph.zoom(z * 1.3, 200); };
        const btnChongzhi = mkBtn(_svg.chongzhi, '重置视图');
        btnChongzhi.onclick = () => { graph.centerAt(0, 0, 400); graph.zoom(1, 400); };
        const btnQuanping = mkBtn(_svg.quanping, '全屏');
        btnQuanping.onclick = () => {
            if (document.fullscreenElement === rongqi) {
                document.exitFullscreen();
            } else {
                rongqi.requestFullscreen().catch(() => {});
            }
        };
        bar.append(btnSuoxiao, zoomLabel, btnFangda, sep(), btnChongzhi, sep(), btnQuanping);
        rongqi.appendChild(bar);

        // 更新缩放百分比
        this._zoomLabelEl = zoomLabel;
        this._quanpingBtn = btnQuanping;
        this._quanpingSvg = _svg;
        graph.onZoom(() => {
            const pct = Math.round(graph.zoom() * 100) + '%';
            if (zoomLabel.textContent !== pct) zoomLabel.textContent = pct;
        });
    }

    _chuangjian_tuli(rongqi, leixingmap, guanxi_secai_map) {
        const tuli = document.createElement('div');
        tuli.style.cssText = 'position:absolute;top:12px;right:12px;background:rgba(255,255,255,0.7);backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px);border:1px solid rgba(226,232,240,0.6);border-radius:12px;padding:10px 14px;font-size:12px;display:flex;flex-wrap:wrap;gap:8px;z-index:2;max-width:260px;box-shadow:0 2px 12px rgba(0,0,0,0.04);color:#334155;font-weight:500';
        for (const [ming, xu] of Object.entries(leixingmap)) {
            const yanse = ZHUTI[xu % ZHUTI.length].zhu;
            tuli.innerHTML += `<span style="display:flex;align-items:center;gap:5px"><span style="width:8px;height:8px;border-radius:50%;background:${yanse};display:inline-block;flex-shrink:0;box-shadow:0 0 0 2px ${yanse}33"></span>${ming}</span>`;
        }
        let gx_html = '';
        for (const [gx, ci] of Object.entries(guanxi_secai_map)) {
            const sc = GUANXI_SECAI[ci % GUANXI_SECAI.length];
            gx_html += `<span style="display:flex;align-items:center;gap:4px"><span style="width:16px;border-top:2px dashed ${sc};display:inline-block;flex-shrink:0"></span><span style="color:${sc};font-size:10px">${gx}</span></span>`;
        }
        tuli.innerHTML += '<span style="display:flex;align-items:center;gap:5px;flex-wrap:wrap;border-top:1px solid #E2E8F0;padding-top:6px;margin-top:2px;width:100%">' +
            '<span style="width:16px;border-top:2px solid #94A3B8;display:inline-block;flex-shrink:0"></span><span style="color:#64748B;font-size:10px">共现</span>' +
            gx_html + '</span>';
        rongqi.appendChild(tuli);
    }

    _chuangjian_xinxi(rongqi) {
        const xinxi = document.createElement('div');
        xinxi.id = 'tupu_xinxi';
        xinxi.style.cssText = 'position:absolute;bottom:14px;left:50%;transform:translateX(-50%);background:rgba(255,255,255,0.92);backdrop-filter:blur(16px);-webkit-backdrop-filter:blur(16px);border:1px solid rgba(226,232,240,0.5);border-radius:12px;padding:10px 18px;font-size:13px;color:#0F172A;z-index:2;display:none;pointer-events:none;box-shadow:0 4px 20px rgba(0,0,0,0.06);font-weight:500;max-width:560px;line-height:1.6;white-space:nowrap';
        rongqi.appendChild(xinxi);
    }

    _chuangjian_celan(rongqi) {
        const celan = document.createElement('div');
        celan.id = 'tupu_celan';
        celan.style.cssText = 'position:absolute;right:0;top:0;width:360px;height:100%;background:rgba(255,255,255,0.88);backdrop-filter:blur(16px);-webkit-backdrop-filter:blur(16px);border-left:1px solid rgba(226,232,240,0.5);z-index:10;overflow-y:auto;display:none;box-shadow:-4px 0 24px rgba(0,0,0,0.06);transition:transform 200ms ease,opacity 200ms ease;transform:translateX(0)';
        rongqi.appendChild(celan);
    }

    _tiaozheng_quanping(rongqi, graph) {
        const shifouquanping = document.fullscreenElement === rongqi;
        const kuan = shifouquanping ? window.innerWidth : (rongqi.clientWidth || 900);
        const gao = shifouquanping ? window.innerHeight : Math.max(700, window.innerHeight - 260);

        graph.width(kuan).height(gao);

        if (shifouquanping) {
            rongqi.style.height = '100%';
            rongqi.style.borderRadius = '0';
            rongqi.style.border = 'none';
        } else {
            rongqi.style.height = gao + 'px';
            rongqi.style.borderRadius = '12px';
            rongqi.style.border = '1px solid #E2E8F0';
        }
        if (this._quanpingBtn && this._quanpingSvg) {
            this._quanpingBtn.innerHTML = shifouquanping ? this._quanpingSvg.tuiquanping : this._quanpingSvg.quanping;
            this._quanpingBtn.title = shifouquanping ? '退出全屏' : '全屏';
        }
    }
}
