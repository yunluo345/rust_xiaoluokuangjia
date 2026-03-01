use std::future::Future;
use std::pin::Pin;
use serde_json::Value;
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::yonghuyanzheng;

pub mod gongjutexing;
mod gongju_shijianchaxun;
pub mod gongju_aiqudaoguanli;
mod gongju_ribaotupu;
mod gongju_ribaobiaoqianguanli;
mod gongju_ribaorenwuguanli;
mod gongju_ribaoshendufenxi;
pub mod ribao;
pub mod ceshi_gongjufenzu;
pub mod guanjianci_suoyin;

use llm::chat::Tool;
use guanjianci_suoyin::Guanjiancipeiqi;
use self::ribao::{gongju_ribaojiancha, gongju_ribaorenwuchuli};

/// 工具分组枚举
#[derive(Debug, Clone, PartialEq)]
pub enum Gongjufenzu {
    Guanli,  // 管理组
    Xitong,  // 系统组
}

/// 工具信息结构体，包含关键词和分组
#[derive(Debug, Clone)]
pub struct Gongjuxinxi {
    pub mingcheng: String,
    pub guanjianci: Vec<String>,
    pub fenzu: Gongjufenzu,
}

/// 关键词索引结构体
#[derive(Debug, Clone)]
pub struct Guanjianciyinsuoqing {
    pub guanjianci: String,
    pub gongjuming: String,
}

/// 工具索引管理器
pub struct Gongjusuoyin {
    guanjianci_suoyin: Vec<Guanjianciyinsuoqing>,
}

impl Gongjusuoyin {
    /// 创建新的索引管理器
    pub fn chuangjian() -> Self {
        let mut guanjianci_suoyin = Vec::new();
        
        // 为所有工具建立关键词索引
        for gongju in suoyouzhuce() {
            for ci in &gongju.xinxi.guanjianci {
                guanjianci_suoyin.push(Guanjianciyinsuoqing {
                    guanjianci: ci.to_lowercase(),
                    gongjuming: gongju.xinxi.mingcheng.clone(),
                });
            }
        }
        
        Self { guanjianci_suoyin }
    }
    
    /// 根据关键词快速查找工具名称
    pub fn chaxun_gongjuming(&self, guanjianci: &str) -> Vec<String> {
        let guanjianci_xiaoxie = guanjianci.to_lowercase();
        self.guanjianci_suoyin
            .iter()
            .filter(|item| guanjianci_xiaoxie.contains(&item.guanjianci))
            .map(|item| item.gongjuming.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }
    
    /// 根据工具名称获取所有关键词
    pub fn chaxun_guanjianci(&self, gongjuming: &str) -> Vec<String> {
        self.guanjianci_suoyin
            .iter()
            .filter(|item| item.gongjuming == gongjuming)
            .map(|item| item.guanjianci.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }
    
    /// 模糊匹配关键词
    pub fn mohu_pipei(&self, wenben: &str) -> Vec<String> {
        let wenben_xiaoxie = wenben.to_lowercase();
        self.guanjianci_suoyin
            .iter()
            .filter(|item| wenben_xiaoxie.contains(&item.guanjianci) || item.guanjianci.contains(&wenben_xiaoxie))
            .map(|item| item.gongjuming.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }
}

/// 工具注册项：定义 + 执行函数 + 信息
struct Gongjuzhuce {
    dinyi: Tool,
    zhixing: fn(Value, &str) -> Pin<Box<dyn Future<Output = String> + Send + 'static>>,
    xinxi: Gongjuxinxi,
}

/// 宏：自动生成工具注册代码
#[macro_export]
macro_rules! zhuce_gongju {
    ($mokuai:ident) => {
        Gongjuzhuce {
            dinyi: $mokuai::dinyi(),
            zhixing: |canshu: Value, lingpai: &str| {
                let canshu_str = canshu.to_string();
                let lingpai = lingpai.to_string();
                Box::pin(async move {
                    $mokuai::zhixing(&canshu_str, &lingpai).await
                })
            },
            xinxi: Gongjuxinxi {
                mingcheng: $mokuai::dinyi().function.name,
                guanjianci: $mokuai::huoqu_guanjianci(),
                fenzu: $mokuai::huoqu_fenzu(),
            },
        }
    };
}

/// 所有已注册的工具列表
fn suoyouzhuce() -> Vec<Gongjuzhuce> {
    vec![
        // 使用宏简化注册
        zhuce_gongju!(gongju_shijianchaxun),
        zhuce_gongju!(gongju_aiqudaoguanli),
        zhuce_gongju!(gongju_ribaojiancha),
        zhuce_gongju!(gongju_ribaorenwuchuli),
        zhuce_gongju!(gongju_ribaotupu),
        zhuce_gongju!(gongju_ribaobiaoqianguanli),
        zhuce_gongju!(gongju_ribaorenwuguanli),
        zhuce_gongju!(gongju_ribaoshendufenxi),
    ]
}

/// 包装函数，解决生命周期问题
fn gongju_shijianchaxun_wrapper(canshu: &str, lingpai: &str) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
    let canshu = canshu.to_string();
    let lingpai = lingpai.to_string();
    Box::pin(async move {
        gongju_shijianchaxun::zhixing(&canshu, &lingpai).await
    })
}

/// AI渠道管理工具包装函数
fn gongju_aiqudaoguanli_wrapper(canshu: &str, lingpai: &str) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
    let canshu = canshu.to_string();
    let lingpai = lingpai.to_string();
    Box::pin(async move {
        gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await
    })
}

/// 日报检查工具包装函数
fn gongju_ribaojiancha_wrapper(canshu: &str, lingpai: &str) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
    let canshu = canshu.to_string();
    let lingpai = lingpai.to_string();
    Box::pin(async move {
        gongju_ribaojiancha::zhixing(&canshu, &lingpai).await
    })
}

/// 日报任务标签处理工具包装函数
fn gongju_ribaorenwuchuli_wrapper(canshu: &str, lingpai: &str) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
    let canshu = canshu.to_string();
    let lingpai = lingpai.to_string();
    Box::pin(async move {
        gongju_ribaorenwuchuli::zhixing(&canshu, &lingpai).await
    })
}

/// 获取所有工具定义，供 Xiaoxiguanli 注册
pub fn huoqu_suoyougongju() -> Vec<Tool> {
    suoyouzhuce().into_iter().map(|z| z.dinyi).collect()
}

/// 获取所有工具信息（包含关键词和分组）
pub fn huoqu_suoyougongjuxinxi() -> Vec<Gongjuxinxi> {
    suoyouzhuce().into_iter().map(|z| z.xinxi).collect()
}

/// 根据分组获取工具定义
pub fn huoqu_fenzu_gongju(fenzu: Gongjufenzu) -> Vec<Tool> {
    suoyouzhuce()
        .into_iter()
        .filter(|z| z.xinxi.fenzu == fenzu)
        .map(|z| z.dinyi)
        .collect()
}

/// 根据关键词匹配获取相关工具
pub fn huoqu_guanjianci_gongju(guanjianci: &str) -> Vec<Tool> {
    let guanjianci_xiaoxie = guanjianci.to_lowercase();
    suoyouzhuce()
        .into_iter()
        .filter(|z| {
            z.xinxi.guanjianci.iter().any(|ci| {
                guanjianci_xiaoxie.contains(&ci.to_lowercase())
            })
        })
        .map(|z| z.dinyi)
        .collect()
}

/// 根据工具名称获取工具信息
pub fn huoqu_gongju_xinxi(gongjuming: &str) -> Option<Gongjuxinxi> {
    suoyouzhuce()
        .into_iter()
        .find(|z| z.dinyi.function.name == gongjuming)
        .map(|z| z.xinxi)
}

/// 创建全局索引管理器
pub fn chuangjian_suoyin() -> Gongjusuoyin {
    Gongjusuoyin::chuangjian()
}

/// 创建 Trie 树索引器
pub fn chuangjian_trie_suoyin() -> Guanjiancipeiqi {
    let mut peiqi = Guanjiancipeiqi::xinjian();
    
    // 为所有工具建立 Trie 树索引
    for gongju in suoyouzhuce() {
        for guanjianci in &gongju.xinxi.guanjianci {
            peiqi.charu(guanjianci, &gongju.xinxi.mingcheng);
        }
    }
    
    peiqi
}

/// 智能提取：从用户输入中自动提取关键词并匹配工具
pub fn zhineng_tiqu_gongju(yonghushuru: &str) -> Vec<Tool> {
    let peiqi = chuangjian_trie_suoyin();
    let pipei_jieguo = peiqi.zhineng_pipei(yonghushuru);
    
    let gongjuming_lie: Vec<String> = pipei_jieguo.into_iter().map(|(ming, _)| ming).collect();
    
    suoyouzhuce()
        .into_iter()
        .filter(|z| gongjuming_lie.contains(&z.xinxi.mingcheng))
        .map(|z| z.dinyi)
        .collect()
}

/// 智能提取并返回工具名和得分
pub fn zhineng_tiqu_gongjuming(yonghushuru: &str) -> Vec<(String, usize)> {
    let peiqi = chuangjian_trie_suoyin();
    peiqi.zhineng_pipei(yonghushuru)
}

/// 根据关键词索引快速获取工具
pub fn suoyin_huoqu_gongju(guanjianci: &str) -> Vec<Tool> {
    let suoyin = chuangjian_suoyin();
    let gongjuming_lie = suoyin.chaxun_gongjuming(guanjianci);
    
    suoyouzhuce()
        .into_iter()
        .filter(|z| gongjuming_lie.contains(&z.xinxi.mingcheng))
        .map(|z| z.dinyi)
        .collect()
}

/// 模糊匹配获取工具
pub fn mohu_huoqu_gongju(wenben: &str) -> Vec<Tool> {
    let suoyin = chuangjian_suoyin();
    let gongjuming_lie = suoyin.mohu_pipei(wenben);
    
    suoyouzhuce()
        .into_iter()
        .filter(|z| gongjuming_lie.contains(&z.xinxi.mingcheng))
        .map(|z| z.dinyi)
        .collect()
}

/// 获取所有工具的关键词映射表
pub fn huoqu_guanjianci_yingshe() -> std::collections::HashMap<String, Vec<String>> {
    let mut yingshe = std::collections::HashMap::new();
    
    for gongju in suoyouzhuce() {
        yingshe.insert(gongju.xinxi.mingcheng, gongju.xinxi.guanjianci);
    }
    
    yingshe
}

/// 获取分组的工具映射表
pub fn huoqu_fenzu_yingshe() -> std::collections::HashMap<String, Vec<String>> {
    let mut yingshe = std::collections::HashMap::new();
    
    for gongju in suoyouzhuce() {
        let fenzu_ming = match gongju.xinxi.fenzu {
            Gongjufenzu::Guanli => "guanli",
            Gongjufenzu::Xitong => "xitong",
        };
        
        yingshe.entry(fenzu_ming.to_string())
            .or_insert_with(Vec::new)
            .push(gongju.xinxi.mingcheng);
    }
    
    yingshe
}

/// 每个工具对应的接口权限路径（满足任一即可）
fn gongju_quanxian_lujing(gongjuming: &str) -> &'static [&'static str] {
    match gongjuming {
        "shijian_chaxun" => &[],
        "aiqudao_guanli" => &["/jiekou/xitong/aiqudao"],
        "ribao_jiancha" => &["/jiekou/ribao/yonghu"],
        "ribao_renwubiaoqian_chuli" => &["/jiekou/ribao/guanli"],
        "ribao_tupu_guanli" => &["/jiekou/ribao/guanli"],
        "ribao_biaoqian_guanli" => &["/jiekou/ribao/guanli"],
        "ribao_renwu_guanli" => &["/jiekou/ribao/guanli"],
        "ribao_shendu_fenxi" => &["/jiekou/ribao/guanli"],
        _ => &[],
    }
}

/// 判断工具是否允许当前令牌调用
pub async fn gongju_yunxu_diaoyong(gongjuming: &str, lingpai: &str) -> bool {
    let lujinglie = gongju_quanxian_lujing(gongjuming);
    if lujinglie.is_empty() {
        return true;
    }
    let zaiti = match yonghuyanzheng::yanzhenglingpai(lingpai).await {
        Ok(z) => z,
        Err(_) => return false,
    };
    for lujing in lujinglie {
        if yonghuyanzheng::jianchajiekouquanxian(&zaiti.yonghuzuid, lujing).await.is_ok() {
            return true;
        }
    }
    false
}

/// 按令牌权限过滤可见工具（无权限工具不暴露给模型）
pub async fn guolv_gongjulie_anlingpai(gongjulie: Vec<Tool>, lingpai: &str) -> Vec<Tool> {
    let zaiti = match yonghuyanzheng::yanzhenglingpai(lingpai).await {
        Ok(z) => z,
        Err(_) => return Vec::new(),
    };
    let mut jieguo = Vec::with_capacity(gongjulie.len());
    for gongju in gongjulie {
        let lujinglie = gongju_quanxian_lujing(&gongju.function.name);
        if lujinglie.is_empty() {
            jieguo.push(gongju);
            continue;
        }
        let mut keyong = false;
        for lujing in lujinglie {
            if yonghuyanzheng::jianchajiekouquanxian(&zaiti.yonghuzuid, lujing).await.is_ok() {
                keyong = true;
                break;
            }
        }
        if keyong {
            jieguo.push(gongju);
        } else {
            println!("[工具过滤] 用户组={} 无权限使用工具={}", zaiti.yonghuzuid, gongju.function.name);
        }
    }
    jieguo
}

/// 按工具名称分发执行，返回结果字符串
pub async fn zhixing(gongjuming: &str, canshu: &str, lingpai: &str) -> String {
    use serde_json::from_str;
    if !gongju_yunxu_diaoyong(gongjuming, lingpai).await {
        return serde_json::json!({"cuowu": "权限不足"}).to_string();
    }
    
    let canshu_value: Value = from_str(canshu).unwrap_or(Value::Null);
    
    for zhuce in suoyouzhuce() {
        if zhuce.dinyi.function.name == gongjuming {
            return (zhuce.zhixing)(canshu_value, lingpai).await;
        }
    }
    format!("未知工具: {}", gongjuming)
}
