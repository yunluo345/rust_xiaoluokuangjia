use std::future::Future;
use std::pin::Pin;

mod gongju_shijianchaxun;
pub mod gongju_aiqudaoguanli;
mod ribao;
pub mod ceshi_gongjufenzu;
pub mod guanjianci_suoyin;

use llm::chat::Tool;
use guanjianci_suoyin::Guanjiancipeiqi;
use ribao::{gongju_ribaojiancha, gongju_ribaotijiao};

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
    zhixing: fn(&str, &str) -> Pin<Box<dyn Future<Output = String> + Send + 'static>>,
    xinxi: Gongjuxinxi,
}

/// 所有已注册的工具列表
fn suoyouzhuce() -> Vec<Gongjuzhuce> {
    vec![
        Gongjuzhuce {
            dinyi: gongju_shijianchaxun::dinyi(),
            zhixing: gongju_shijianchaxun_wrapper,
            xinxi: Gongjuxinxi {
                mingcheng: "shijian_chaxun".to_string(),
                guanjianci: gongju_shijianchaxun::huoqu_guanjianci(),
                fenzu: match gongju_shijianchaxun::huoqu_fenzu() {
                    gongju_shijianchaxun::Gongjufenzu::Guanli => Gongjufenzu::Guanli,
                    gongju_shijianchaxun::Gongjufenzu::Xitong => Gongjufenzu::Xitong,
                },
            },
        },
        Gongjuzhuce {
            dinyi: gongju_aiqudaoguanli::dinyi(),
            zhixing: gongju_aiqudaoguanli_wrapper,
            xinxi: Gongjuxinxi {
                mingcheng: "aiqudao_guanli".to_string(),
                guanjianci: gongju_aiqudaoguanli::huoqu_guanjianci(),
                fenzu: match gongju_aiqudaoguanli::huoqu_fenzu() {
                    gongju_aiqudaoguanli::Gongjufenzu::Guanli => Gongjufenzu::Guanli,
                    gongju_aiqudaoguanli::Gongjufenzu::Xitong => Gongjufenzu::Xitong,
                },
            },
        },
        Gongjuzhuce {
            dinyi: gongju_ribaojiancha::dinyi(),
            zhixing: gongju_ribaojiancha_wrapper,
            xinxi: Gongjuxinxi {
                mingcheng: "ribao_jiancha".to_string(),
                guanjianci: gongju_ribaojiancha::huoqu_guanjianci(),
                fenzu: match gongju_ribaojiancha::huoqu_fenzu() {
                    gongju_ribaojiancha::Gongjufenzu::Guanli => Gongjufenzu::Guanli,
                    gongju_ribaojiancha::Gongjufenzu::Xitong => Gongjufenzu::Xitong,
                },
            },
        },
        Gongjuzhuce {
            dinyi: gongju_ribaotijiao::dinyi(),
            zhixing: gongju_ribaotijiao_wrapper,
            xinxi: Gongjuxinxi {
                mingcheng: "ribao_tijiao".to_string(),
                guanjianci: gongju_ribaotijiao::huoqu_guanjianci(),
                fenzu: match gongju_ribaotijiao::huoqu_fenzu() {
                    gongju_ribaotijiao::Gongjufenzu::Guanli => Gongjufenzu::Guanli,
                    gongju_ribaotijiao::Gongjufenzu::Xitong => Gongjufenzu::Xitong,
                },
            },
        },
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

/// 日报提交工具包装函数
fn gongju_ribaotijiao_wrapper(canshu: &str, lingpai: &str) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
    let canshu = canshu.to_string();
    let lingpai = lingpai.to_string();
    Box::pin(async move {
        gongju_ribaotijiao::zhixing(&canshu, &lingpai).await
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

/// 按工具名称分发执行，返回结果字符串
pub async fn zhixing(gongjuming: &str, canshu: &str, lingpai: &str) -> String {
    for zhuce in suoyouzhuce() {
        if zhuce.dinyi.function.name == gongjuming {
            return (zhuce.zhixing)(canshu, lingpai).await;
        }
    }
    format!("未知工具: {}", gongjuming)
}
