// 标签管理 - 界面层
export class Biaoqianjiemian {
    constructor(luoji, rongqiid) {
        this.luoji = luoji;
        this.rongqi = document.getElementById(rongqiid);
        this.dangqianshitu = 'leixing';
        this.xuanzhongleixingid = null;
    }

    async chushihua() {
        this.rongqi.innerHTML = '';
        const tou = document.createElement('div');
        tou.style.cssText = 'display:flex;justify-content:space-between;align-items:center;margin-bottom:12px;flex-wrap:wrap;gap:8px';
        tou.innerHTML = `<h2 style="font-size:15px;color:#475569;margin:0">标签管理</h2>
            <div style="display:flex;gap:8px;align-items:center;flex-wrap:wrap">
                <button class="aq-btn aq-btn-zhu" onclick="biaoqian_qiehuanshitu('leixing')">类型管理</button>
                <button class="aq-btn aq-btn-zhu" onclick="biaoqian_qiehuanshitu('biaoqian')">标签管理</button>
                <button class="aq-btn aq-btn-lv" onclick="biaoqian_shuaxin()">刷新数据</button>
                <span id="biaoqian_caozuo"></span>
            </div>`;
        this.rongqi.appendChild(tou);
        const neirong = document.createElement('div');
        neirong.id = 'biaoqian_neirong';
        neirong.innerHTML = '<p style="color:#94A3B8;font-size:14px">点击「刷新数据」加载标签信息</p>';
        this.rongqi.appendChild(neirong);
    }

    async shuaxin() {
        await this.qiehuanshitu(this.dangqianshitu);
    }

    gengxincaozuo(html) {
        const el = document.getElementById('biaoqian_caozuo');
        if (el) el.innerHTML = html;
    }

    async qiehuanshitu(shitu) {
        this.dangqianshitu = shitu;
        shitu === 'leixing' ? await this.xianshileixing() : await this.xianshibibaoqian();
    }

    async xianshileixing() {
        this.gengxincaozuo('<button class="aq-btn aq-btn-lv" onclick="biaoqian_xinzengleixing()">新增类型</button>');
        const nr = document.getElementById('biaoqian_neirong');
        nr.innerHTML = '<p style="color:#64748B">加载中...</p>';
        const jg = await this.luoji.leixing_chaxun_quanbu();
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = `<p style="color:#EF4444">${jg?.xiaoxi || '加载失败'}</p>`;
            return;
        }
        const lie = jg.shuju || [];
        let html = '';
        if (lie.length === 0) {
            html = '<p style="color:#94A3B8">暂无类型</p>';
        } else {
            html = '<div style="display:grid;gap:12px">';
            for (const x of lie) {
                html += `<div style="padding:12px;background:#F8FAFC;border-radius:8px;display:flex;justify-content:space-between;align-items:center">
                    <span style="font-size:14px;color:#1E293B">${x.mingcheng}</span>
                    <div style="display:flex;gap:8px">
                        <button class="aq-btn aq-btn-xiao" onclick="biaoqian_bianjibiaoqian('${x.id}')">标签</button>
                        <button class="aq-btn aq-btn-xiao" onclick="biaoqian_bianjileixing('${x.id}')">编辑</button>
                        <button class="aq-btn aq-btn-xiao" onclick="biaoqian_shanchuleixing('${x.id}')" style="background:#FEE2E2;color:#DC2626">删除</button>
                    </div>
                </div>`;
            }
            html += '</div>';
        }
        nr.innerHTML = html;
    }

    async xianshibibaoqian() {
        this.gengxincaozuo('<button class="aq-btn aq-btn-lv" onclick="biaoqian_xinzengbiaoqian()">新增标签</button>');
        const nr = document.getElementById('biaoqian_neirong');
        nr.innerHTML = '<p style="color:#64748B">加载中...</p>';
        const leixingjg = await this.luoji.leixing_chaxun_quanbu();
        if (!leixingjg || leixingjg.zhuangtaima !== 200) {
            nr.innerHTML = `<p style="color:#EF4444">类型加载失败</p>`;
            return;
        }
        const biaoqianjg = await this.luoji.biaoqian_chaxun_quanbu();
        if (!biaoqianjg || biaoqianjg.zhuangtaima !== 200) {
            nr.innerHTML = `<p style="color:#EF4444">标签加载失败</p>`;
            return;
        }
        const leixinglie = leixingjg.shuju || [];
        const biaoqianlie = biaoqianjg.shuju || [];
        const leixingmap = Object.fromEntries(leixinglie.map(x => [x.id, x.mingcheng]));
        let html = '';
        if (biaoqianlie.length === 0) {
            html = '<p style="color:#94A3B8">暂无标签</p>';
        } else {
            html = '<div style="display:grid;gap:12px">';
            for (const bq of biaoqianlie) {
                const leixingming = leixingmap[bq.leixingid] || '未知类型';
                html += `<div style="padding:12px;background:#F8FAFC;border-radius:8px;display:flex;justify-content:space-between;align-items:center">
                    <div><span style="font-size:14px;color:#1E293B">${bq.zhi}</span><span style="margin-left:8px;font-size:12px;color:#64748B">[${leixingming}]</span></div>
                    <div style="display:flex;gap:8px">
                        <button class="aq-btn aq-btn-xiao" onclick="biaoqian_bianjibiaoqian_danxiang('${bq.id}')">编辑</button>
                        <button class="aq-btn aq-btn-xiao" onclick="biaoqian_shanchubiaoqian('${bq.id}')" style="background:#FEE2E2;color:#DC2626">删除</button>
                    </div>
                </div>`;
            }
            html += '</div>';
        }
        nr.innerHTML = html;
    }

    async xinzengleixing() {
        const mingcheng = prompt('输入类型名称：');
        if (!mingcheng) return;
        const jg = await this.luoji.leixing_xinzeng(mingcheng);
        if (jg && jg.zhuangtaima === 200) this.xianshileixing();
    }

    async bianjileixing(id) {
        const jg = await this.luoji.leixing_chaxun_id(id);
        if (!jg || jg.zhuangtaima !== 200) return;
        const mingcheng = prompt('修改类型名称：', jg.shuju.mingcheng);
        if (!mingcheng) return;
        const gxjg = await this.luoji.leixing_gengxin(id, mingcheng);
        if (gxjg && gxjg.zhuangtaima === 200) this.xianshileixing();
    }

    async shanchuleixing(id) {
        if (!confirm('确认删除此类型？')) return;
        const jg = await this.luoji.leixing_shanchu(id);
        if (jg && jg.zhuangtaima === 200) this.xianshileixing();
    }

    async bianjibiaoqian(leixingid) {
        this.xuanzhongleixingid = leixingid;
        this.gengxincaozuo(`<button class="aq-btn aq-btn-lv" onclick="biaoqian_xinzengbiaoqian_leixing('${leixingid}')">新增标签</button><button class="aq-btn" onclick="biaoqian_fanhui()">返回</button>`);
        const nr = document.getElementById('biaoqian_neirong');
        nr.innerHTML = '<p style="color:#64748B">加载中...</p>';
        const jg = await this.luoji.biaoqian_chaxun_leixingid(leixingid);
        if (!jg || jg.zhuangtaima !== 200) {
            nr.innerHTML = `<p style="color:#EF4444">${jg?.xiaoxi || '加载失败'}</p>`;
            return;
        }
        const lie = jg.shuju || [];
        let html = '';
        if (lie.length === 0) {
            html = '<p style="color:#94A3B8">暂无标签</p>';
        } else {
            html = '<div style="display:grid;gap:12px">';
            for (const bq of lie) {
                html += `<div style="padding:12px;background:#F8FAFC;border-radius:8px;display:flex;justify-content:space-between;align-items:center">
                    <span style="font-size:14px;color:#1E293B">${bq.zhi}</span>
                    <div style="display:flex;gap:8px">
                        <button class="aq-btn aq-btn-xiao" onclick="biaoqian_bianjibiaoqian_danxiang('${bq.id}')">编辑</button>
                        <button class="aq-btn aq-btn-xiao" onclick="biaoqian_shanchubiaoqian('${bq.id}')" style="background:#FEE2E2;color:#DC2626">删除</button>
                    </div>
                </div>`;
            }
            html += '</div>';
        }
        nr.innerHTML = html;
    }

    async xinzengbiaoqian() {
        const leixingjg = await this.luoji.leixing_chaxun_quanbu();
        if (!leixingjg || leixingjg.zhuangtaima !== 200 || !leixingjg.shuju || leixingjg.shuju.length === 0) {
            alert('请先创建标签类型');
            return;
        }
        const leixingid = prompt(`选择类型ID（${leixingjg.shuju.map(x => `${x.id}:${x.mingcheng}`).join(', ')}）：`);
        if (!leixingid) return;
        const zhi = prompt('输入标签值：');
        if (!zhi) return;
        const jg = await this.luoji.biaoqian_xinzeng(leixingid, zhi);
        if (jg && jg.zhuangtaima === 200) this.xianshibibaoqian();
    }

    async xinzengbiaoqian_leixing(leixingid) {
        const zhi = prompt('输入标签值：');
        if (!zhi) return;
        const jg = await this.luoji.biaoqian_xinzeng(leixingid, zhi);
        if (jg && jg.zhuangtaima === 200) this.bianjibiaoqian(leixingid);
    }

    async bianjibiaoqian_danxiang(id) {
        const jg = await this.luoji.biaoqian_chaxun_id(id);
        if (!jg || jg.zhuangtaima !== 200) return;
        const zhi = prompt('修改标签值：', jg.shuju.zhi);
        if (!zhi) return;
        const gxjg = await this.luoji.biaoqian_gengxin(id, zhi);
        if (gxjg && gxjg.zhuangtaima === 200) {
            this.xuanzhongleixingid ? this.bianjibiaoqian(this.xuanzhongleixingid) : this.xianshibibaoqian();
        }
    }

    async shanchubiaoqian(id) {
        if (!confirm('确认删除此标签？')) return;
        const jg = await this.luoji.biaoqian_shanchu(id);
        if (jg && jg.zhuangtaima === 200) {
            this.xuanzhongleixingid ? this.bianjibiaoqian(this.xuanzhongleixingid) : this.xianshibibaoqian();
        }
    }

    async fanhui() {
        this.xuanzhongleixingid = null;
        this.xianshileixing();
    }
}
