// 询问工具 UI 组件
// 当后端 xunwen 工具被调用时，渲染问题和选项按钮，用户可选择或自由输入

export class Xunwengongju {
    /**
     * @param {object} opts
     * @param {HTMLElement} opts.rongqi - 对话区域容器元素
     * @param {function} opts.fasonghuifu - 发送回复的回调，接收用户的回答字符串
     * @param {function} opts.zhuanyihtml - HTML 转义函数
     */
    constructor(opts) {
        this.rongqi = opts.rongqi;
        this.fasonghuifu = opts.fasonghuifu;
        this.zhuanyihtml = opts.zhuanyihtml || (s => String(s ?? ''));
        this.dangqianid = null; // 当前询问 UI 的 DOM id
    }

    /**
     * 渲染询问 UI
     * @param {string} wenti - 问题内容
     * @param {string[]} xuanxiang - 选项列表（可为空）
     */
    xuanran(wenti, xuanxiang = []) {
        this.yichu(); // 先移除旧的

        const id = 'xunwen_ui_' + Date.now();
        this.dangqianid = id;

        const waiceng = document.createElement('div');
        waiceng.id = id;
        waiceng.style.cssText = 'display:flex;justify-content:flex-start;margin-bottom:12px';

        const neirongqu = document.createElement('div');
        neirongqu.style.cssText = 'max-width:85%;background:#FEF3C7;border-radius:8px;padding:12px;border-left:3px solid #F59E0B';

        // 标题
        const biaoti = document.createElement('div');
        biaoti.style.cssText = 'font-size:12px;font-weight:600;color:#D97706;margin-bottom:8px';
        biaoti.textContent = 'AI 询问';
        neirongqu.appendChild(biaoti);

        // 问题内容
        const wentiqu = document.createElement('div');
        wentiqu.style.cssText = 'font-size:14px;color:#1E293B;margin-bottom:12px;white-space:pre-wrap;word-break:break-word';
        wentiqu.textContent = wenti;
        neirongqu.appendChild(wentiqu);

        // 选项按钮区域
        if (xuanxiang.length > 0) {
            const xuanxiangqu = document.createElement('div');
            xuanxiangqu.style.cssText = 'display:flex;flex-wrap:wrap;gap:8px;margin-bottom:10px';

            for (const xiang of xuanxiang) {
                const btn = document.createElement('button');
                btn.className = 'aq-btn';
                btn.style.cssText = 'padding:6px 16px;font-size:13px;border-radius:6px;cursor:pointer;background:#FDE68A;color:#92400E;border:1px solid #F59E0B;min-height:32px';
                btn.textContent = xiang;
                btn.addEventListener('click', () => this._huifu(xiang));
                // hover 效果
                btn.addEventListener('mouseenter', () => { btn.style.background = '#FCD34D'; });
                btn.addEventListener('mouseleave', () => { btn.style.background = '#FDE68A'; });
                xuanxiangqu.appendChild(btn);
            }
            neirongqu.appendChild(xuanxiangqu);
        }

        // 自由输入区域
        const shuruqu = document.createElement('div');
        shuruqu.style.cssText = 'display:flex;gap:6px;align-items:center';

        const shurukuang = document.createElement('input');
        shurukuang.type = 'text';
        shurukuang.placeholder = '或输入自定义回答...';
        shurukuang.style.cssText = 'flex:1;border:1px solid #D1D5DB;border-radius:6px;padding:6px 10px;font-size:13px;outline:none;background:#FFFBEB';
        shurukuang.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && shurukuang.value.trim()) {
                this._huifu(shurukuang.value.trim());
            }
        });

        const fasongbtn = document.createElement('button');
        fasongbtn.className = 'aq-btn';
        fasongbtn.style.cssText = 'padding:6px 12px;font-size:13px;border-radius:6px;cursor:pointer;background:#F59E0B;color:white;border:none;min-height:32px';
        fasongbtn.textContent = '回复';
        fasongbtn.addEventListener('click', () => {
            if (shurukuang.value.trim()) {
                this._huifu(shurukuang.value.trim());
            }
        });

        shuruqu.appendChild(shurukuang);
        shuruqu.appendChild(fasongbtn);
        neirongqu.appendChild(shuruqu);

        waiceng.appendChild(neirongqu);
        this.rongqi.appendChild(waiceng);
        this.rongqi.scrollTop = this.rongqi.scrollHeight;
    }

    /**
     * 处理用户回复：移除询问 UI，触发回调
     */
    _huifu(neirong) {
        this.yichu();
        if (this.fasonghuifu) {
            this.fasonghuifu(neirong);
        }
    }

    /**
     * 移除当前询问 UI
     */
    yichu() {
        if (this.dangqianid) {
            const el = document.getElementById(this.dangqianid);
            if (el) el.remove();
            this.dangqianid = null;
        }
    }
}
