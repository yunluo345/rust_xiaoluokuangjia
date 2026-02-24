use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::*;

#[allow(non_upper_case_globals)]
const xiaoxiezimu: &str = "abcdefghijklmnopqrstuvwxyz";
#[allow(non_upper_case_globals)]
const daxiezimu: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
#[allow(non_upper_case_globals)]
const shuzi: &str = "0123456789";
#[allow(non_upper_case_globals)]
const teshufuhao: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";

/// 字符集类型
#[derive(Debug, Clone, Copy)]
pub enum Zifuji {
    Xiaoxie,    // 小写字母
    Daxie,      // 大写字母
    Shuzi,      // 数字
    Teshu,      // 特殊符号
}

impl Zifuji {
    fn huoquzifuchuan(&self) -> &'static str {
        match self {
            Zifuji::Xiaoxie => xiaoxiezimu,
            Zifuji::Daxie => daxiezimu,
            Zifuji::Shuzi => shuzi,
            Zifuji::Teshu => teshufuhao,
        }
    }
}

/// 随机字符串生成配置
#[derive(Debug, Clone)]
pub struct Suijipeizhi {
    pub zifuji: Vec<Zifuji>,           // 使用的字符集
    pub changdu: usize,                 // 总长度
    pub hunluan: bool,                  // 是否完全随机混乱
    pub fenzuchangdu: Option<Vec<usize>>, // 每组长度（按顺序）
}

impl Default for Suijipeizhi {
    fn default() -> Self {
        Self {
            zifuji: vec![Zifuji::Xiaoxie, Zifuji::Daxie, Zifuji::Shuzi],
            changdu: 16,
            hunluan: true,
            fenzuchangdu: None,
        }
    }
}

impl Suijipeizhi {
    /// 创建新配置
    pub fn xin() -> Self {
        Self::default()
    }
    
    /// 设置字符集
    pub fn shezhi_zifuji(mut self, zifuji: Vec<Zifuji>) -> Self {
        self.zifuji = zifuji;
        self
    }
    
    /// 设置长度
    pub fn shezhi_changdu(mut self, changdu: usize) -> Self {
        self.changdu = changdu;
        self
    }
    
    /// 设置是否混乱
    pub fn shezhi_hunluan(mut self, hunluan: bool) -> Self {
        self.hunluan = hunluan;
        self
    }
    
    /// 设置分组长度
    pub fn shezhi_fenzuchangdu(mut self, fenzuchangdu: Vec<usize>) -> Self {
        self.fenzuchangdu = Some(fenzuchangdu);
        self
    }
}

fn congzifujixuanze(zifuji: &str, shuliang: usize) -> String {
    let mut rng = rand::rng();
    let zifu: Vec<char> = zifuji.chars().collect();
    (0..shuliang)
        .map(|_| zifu[rng.random_range(0..zifu.len())])
        .collect()
}

fn hunluanzifuchuan(s: &str) -> String {
    let mut rng = rand::rng();
    let mut zifu: Vec<char> = s.chars().collect();
    
    for i in (1..zifu.len()).rev() {
        let j = rng.random_range(0..=i);
        zifu.swap(i, j);
    }
    
    zifu.into_iter().collect()
}

/// 生成随机字符串
#[allow(dead_code)]
pub fn shengchengsuijizifuchuan(peizhi: &Suijipeizhi) -> String {
    if peizhi.zifuji.is_empty() {
        return String::new();
    }
    
    let mut jieguo = String::with_capacity(peizhi.changdu);
    
    if peizhi.hunluan {
        // 完全随机混乱模式
        let hecheng: String = peizhi.zifuji.iter()
            .map(|zf| zf.huoquzifuchuan())
            .collect::<Vec<_>>()
            .join("");
        jieguo = congzifujixuanze(&hecheng, peizhi.changdu);
    } else if let Some(fenzuchangdu) = &peizhi.fenzuchangdu {
        // 分组模式（按顺序）
        for (i, &changdu) in fenzuchangdu.iter().enumerate() {
            if i < peizhi.zifuji.len() {
                let zifuchuan = peizhi.zifuji[i].huoquzifuchuan();
                jieguo.push_str(&congzifujixuanze(zifuchuan, changdu));
            }
        }
    } else {
        // 均匀分配模式
        let meizu = peizhi.changdu / peizhi.zifuji.len();
        let shengyu = peizhi.changdu % peizhi.zifuji.len();
        
        for (i, zf) in peizhi.zifuji.iter().enumerate() {
            let changdu = if i < shengyu { meizu + 1 } else { meizu };
            jieguo.push_str(&congzifujixuanze(zf.huoquzifuchuan(), changdu));
        }
        
        // 打乱顺序
        jieguo = hunluanzifuchuan(&jieguo);
    }
    
    jieguo
}

/// 批量生成随机字符串
#[allow(dead_code)]
pub fn piliang_shengcheng(peizhi: &Suijipeizhi, shuliang: usize) -> Vec<String> {
    (0..shuliang)
        .map(|_| shengchengsuijizifuchuan(peizhi))
        .collect()
}

/// 获取当前 13 位毫秒级时间戳
#[allow(dead_code)]
pub fn huoqushijianchuo() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|shichang| shichang.as_millis() as u64)
        .unwrap_or(0)
}

/// 计算分页偏移量，返回 (每页条数, 偏移量) 的字符串元组
pub fn jisuanfenye(yeshu: i64, meiyetiaoshu: i64) -> (String, String) {
    let tiaoshu = meiyetiaoshu.max(1);
    let pianyi = (yeshu.max(1) - 1) * tiaoshu;
    (tiaoshu.to_string(), pianyi.to_string())
}

/// 构建 IN 子句占位符：$1::BIGINT,$2::BIGINT,...
pub fn goujian_in_zhanhaowei(shuliang: usize) -> String {
    (1..=shuliang)
        .map(|i| format!("${}::BIGINT", i))
        .collect::<Vec<_>>()
        .join(",")
}

/// 批量删除：按指定字段的 IN 条件删除
pub async fn piliang_shanchu_ziduan(biaoming: &str, ziduan: &str, zhilie: &[&str]) -> Option<u64> {
    use crate::shujuku::psqlshujuku::psqlcaozuo;
    match zhilie.is_empty() {
        true => Some(0),
        false => psqlcaozuo::zhixing(
            &format!("DELETE FROM {} WHERE {} IN ({})", biaoming, ziduan, goujian_in_zhanhaowei(zhilie.len())),
            zhilie,
        ).await,
    }
}

/// 批量删除：按 id 字段
pub async fn piliang_shanchu(biaoming: &str, idlie: &[&str]) -> Option<u64> {
    piliang_shanchu_ziduan(biaoming, "id", idlie).await
}
