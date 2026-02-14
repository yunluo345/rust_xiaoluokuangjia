use super::gongjuzhuce::{self, Gongjuyuanshuju};

/// 搜索结果
#[derive(Clone)]
pub struct Sousuojieguo {
    pub gongju: &'static Gongjuyuanshuju,
    /// 匹配得分（0.0~1.0，越高越相关）
    pub defen: f64,
    /// 匹配原因
    pub yuanyin: String,
}

/// 综合搜索：关键词 + 文本相似度
/// 
/// 输入用户意图描述，返回按相关度排序的工具列表
pub fn sousuo(yitu: &str, zuida: usize) -> Vec<Sousuojieguo> {
    let faxiangongju = gongjuzhuce::huoqu_faxiangongju();
    if faxiangongju.is_empty() {
        return vec![];
    }

    let yitu_xiaoxie = yitu.to_lowercase();
    let yitu_ci: Vec<&str> = fencie(&yitu_xiaoxie);

    let mut jieguo: Vec<Sousuojieguo> = faxiangongju
        .into_iter()
        .filter_map(|g| {
            let (defen, yuanyin) = jisuan_defen(g, &yitu_xiaoxie, &yitu_ci);
            if defen > 0.0 {
                Some(Sousuojieguo { gongju: g, defen, yuanyin })
            } else {
                None
            }
        })
        .collect();

    // 按得分降序排列
    jieguo.sort_by(|a, b| b.defen.partial_cmp(&a.defen).unwrap_or(std::cmp::Ordering::Equal));
    jieguo.truncate(zuida);
    jieguo
}

/// 按分组搜索
pub fn anfenzu_sousuo(fenzu: &str) -> Vec<Sousuojieguo> {
    gongjuzhuce::anfenzu_chazhao(fenzu)
        .into_iter()
        .map(|g| Sousuojieguo {
            gongju: g,
            defen: 1.0,
            yuanyin: format!("分组匹配: {}", fenzu),
        })
        .collect()
}

/// 获取所有分组的摘要信息（供AI选择分组用）
pub fn huoqu_fenzuzhaiyao() -> Vec<Fenzuzhaiyao> {
    let suoyoufenzu = gongjuzhuce::huoqu_suoyoufenzu();
    suoyoufenzu
        .into_iter()
        .map(|fenzu| {
            let gongjulie = gongjuzhuce::anfenzu_chazhao(fenzu);
            let gongjuminglie: Vec<&str> = gongjulie.iter().map(|g| g.mingcheng).collect();
            let miaoshulie: Vec<&str> = gongjulie.iter().map(|g| g.miaoshu).collect();
            Fenzuzhaiyao {
                mingcheng: fenzu.to_string(),
                gongjushuliang: gongjulie.len(),
                gongjuminglie,
                miaoshu: miaoshulie.join("；"),
            }
        })
        .collect()
}

/// 分组摘要
pub struct Fenzuzhaiyao {
    pub mingcheng: String,
    pub gongjushuliang: usize,
    pub gongjuminglie: Vec<&'static str>,
    pub miaoshu: String,
}

// ==================== 内部实现 ====================

/// 计算工具与意图的匹配得分
fn jisuan_defen(gongju: &Gongjuyuanshuju, yitu_xiaoxie: &str, yitu_ci: &[&str]) -> (f64, String) {
    let mut zongfen: f64 = 0.0;
    let mut yuanyin_lie: Vec<String> = Vec::new();

    // 1. 场景触发词匹配（权重最高：0.4）
    let changjing_defen = guanjianci_pipei(gongju.changjingci, yitu_xiaoxie, yitu_ci);
    if changjing_defen > 0.0 {
        zongfen += changjing_defen * 0.4;
        yuanyin_lie.push(format!("场景匹配({:.0}%)", changjing_defen * 100.0));
    }

    // 2. 关键词精确匹配（权重：0.3）
    let guanjianci_defen = guanjianci_pipei(gongju.guanjianci, yitu_xiaoxie, yitu_ci);
    if guanjianci_defen > 0.0 {
        zongfen += guanjianci_defen * 0.3;
        yuanyin_lie.push(format!("关键词匹配({:.0}%)", guanjianci_defen * 100.0));
    }

    // 3. 工具名称匹配（权重：0.15）
    let mingcheng_defen = wenben_xiangsi(gongju.mingcheng, yitu_xiaoxie, yitu_ci);
    if mingcheng_defen > 0.0 {
        zongfen += mingcheng_defen * 0.15;
        yuanyin_lie.push(format!("名称匹配({:.0}%)", mingcheng_defen * 100.0));
    }

    // 4. 描述文本相似度（权重：0.1）
    let miaoshu_defen = wenben_xiangsi(gongju.miaoshu, yitu_xiaoxie, yitu_ci);
    if miaoshu_defen > 0.0 {
        zongfen += miaoshu_defen * 0.1;
        yuanyin_lie.push(format!("描述匹配({:.0}%)", miaoshu_defen * 100.0));
    }

    // 5. 分组名称匹配（权重：0.05）
    let fenzu_defen = wenben_xiangsi(gongju.fenzu, yitu_xiaoxie, yitu_ci);
    if fenzu_defen > 0.0 {
        zongfen += fenzu_defen * 0.05;
        yuanyin_lie.push(format!("分组匹配({:.0}%)", fenzu_defen * 100.0));
    }

    let yuanyin = if yuanyin_lie.is_empty() {
        String::new()
    } else {
        yuanyin_lie.join(", ")
    };

    (zongfen, yuanyin)
}

/// 关键词匹配：检查意图中是否包含工具的关键词
fn guanjianci_pipei(guanjianci: &[&str], yitu_xiaoxie: &str, yitu_ci: &[&str]) -> f64 {
    if guanjianci.is_empty() {
        return 0.0;
    }
    let pipei_shu: usize = guanjianci
        .iter()
        .filter(|gc| {
            let gc_xiaoxie = gc.to_lowercase();
            // 完整包含 或 分词匹配
            yitu_xiaoxie.contains(&gc_xiaoxie)
                || yitu_ci.iter().any(|c| gc_xiaoxie.contains(c) || c.contains(&gc_xiaoxie.as_str()))
        })
        .count();
    pipei_shu as f64 / guanjianci.len() as f64
}

/// 文本相似度：基于词重叠的Jaccard相似度
fn wenben_xiangsi(mubiao: &str, _yitu_xiaoxie: &str, yitu_ci: &[&str]) -> f64 {
    let mubiao_xiaoxie = mubiao.to_lowercase();
    let mubiao_ci: Vec<&str> = fencie(&mubiao_xiaoxie);
    
    if mubiao_ci.is_empty() || yitu_ci.is_empty() {
        return 0.0;
    }

    // Jaccard相似度 + 包含关系加分
    let jiaoji: usize = yitu_ci
        .iter()
        .filter(|c| {
            mubiao_ci.iter().any(|mc| mc.contains(*c) || c.contains(mc))
        })
        .count();

    if jiaoji == 0 {
        return 0.0;
    }

    let bingji = yitu_ci.len() + mubiao_ci.len() - jiaoji;
    jiaoji as f64 / bingji as f64
}

/// 简单分词：按空格、标点分割，过滤短词
fn fencie(wenben: &str) -> Vec<&str> {
    wenben
        .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation() || is_zhongwen_biaodian(c))
        .filter(|s| !s.is_empty() && s.len() >= 2)
        .collect()
}

fn is_zhongwen_biaodian(c: char) -> bool {
    matches!(c,
        '\u{FF0C}' | '\u{3002}' | '\u{3001}' | '\u{FF1B}' | '\u{FF1A}' |
        '\u{201C}' | '\u{201D}' | '\u{2018}' | '\u{2019}' |
        '\u{FF08}' | '\u{FF09}' | '\u{3010}' | '\u{3011}'
    )
}
