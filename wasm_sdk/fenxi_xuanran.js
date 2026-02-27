// 分析视图 - 渲染层（纯 HTML 生成，不操作 DOM）

function jiexishijian(canchuo) {
    const ms = Number(canchuo);
    if (!ms || isNaN(ms)) return canchuo || '';
    const d = new Date(ms);
    const pad = n => String(n).padStart(2, '0');
    return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`;
}

// ========== 侧栏列表渲染 ==========

export function xuanran_fenxi_liebiao(liebiao, leixing, guanlianfenxi = false) {
    if (!liebiao || liebiao.length === 0) return '<div style="padding:10px;color:#94A3B8;font-size:13px">暂无数据</div>';
    let html = '';
    for (const xiang of liebiao) {
        const mingcheng = xiang.zhi || '';
        const shu = xiang.ribao_shu || 0;
        html += `<div class="fenxi-item fenxi_liebiao_xiang" data-leixing="${leixing}" data-mingcheng="${mingcheng}">`;
        if (guanlianfenxi) {
            html += `<input type="checkbox" class="fenxi-checkbox fenxi_shiti_xz" data-leixing="${leixing}" data-mingcheng="${mingcheng}" onchange="ribao_fenxi_shiti_gouxuan(this)" onclick="event.stopPropagation()">`;
        }
        const mc_escaped = mingcheng.replace(/'/g, "\\'");
        html += `<div class="fenxi-item-main" onclick="ribao_fenxi_jiaoliu('${leixing}','${mc_escaped}')">`;
        html += `<div class="fenxi-item-title">${mingcheng}</div>`;
        html += `<div class="fenxi-item-sub">${shu} 篇日报</div>`;
        html += '</div>';
        html += '</div>';
    }
    return html;
}

// ========== 主视图：实体交流详情页 ==========

export function xuanran_shiti_xiangqing(shiti_mingcheng, shiti_leixing, ribaolie, biaoqianlie) {
    let html = `<div class="fenxi-title">${shiti_mingcheng}</div>`;
    html += `<div class="fenxi-sub">类型：${shiti_leixing}　日报：${ribaolie.length} 篇</div>`;
    // 维度选择器
    html += `<div style="margin-bottom:12px">`;
    html += `<div style="font-size:12px;font-weight:700;color:#475569;margin-bottom:8px">选择分析维度</div>`;
    html += `<div id="fenxi_weidu_qu" style="display:flex;flex-wrap:wrap;gap:6px;margin-bottom:10px">`;
    for (const wd of MOREN_WEIDU_LIEBIAO) {
        const se = WEIDU_YANSE[wd] || { dot: '#475569' };
        const wd_esc = wd.replace(/'/g, "\\'");
        html += `<label style="display:inline-flex;align-items:center;gap:5px;padding:5px 12px;background:#F8FAFC;border:1px solid #E2E8F0;border-radius:999px;cursor:pointer;font-size:12px;color:#334155;transition:all 150ms;user-select:none" onmouseenter="this.style.borderColor='#93C5FD'" onmouseleave="this.style.borderColor=this.querySelector('input').checked?'#3B82F6':'#E2E8F0'">`;
        html += `<input type="checkbox" class="fenxi_weidu_xz" value="${wd}" checked style="width:14px;height:14px;accent-color:${se.dot};cursor:pointer">`;
        html += `<span style="display:inline-block;width:8px;height:8px;border-radius:50%;background:${se.dot};flex-shrink:0"></span>`;
        html += `${wd}`;
        html += `</label>`;
    }
    html += `</div>`;
    // 自定义维度输入
    html += `<div style="display:flex;gap:8px;align-items:center">`;
    html += `<input id="fenxi_zidingyi_weidu" type="text" placeholder="输入自定义分析维度，如：技术方案分析" style="flex:1;height:34px;padding:0 12px;border:1px solid #E2E8F0;border-radius:8px;font-size:13px;outline:none" onkeydown="if(event.key==='Enter')ribao_fenxi_tianjia_weidu()">`;
    html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_fenxi_tianjia_weidu()" style="height:34px">添加</button>`;
    html += `</div>`;
    html += `</div>`;
    // 操作栏
    html += `<div class="fenxi-actions">`;
    html += `<button id="fenxi_kaishi_btn" class="aq-btn aq-btn-zhu" onclick="ribao_fenxi_kaishi_fenxi()">开始分析选中维度</button>`;
    html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_fenxi_quanxuan_weidu(true)" style="height:34px">全选</button>`;
    html += `<button class="aq-btn aq-btn-xiao" onclick="ribao_fenxi_quanxuan_weidu(false)" style="height:34px">全不选</button>`;
    html += `<button id="fenxi_tingzhi_btn" class="aq-btn aq-btn-hong" onclick="ribao_fenxi_tingzhi_fenxi()" style="display:none">停止分析</button>`;
    html += `<span id="fenxi_zhuangtai" class="fenxi-status"></span>`;
    html += `</div>`;

    // 关联标签
    if (biaoqianlie.length > 0) {
        html += '<div class="fenxi-section">';
        html += '<div class="fenxi-section-h">关联标签</div>';
        html += '<div style="display:flex;flex-wrap:wrap;gap:8px">';
        for (const bq of biaoqianlie) {
            html += `<span class="fenxi-tag">${bq.leixingmingcheng}：${bq.zhi}${bq.cishu > 1 ? ' (' + bq.cishu + ')' : ''}</span>`;
        }
        html += '</div></div>';
    }

    // 日报原文区（折叠式）
    if (ribaolie.length > 0) {
        html += '<div class="fenxi-section">';
        html += `<div class="fenxi-section-h">关联日报（${ribaolie.length}篇）</div>`;
        html += '<div class="fenxi-ribao-list">';
        for (const rb of ribaolie) {
            const riqi = jiexishijian(rb.fabushijian);
            const biaoti = rb.biaoti || '无标题';
            const zhaiyao = rb.zhaiyao || '';
            const neirong = rb.neirong || '';
            html += `<details class="fenxi-ribao">`;
            html += `<summary>`;
            html += `<div class="fenxi-ribao-head">`;
            html += `<div class="fenxi-ribao-title">${biaoti}${zhaiyao ? `<span class="fenxi-ribao-zhaiyao">${zhaiyao}</span>` : ''}</div>`;
            html += `<div class="fenxi-ribao-meta">${riqi}</div>`;
            html += `</div>`;
            html += `</summary>`;
            html += `<div class="fenxi-ribao-body">${neirong}</div>`;
            html += `</details>`;
        }
        html += '</div></div>';
    }
    if (ribaolie.length === 0 && biaoqianlie.length === 0) {
        html += '<div style="color:#94A3B8;font-size:13px">暂无关联日报</div>';
    }

    // AI 分析结果占位
    html += '<div id="fenxi_ai_jieguo"></div>';
    return html;
}

// ========== AI 深度分析卡片 ==========

// 默认维度列表（可选）
export const MOREN_WEIDU_LIEBIAO = [
    '综合概况', '关键问题诊断', '风险评估', '人员协作分析',
    '客户关系评估', '竞争态势', '行动建议',
];

const WEIDU_YANSE = {
    '综合概况': { dot: '#2563EB' },
    '关键问题诊断': { dot: '#DC2626' },
    '风险评估': { dot: '#B45309' },
    '人员协作分析': { dot: '#16A34A' },
    '客户关系评估': { dot: '#7E22CE' },
    '竞争态势': { dot: '#C2410C' },
    '行动建议': { dot: '#0369A1' },
};

export function xuanran_shendu_kapian(weidu, fenxi) {
    const se = WEIDU_YANSE[weidu] || { dot: '#475569' };
    let html = `<div class="fenxi-ai-card">`;
    html += `<div class="fenxi-ai-card-tou">`;
    html += `<div class="fenxi-ai-card-h"><span class="fenxi-dot" style="background:${se.dot}"></span><div class="fenxi-ai-title">${weidu}</div></div>`;
    html += `</div>`;
    html += `<div class="fenxi-ai-body">`;
    html += xuanran_fenxi_jieguo_shipei(fenxi);
    html += `</div>`;
    html += `</div>`;
    return html;
}

// ========== AI 分析结果自适应渲染 ==========

export function xuanran_fenxi_jieguo_shipei(fenxi) {
    if (!fenxi || typeof fenxi !== 'object') {
        return xuanran_tongyong_json(fenxi);
    }
    // 如果 AI 用了 Report X 等包裹键，尝试解包
    if (!fenxi.zhutihuizong && !fenxi.guanjianwenti && !fenxi.jianyi && !fenxi.yanbianguiji) {
        const jianlie = Object.keys(fenxi);
        if (jianlie.length === 1) {
            const neibu = fenxi[jianlie[0]];
            if (neibu && typeof neibu === 'object' && !Array.isArray(neibu)) {
                if (neibu.zhutihuizong || neibu.guanjianwenti || neibu.jianyi || neibu.yanbianguiji) {
                    fenxi = neibu;
                }
            }
        } else {
            const diyi = fenxi[jianlie[0]];
            if (diyi && typeof diyi === 'object' && !Array.isArray(diyi) &&
                (diyi.zhutihuizong || diyi.guanjianwenti || diyi.jianyi || diyi.yanbianguiji)) {
                const hebing = { zhutihuizong: [], guanjianwenti: [], yanbianguiji: '', jianyi: '' };
                for (const jian of jianlie) {
                    const zi = fenxi[jian];
                    if (!zi || typeof zi !== 'object') continue;
                    if (Array.isArray(zi.zhutihuizong)) hebing.zhutihuizong.push(...zi.zhutihuizong);
                    if (Array.isArray(zi.guanjianwenti)) hebing.guanjianwenti.push(...zi.guanjianwenti);
                    if (zi.yanbianguiji) hebing.yanbianguiji += (hebing.yanbianguiji ? '\n' : '') + zi.yanbianguiji;
                    if (zi.jianyi) hebing.jianyi += (hebing.jianyi ? '\n' : '') + zi.jianyi;
                }
                fenxi = hebing;
            }
        }
    }
    const you_biaozhun = (fenxi.zhutihuizong || fenxi.guanjianwenti || fenxi.jianyi || fenxi.yanbianguiji);
    if (!you_biaozhun) {
        return xuanran_tongyong_json(fenxi);
    }

    let html = '';

    // 主题汇总
    if (Array.isArray(fenxi.zhutihuizong) && fenxi.zhutihuizong.length > 0) {
        html += '<div class="fenxi-kv">';
        html += '<div class="fenxi-k">主题汇总</div>';
        for (const zt of fenxi.zhutihuizong) {
            if (typeof zt === 'string') {
                html += `<div class="fenxi-v" style="padding:10px 12px;border:1px solid #E2E8F0;border-radius:12px;background:#FFFFFF;margin-bottom:8px">${zt}</div>`;
            } else {
                const cishu = zt.cishu ?? '';
                const zhutiming = zt.zhuti || zt.mingcheng || '';
                const zhutiMiaoshu = zt.miaoshu || zt.neirong || '';
                html += `<div style="padding:10px 12px;border:1px solid #E2E8F0;border-radius:12px;background:#FFFFFF;margin-bottom:8px">`;
                html += `<div style="display:flex;gap:10px;align-items:center;justify-content:space-between">`;
                html += `<div style="font-weight:800;color:#0F172A;font-size:13px;line-height:1.4">${zhutiming}</div>`;
                html += cishu ? `<span class="fenxi-badge" style="background:#F1F5F9;color:#475569;border:1px solid #E2E8F0">出现${cishu}次</span>` : '';
                html += `</div>`;
                if (zhutiMiaoshu) html += `<div class="fenxi-v" style="margin-top:6px;color:#475569">${zhutiMiaoshu}</div>`;
                if (Array.isArray(zt.shejiXiangmu) && zt.shejiXiangmu.length > 0) {
                    html += '<div style="display:flex;flex-wrap:wrap;gap:4px;margin-top:6px">';
                    for (const xm of zt.shejiXiangmu) {
                        html += `<span style="display:inline-block;padding:2px 8px;background:#F1F5F9;border:1px solid #E2E8F0;border-radius:10px;font-size:11px;color:#475569">${xm}</span>`;
                    }
                    html += '</div>';
                }
                html += `</div>`;
            }
        }
        html += '</div>';
    }

    // 演变轨迹
    if (fenxi.yanbianguiji) {
        html += '<div class="fenxi-kv">';
        html += '<div class="fenxi-k">演变轨迹</div>';
        html += `<div class="fenxi-v" style="padding:10px 12px;border:1px solid #E2E8F0;border-radius:12px;background:#FFFFFF">${String(fenxi.yanbianguiji)}</div>`;
        html += '</div>';
    }

    // 关键问题
    if (Array.isArray(fenxi.guanjianwenti) && fenxi.guanjianwenti.length > 0) {
        html += '<div class="fenxi-kv">';
        html += '<div class="fenxi-k">关键问题</div>';
        for (const wt of fenxi.guanjianwenti) {
            if (typeof wt === 'string') {
                html += `<div class="fenxi-issue"><div class="fenxi-issue-text">${wt}</div></div>`;
                continue;
            }
            const cd = (wt.yanzhongchengdu || '').trim();
            const cdClass = cd === '高' ? 'fenxi-badge-gao' : cd === '中' ? 'fenxi-badge-zhong' : cd === '低' ? 'fenxi-badge-di' : '';
            const wenben = wt.wenti || wt.neirong || '';
            html += `<div class="fenxi-issue" style="flex-direction:column;gap:6px">`;
            html += `<div style="display:flex;align-items:flex-start;gap:10px;justify-content:space-between">`;
            html += `<div class="fenxi-issue-text">${wenben}</div>`;
            html += cd ? `<span class="fenxi-badge ${cdClass}" style="flex-shrink:0">严重程度：${cd}</span>` : '';
            html += `</div>`;
            if (Array.isArray(wt.shejiXiangmu) && wt.shejiXiangmu.length > 0) {
                html += '<div style="display:flex;flex-wrap:wrap;gap:4px">';
                for (const xm of wt.shejiXiangmu) {
                    html += `<span style="display:inline-block;padding:2px 8px;background:#F1F5F9;border:1px solid #E2E8F0;border-radius:10px;font-size:11px;color:#475569">${xm}</span>`;
                }
                html += '</div>';
            }
            html += `</div>`;
        }
        html += '</div>';
    }

    // 建议
    if (fenxi.jianyi) {
        html += '<div class="fenxi-kv">';
        html += '<div class="fenxi-k">建议</div>';
        const jianyi_wenben = String(fenxi.jianyi);
        const jianyi_tiao = jianyi_wenben.split(/(?=\d+[.\u3001])/).filter(s => s.trim());
        if (jianyi_tiao.length > 1) {
            html += '<ol style="margin:0;padding-left:18px">';
            for (const tiao of jianyi_tiao) {
                const neirong = tiao.replace(/^\d+[.\u3001]\s*/, '').trim();
                if (neirong) html += `<li class="fenxi-v">${neirong}</li>`;
            }
            html += '</ol>';
        } else {
            html += `<div class="fenxi-v">${jianyi_wenben}</div>`;
        }
        html += '</div>';
    }

    return html || xuanran_tongyong_json(fenxi);
}

// ========== 通用 JSON 渲染 ==========

export function xuanran_tongyong_json(obj) {
    if (typeof obj === 'string' || typeof obj === 'number' || typeof obj === 'boolean') {
        return `<div class="fenxi-v">${String(obj)}</div>`;
    }
    if (Array.isArray(obj)) {
        if (obj.length === 0) return '';
        let html = '<div style="display:flex;flex-direction:column;gap:8px">';
        for (const xiang of obj) {
            if (typeof xiang === 'string' || typeof xiang === 'number' || typeof xiang === 'boolean') {
                html += `<div class="fenxi-v" style="padding:10px 12px;border:1px solid #E2E8F0;border-radius:12px;background:#FFFFFF">${String(xiang)}</div>`;
            } else if (typeof xiang === 'object' && xiang !== null) {
                html += `<div style="padding:10px 12px;border:1px solid #E2E8F0;border-radius:12px;background:#FFFFFF">`;
                html += xuanran_tongyong_json(xiang);
                html += '</div>';
            }
        }
        html += '</div>';
        return html;
    }
    if (typeof obj === 'object' && obj !== null) {
        let html = '';
        for (const [jian, zhi] of Object.entries(obj)) {
            const biaoti = jian.replace(/_/g, ' ');
            html += `<div class="fenxi-kv">`;
            html += `<div class="fenxi-k">${biaoti}</div>`;
            html += xuanran_tongyong_json(zhi);
            html += `</div>`;
        }
        return html;
    }
    return '';
}

// ========== 交流分析报告卡片 ==========

export function xuanran_jiaoliu_fenxi_jieguo(fenxi) {
    let html = '<div class="fenxi-ai-card" style="background:#EFF6FF;border-color:#BFDBFE">';
    html += '<div class="fenxi-ai-card-tou"><div class="fenxi-ai-card-h"><span class="fenxi-dot" style="background:#2563EB"></span><div class="fenxi-ai-title">分析报告</div></div></div>';
    // 主题汇总
    if (fenxi.zhutihuizong && fenxi.zhutihuizong.length > 0) {
        html += '<div style="margin-bottom:12px"><div style="font-size:13px;font-weight:600;color:#1E40AF;margin-bottom:6px">主题汇总</div>';
        html += '<div style="display:flex;flex-direction:column;gap:8px">';
        for (const zt of fenxi.zhutihuizong) {
            if (typeof zt === 'string') {
                html += `<div style="padding:8px 12px;background:#FFF;border:1px solid #DBEAFE;border-radius:6px;font-size:13px;color:#334155;line-height:1.5">${zt}</div>`;
            } else {
                html += '<div style="padding:8px 12px;background:#FFF;border:1px solid #DBEAFE;border-radius:6px">';
                html += `<div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:4px">`;
                html += `<span style="font-size:13px;font-weight:600;color:#1E293B">${zt.zhuti || ''}</span>`;
                if (zt.cishu) html += `<span style="font-size:11px;color:#64748B;background:#F1F5F9;padding:2px 8px;border-radius:10px">出现${zt.cishu}次</span>`;
                html += '</div>';
                if (zt.miaoshu) html += `<div style="font-size:12px;color:#475569;line-height:1.5">${zt.miaoshu}</div>`;
                html += '</div>';
            }
        }
        html += '</div></div>';
    }
    // 演变轨迹
    if (fenxi.yanbianguiji) {
        html += `<div style="margin-bottom:12px"><div style="font-size:13px;font-weight:600;color:#1E40AF;margin-bottom:4px">演变轨迹</div><p style="margin:0;font-size:13px;color:#334155;line-height:1.6">${fenxi.yanbianguiji}</p></div>`;
    }
    // 关键问题
    if (fenxi.guanjianwenti && fenxi.guanjianwenti.length > 0) {
        html += '<div style="margin-bottom:12px"><div style="font-size:13px;font-weight:600;color:#1E40AF;margin-bottom:6px">关键问题</div>';
        html += '<div style="display:flex;flex-direction:column;gap:6px">';
        const yzcd_yanse = { '高': '#DC2626', '中': '#D97706', '低': '#059669' };
        const yzcd_beijing = { '高': '#FEF2F2', '中': '#FFFBEB', '低': '#F0FDF4' };
        const yzcd_biankuang = { '高': '#FECACA', '中': '#FDE68A', '低': '#BBF7D0' };
        for (const wt of fenxi.guanjianwenti) {
            if (typeof wt === 'string') {
                html += `<div style="padding:8px 12px;background:#FFF;border:1px solid #DBEAFE;border-radius:6px;font-size:13px;color:#334155;line-height:1.5">${wt}</div>`;
            } else {
                const cd = wt.yanzhongchengdu || '';
                const bgc = yzcd_beijing[cd] || '#FFF';
                const bdc = yzcd_biankuang[cd] || '#E2E8F0';
                const txc = yzcd_yanse[cd] || '#475569';
                const wenben = wt.wenti || wt.neirong || '';
                html += `<div style="padding:8px 12px;background:${bgc};border:1px solid ${bdc};border-radius:6px">`;
                html += `<div style="display:flex;justify-content:space-between;align-items:flex-start">`;
                html += `<span style="font-size:13px;color:#1E293B;line-height:1.5;flex:1">${wenben}</span>`;
                if (cd) html += `<span style="font-size:11px;font-weight:600;color:${txc};margin-left:8px;white-space:nowrap;flex-shrink:0">严重程度：${cd}</span>`;
                html += '</div>';
                if (Array.isArray(wt.shejiXiangmu) && wt.shejiXiangmu.length > 0) {
                    html += '<div style="display:flex;flex-wrap:wrap;gap:4px;margin-top:6px">';
                    for (const xm of wt.shejiXiangmu) {
                        html += `<span style="display:inline-block;padding:2px 8px;background:rgba(255,255,255,0.7);border:1px solid ${bdc};border-radius:10px;font-size:11px;color:#475569">${xm}</span>`;
                    }
                    html += '</div>';
                }
                html += '</div>';
            }
        }
        html += '</div></div>';
    }
    // 建议
    if (fenxi.jianyi) {
        html += '<div><div style="font-size:13px;font-weight:600;color:#1E40AF;margin-bottom:4px">建议</div>';
        const jianyi_wenben = String(fenxi.jianyi);
        const jianyi_tiao = jianyi_wenben.split(/(?=\d+[.\u3001])/).filter(s => s.trim());
        if (jianyi_tiao.length > 1) {
            html += '<ol style="margin:0;padding-left:18px;font-size:13px;color:#334155;line-height:1.6">';
            for (const tiao of jianyi_tiao) {
                const neirong = tiao.replace(/^\d+[.\u3001]\s*/, '').trim();
                if (neirong) html += `<li>${neirong}</li>`;
            }
            html += '</ol>';
        } else {
            html += `<p style="margin:0;font-size:13px;color:#334155;line-height:1.6">${jianyi_wenben}</p>`;
        }
        html += '</div>';
    }
    html += '</div>';
    return html;
}

// ========== 关联分析报告卡片 ==========

export function xuanran_xiangmu_guanlian_jieguo(fenxi) {
    let html = '<div class="fenxi-ai-card" style="background:#FFFBEB;border-color:#FDE68A">';
    html += '<div class="fenxi-ai-card-tou"><div class="fenxi-ai-card-h"><span class="fenxi-dot" style="background:#B45309"></span><div class="fenxi-ai-title">关联分析报告</div></div></div>';
    if (fenxi.xiangmuguanxi && fenxi.xiangmuguanxi.length > 0) {
        html += '<div class="fenxi-kv"><div class="fenxi-k">项目关联</div>';
        for (const gx of fenxi.xiangmuguanxi) {
            html += `<div style="padding:10px 12px;margin-bottom:8px;background:#FFFFFF;border:1px solid #FDE68A;border-radius:12px">`;
            html += `<div style="font-size:13px;font-weight:800;color:#0F172A">${gx.xm1 || ''} ↔ ${gx.xm2 || ''}</div>`;
            if (gx.guanxi) html += `<div class="fenxi-v" style="margin-top:4px">关系：${gx.guanxi}</div>`;
            if (gx.gongxiangziyuan && gx.gongxiangziyuan.length > 0) {
                html += `<div class="fenxi-v" style="margin-top:4px;color:#64748B">共享资源：${gx.gongxiangziyuan.join('、')}</div>`;
            }
            if (gx.miaoshu) html += `<div class="fenxi-v" style="margin-top:6px">${gx.miaoshu}</div>`;
            html += '</div>';
        }
        html += '</div>';
    }
    if (fenxi.fengxiantishi && fenxi.fengxiantishi.length > 0) {
        html += '<div class="fenxi-kv"><div class="fenxi-k" style="color:#B91C1C">风险提示</div>';
        html += '<ul style="margin:0;padding-left:18px">';
        for (const fx of fenxi.fengxiantishi) html += `<li class="fenxi-v">${typeof fx === 'string' ? fx : JSON.stringify(fx)}</li>`;
        html += '</ul></div>';
    }
    html += '</div>';
    return html;
}
