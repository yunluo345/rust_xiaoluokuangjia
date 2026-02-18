use std::collections::{HashMap, HashSet};

/// Trie 树节点
#[derive(Debug, Clone)]
struct Triejiedian {
    zinodemap: HashMap<char, Box<Triejiedian>>,
    shimowei: bool,
    gongjuming: Vec<String>,
}

impl Triejiedian {
    fn xinjian() -> Self {
        Self {
            zinodemap: HashMap::new(),
            shimowei: false,
            gongjuming: Vec::new(),
        }
    }
}

/// 关键词索引器（基于 Trie 树）
pub struct Guanjiancipeiqi {
    genjiedian: Triejiedian,
    guanjianci_changdu: HashMap<String, usize>,
}

impl Guanjiancipeiqi {
    /// 创建新的索引器
    pub fn xinjian() -> Self {
        Self {
            genjiedian: Triejiedian::xinjian(),
            guanjianci_changdu: HashMap::new(),
        }
    }

    /// 插入关键词和对应的工具名
    pub fn charu(&mut self, guanjianci: &str, gongjuming: &str) {
        let guanjianci_xiaoxie = guanjianci.to_lowercase();
        let changdu = guanjianci_xiaoxie.chars().count();
        
        self.guanjianci_changdu.insert(guanjianci_xiaoxie.clone(), changdu);
        
        let mut dangqian = &mut self.genjiedian;
        for zifu in guanjianci_xiaoxie.chars() {
            dangqian = dangqian.zinodemap
                .entry(zifu)
                .or_insert_with(|| Box::new(Triejiedian::xinjian()));
        }
        
        dangqian.shimowei = true;
        if !dangqian.gongjuming.contains(&gongjuming.to_string()) {
            dangqian.gongjuming.push(gongjuming.to_string());
        }
    }

    /// 从文本中提取所有匹配的关键词和对应的工具
    pub fn tiqu_guanjianci(&self, wenben: &str) -> Vec<(String, Vec<String>)> {
        let wenben_xiaoxie = wenben.to_lowercase();
        let zifu_lie: Vec<char> = wenben_xiaoxie.chars().collect();
        let mut jieguo = Vec::new();
        let mut yipipei = HashSet::new();

        // 从每个位置开始尝试匹配
        for kaishi in 0..zifu_lie.len() {
            let mut dangqian = &self.genjiedian;
            
            for pianyi in kaishi..zifu_lie.len() {
                let zifu = zifu_lie[pianyi];
                
                if let Some(zijiedian) = dangqian.zinodemap.get(&zifu) {
                    dangqian = zijiedian;
                    
                    // 找到完整关键词
                    if dangqian.shimowei && !dangqian.gongjuming.is_empty() {
                        let pipei_wenben: String = zifu_lie[kaishi..=pianyi].iter().collect();
                        let weizhi_biaoshi = format!("{}:{}", kaishi, pianyi);
                        
                        // 避免重复匹配同一位置
                        if !yipipei.contains(&weizhi_biaoshi) {
                            yipipei.insert(weizhi_biaoshi);
                            jieguo.push((pipei_wenben, dangqian.gongjuming.clone()));
                        }
                    }
                } else {
                    break;
                }
            }
        }

        jieguo
    }

    /// 从文本中提取匹配的工具名（去重）
    pub fn tiqu_gongjuming(&self, wenben: &str) -> Vec<String> {
        let pipei_jieguo = self.tiqu_guanjianci(wenben);
        let mut gongjuming_jihe = HashSet::new();
        
        for (_, gongjuming_lie) in pipei_jieguo {
            for gongjuming in gongjuming_lie {
                gongjuming_jihe.insert(gongjuming);
            }
        }
        
        gongjuming_jihe.into_iter().collect()
    }

    /// 智能匹配：返回工具名和匹配得分（匹配的关键词数量）
    pub fn zhineng_pipei(&self, wenben: &str) -> Vec<(String, usize)> {
        let pipei_jieguo = self.tiqu_guanjianci(wenben);
        let mut gongjuming_defen: HashMap<String, usize> = HashMap::new();
        
        for (_, gongjuming_lie) in pipei_jieguo {
            for gongjuming in gongjuming_lie {
                *gongjuming_defen.entry(gongjuming).or_insert(0) += 1;
            }
        }
        
        let mut jieguo: Vec<(String, usize)> = gongjuming_defen.into_iter().collect();
        // 按得分降序排序
        jieguo.sort_by(|a, b| b.1.cmp(&a.1));
        jieguo
    }

    /// 前缀匹配：查找以指定前缀开头的所有工具
    pub fn qianzui_pipei(&self, qianzui: &str) -> Vec<String> {
        let qianzui_xiaoxie = qianzui.to_lowercase();
        let mut dangqian = &self.genjiedian;
        
        // 先找到前缀对应的节点
        for zifu in qianzui_xiaoxie.chars() {
            if let Some(zijiedian) = dangqian.zinodemap.get(&zifu) {
                dangqian = zijiedian;
            } else {
                return Vec::new();
            }
        }
        
        // 收集该节点下所有的工具名
        let mut jieguo = HashSet::new();
        self.shouji_suoyou_gongjuming(dangqian, &mut jieguo);
        jieguo.into_iter().collect()
    }

    /// 递归收集节点下所有工具名
    fn shouji_suoyou_gongjuming(&self, jiedian: &Triejiedian, jieguo: &mut HashSet<String>) {
        if jiedian.shimowei {
            for gongjuming in &jiedian.gongjuming {
                jieguo.insert(gongjuming.clone());
            }
        }
        
        for zijiedian in jiedian.zinodemap.values() {
            self.shouji_suoyou_gongjuming(zijiedian, jieguo);
        }
    }
}

#[cfg(test)]
mod ceshi {
    use super::*;

    #[test]
    fn ceshi_jiben_pipei() {
        let mut peiqi = Guanjiancipeiqi::xinjian();
        peiqi.charu("时间", "shijian_chaxun");
        peiqi.charu("渠道", "aiqudao_guanli");
        peiqi.charu("AI", "aiqudao_guanli");

        let jieguo = peiqi.tiqu_gongjuming("查询当前时间");
        assert!(jieguo.contains(&"shijian_chaxun".to_string()));

        let jieguo = peiqi.tiqu_gongjuming("管理AI渠道");
        assert!(jieguo.contains(&"aiqudao_guanli".to_string()));
    }

    #[test]
    fn ceshi_zhineng_pipei() {
        let mut peiqi = Guanjiancipeiqi::xinjian();
        peiqi.charu("时间", "shijian_chaxun");
        peiqi.charu("查询", "shijian_chaxun");
        peiqi.charu("渠道", "aiqudao_guanli");
        peiqi.charu("管理", "aiqudao_guanli");

        let jieguo = peiqi.zhineng_pipei("查询时间和管理渠道");
        // shijian_chaxun 匹配2个关键词，aiqudao_guanli 匹配2个关键词
        assert_eq!(jieguo.len(), 2);
    }

    #[test]
    fn ceshi_qianzui_pipei() {
        let mut peiqi = Guanjiancipeiqi::xinjian();
        peiqi.charu("时间查询", "shijian_chaxun");
        peiqi.charu("时间戳", "shijian_chaxun");

        let jieguo = peiqi.qianzui_pipei("时间");
        assert!(jieguo.contains(&"shijian_chaxun".to_string()));
    }
}
