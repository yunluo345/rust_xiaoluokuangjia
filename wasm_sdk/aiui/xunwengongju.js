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
        waiceng.style.cssText = 'display:flex;justify-content:flex-start;margin:12px 0 14px';

        const neirongqu = document.createElement('div');
        neirongqu.style.cssText = [
            'max-width:88%',
            'width:min(760px, 100%)',
            'background:linear-gradient(180deg,#FFFBEB 0%,#FEF3C7 100%)',
            'border:1px solid #FCD34D',
            'border-left:4px solid #F59E0B',
            'border-radius:14px',
            'padding:14px 14px 12px',
            'box-shadow:0 2px 10px rgba(245,158,11,0.10)',
            'box-sizing:border-box',
        ].join(';');

        // 头部：标题 + 说明
        const tou = document.createElement('div');
        tou.style.cssText = 'display:flex;justify-content:space-between;align-items:center;gap:10px;margin-bottom:10px;flex-wrap:wrap';
        const biaoti = document.createElement('div');
        biaoti.style.cssText = 'display:inline-flex;align-items:center;gap:6px;font-size:12px;font-weight:700;color:#B45309;letter-spacing:.2px';
        biaoti.textContent = '需要你的确认';
        const shuoming = document.createElement('div');
        shuoming.style.cssText = 'font-size:12px;color:#92400E;opacity:.82;line-height:1.5';
        shuoming.textContent = '你可以点选，也可以直接输入';
        tou.appendChild(biaoti);
        tou.appendChild(shuoming);
        neirongqu.appendChild(tou);

        const fenge = document.createElement('div');
        fenge.style.cssText = 'height:1px;background:rgba(217,119,6,.18);margin:0 0 10px';
        neirongqu.appendChild(fenge);

        // 问题内容
        const wentiqu = document.createElement('div');
        wentiqu.style.cssText = 'font-size:15px;font-weight:600;line-height:1.65;color:#1F2937;margin:0 0 12px;white-space:pre-wrap;word-break:break-word';
        wentiqu.textContent = String(wenti || '请确认下一步操作');
        neirongqu.appendChild(wentiqu);

        // 选项按钮区域
        if (Array.isArray(xuanxiang) && xuanxiang.length > 0) {
            const xuanxiangqu = document.createElement('div');
            xuanxiangqu.style.cssText = 'display:grid;grid-template-columns:repeat(auto-fit,minmax(128px,1fr));gap:8px;margin-bottom:12px';

            for (const xiang of xuanxiang) {
                const btn = document.createElement('button');
                btn.className = 'aq-btn';
                btn.type = 'button';
                btn.style.cssText = [
                    'padding:10px 12px',
                    'font-size:13px',
                    'font-weight:600',
                    'line-height:1.2',
                    'border-radius:10px',
                    'cursor:pointer',
                    'background:#FFF7ED',
                    'color:#9A3412',
                    'border:1px solid #FDBA74',
                    'min-height:44px',
                    'text-align:center',
                    'transition:all .18s ease',
                    'outline:none',
                    'box-shadow:0 1px 0 rgba(217,119,6,.08)',
                ].join(';');
                btn.textContent = xiang;
                btn.addEventListener('click', () => this._huifu(xiang));
                // hover/focus 效果
                btn.addEventListener('mouseenter', () => {
                    btn.style.background = '#FFEDD5';
                    btn.style.borderColor = '#FB923C';
                    btn.style.transform = 'translateY(-1px)';
                });
                btn.addEventListener('mouseleave', () => {
                    btn.style.background = '#FFF7ED';
                    btn.style.borderColor = '#FDBA74';
                    btn.style.transform = 'translateY(0)';
                });
                btn.addEventListener('focus', () => {
                    btn.style.boxShadow = '0 0 0 3px rgba(245,158,11,.22)';
                    btn.style.borderColor = '#F59E0B';
                });
                btn.addEventListener('blur', () => {
                    btn.style.boxShadow = '0 1px 0 rgba(217,119,6,.08)';
                    btn.style.borderColor = '#FDBA74';
                });
                xuanxiangqu.appendChild(btn);
            }
            neirongqu.appendChild(xuanxiangqu);
        }

        // 自由输入区域
        const zibiaoti = document.createElement('div');
        zibiaoti.style.cssText = 'font-size:12px;color:#92400E;margin-bottom:6px;line-height:1.5';
        zibiaoti.textContent = '自定义回答';
        neirongqu.appendChild(zibiaoti);
        const shuruqu = document.createElement('div');
        shuruqu.style.cssText = 'display:flex;gap:8px;align-items:stretch';

        const shurukuang = document.createElement('input');
        shurukuang.type = 'text';
        shurukuang.placeholder = '请输入你的回答…';
        shurukuang.autocomplete = 'off';
        shurukuang.style.cssText = [
            'flex:1',
            'min-width:0',
            'border:1px solid #FCD34D',
            'border-radius:10px',
            'padding:10px 12px',
            'font-size:14px',
            'line-height:1.4',
            'outline:none',
            'background:#FFFFFF',
            'color:#1F2937',
            'min-height:44px',
            'box-sizing:border-box',
        ].join(';');
        shurukuang.addEventListener('focus', () => {
            shurukuang.style.borderColor = '#F59E0B';
            shurukuang.style.boxShadow = '0 0 0 3px rgba(245,158,11,.22)';
        });
        shurukuang.addEventListener('blur', () => {
            shurukuang.style.borderColor = '#FCD34D';
            shurukuang.style.boxShadow = 'none';
        });

        const fasongbtn = document.createElement('button');
        fasongbtn.className = 'aq-btn';
        fasongbtn.type = 'button';
        fasongbtn.style.cssText = [
            'padding:0 16px',
            'font-size:13px',
            'font-weight:700',
            'border-radius:10px',
            'cursor:pointer',
            'background:#F59E0B',
            'color:#fff',
            'border:none',
            'min-height:44px',
            'min-width:76px',
            'transition:all .16s ease',
            'outline:none',
            'box-shadow:0 2px 6px rgba(217,119,6,.25)',
        ].join(';');
        fasongbtn.textContent = '发送';
        fasongbtn.disabled = true;

        const gengxinzhuangtai = () => {
            const keyong = !!shurukuang.value.trim();
            fasongbtn.disabled = !keyong;
            fasongbtn.style.opacity = keyong ? '1' : '.55';
            fasongbtn.style.cursor = keyong ? 'pointer' : 'not-allowed';
        };
        shurukuang.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && shurukuang.value.trim()) {
                this._huifu(shurukuang.value.trim());
            }
        });
        shurukuang.addEventListener('input', gengxinzhuangtai);
        fasongbtn.addEventListener('click', () => {
            if (shurukuang.value.trim()) {
                this._huifu(shurukuang.value.trim());
            }
        });
        fasongbtn.addEventListener('mouseenter', () => {
            if (!fasongbtn.disabled) fasongbtn.style.transform = 'translateY(-1px)';
        });
        fasongbtn.addEventListener('mouseleave', () => {
            fasongbtn.style.transform = 'translateY(0)';
        });
        fasongbtn.addEventListener('focus', () => {
            fasongbtn.style.boxShadow = '0 0 0 3px rgba(245,158,11,.28)';
        });
        fasongbtn.addEventListener('blur', () => {
            fasongbtn.style.boxShadow = '0 2px 6px rgba(217,119,6,.25)';
        });

        shuruqu.appendChild(shurukuang);
        shuruqu.appendChild(fasongbtn);
        neirongqu.appendChild(shuruqu);
        const tip = document.createElement('div');
        tip.style.cssText = 'margin-top:8px;font-size:12px;color:#A16207;opacity:.9;line-height:1.5';
        tip.textContent = '提示：按 Enter 快速发送';
        neirongqu.appendChild(tip);

        waiceng.appendChild(neirongqu);
        this.rongqi.appendChild(waiceng);
        this.rongqi.scrollTop = this.rongqi.scrollHeight;
        shurukuang.focus();
        gengxinzhuangtai();
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
