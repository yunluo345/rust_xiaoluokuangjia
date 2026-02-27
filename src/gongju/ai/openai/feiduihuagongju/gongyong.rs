use crate::gongju::ai::openai::{aixiaoxiguanli, openaizhuti};
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use sha2::{Digest, Sha256};
use serde_json::{json, Value};
use std::collections::HashSet;

/// 通用AI文本请求
pub async fn ai_putongqingqiu_wenben(xitongtishici: &str, yonghuxiaoxi: String, chaoshi: u64) -> Option<String> {
    let aipeizhi = crate::jiekouxt::jiekou_nr::ai::huoqu_peizhi().await?
        .shezhi_chaoshi(chaoshi)
        .shezhi_chongshi(1);

    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(xitongtishici);
    guanli.zhuijia_yonghuxiaoxi(yonghuxiaoxi);

    openaizhuti::putongqingqiu(&aipeizhi, &guanli)
        .await
        .map(|h| h.trim().to_string())
}

/// 清理AI返回的JSON（去除markdown代码块标记）
pub fn jinghua_json_huifu(huifu: &str) -> &str {
    huifu.trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
}

/// 计算SHA256哈希
pub fn jisuan_sha256(wenben: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(wenben.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// 按字符数分段（支持重叠）
pub fn anzi_fenduan(wenben: &str, meiduanchangdu: usize, zhongdiechangdu: usize, zuidaduanshu: usize) -> Vec<String> {
    if meiduanchangdu == 0 || zuidaduanshu == 0 {
        return vec![wenben.to_string()];
    }
    let zifu: Vec<char> = wenben.chars().collect();
    if zifu.len() <= meiduanchangdu {
        return vec![wenben.to_string()];
    }

    let buchang = meiduanchangdu.saturating_sub(zhongdiechangdu).max(1);
    let mut kaishi = 0usize;
    let mut duanlie: Vec<String> = Vec::new();
    while kaishi < zifu.len() && duanlie.len() < zuidaduanshu {
        let jieshu = (kaishi + meiduanchangdu).min(zifu.len());
        duanlie.push(zifu[kaishi..jieshu].iter().collect());
        if jieshu >= zifu.len() {
            break;
        }
        kaishi += buchang;
    }
    duanlie
}

/// 从Value中提取字符串（兼容字符串和数字类型）
pub fn huoquzifuchuan(shuju: &Value, ziduan: &str) -> Option<String> {
    shuju.get(ziduan).and_then(|v| {
        v.as_str()
            .map(|s| s.to_string())
            .or_else(|| v.as_i64().map(|n| n.to_string()))
            .or_else(|| v.as_u64().map(|n| n.to_string()))
    })
}

/// 验证必填标签是否齐全
pub fn yanzheng_bitian_biaoqian(tiquxiang: &[(String, String)], peizhi: &Ai) -> Option<Vec<String>> {
    let bitian_mingcheng: HashSet<String> = peizhi.ribao_biaoqian.iter()
        .filter(|bq| bq.bitian)
        .map(|bq| bq.mingcheng.clone())
        .collect();

    let yitiqumingcheng: HashSet<String> = tiquxiang.iter()
        .map(|(ming, _)| ming.clone())
        .collect();

    let queshi: Vec<String> = bitian_mingcheng.difference(&yitiqumingcheng)
        .cloned()
        .collect();

    (!queshi.is_empty()).then_some(queshi)
}

/// 处理文本型AI生成结果（标题/摘要），成功时写入对应字段
pub async fn chuli_wenben_aijieguo(xuyao: bool, yuanshi: Option<String>, ziduan: &str, mingcheng: &str, renwuid: &str, ribaoid: &str) -> Option<String> {
    use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::shujucaozuo_ribao;
    match (xuyao, yuanshi) {
        (true, Some(zhi)) => {
            let _ = shujucaozuo_ribao::gengxin(ribaoid, &[(ziduan, zhi.as_str())]).await;
            println!("[任务处理] 任务={} 日报={} {}已生成", renwuid, ribaoid, mingcheng);
            Some(zhi)
        }
        (true, None) => {
            println!("[任务处理] 任务={} {}生成失败，跳过", renwuid, mingcheng);
            None
        }
        _ => None,
    }
}

/// 处理扩展JSON型AI生成结果（思维导图/关系分析），成功时合并至kuozhan
pub fn chuli_kuozhan_aijieguo(
    xuyao: bool, yuanshi: Option<String>, jian: &str, mingcheng: &str,
    renwuid: &str, ribaoid: &str, kuozhan: &mut Value, yigengxin: &mut bool,
    ewaijian: Option<(&str, &str)>,
) -> bool {
    match (xuyao, yuanshi) {
        (true, Some(wenben)) => {
            if let Ok(json) = serde_json::from_str::<Value>(&wenben) {
                kuozhan[jian] = json;
                if let Some((k, v)) = ewaijian {
                    kuozhan[k] = Value::String(v.to_string());
                }
                *yigengxin = true;
            }
            println!("[任务处理] 任务={} 日报={} {}已生成", renwuid, ribaoid, mingcheng);
            true
        }
        (true, None) => {
            println!("[任务处理] 任务={} {}生成失败，跳过", renwuid, mingcheng);
            false
        }
        _ => false,
    }
}

/// 解析kuozhan为结构化JSON，兼容旧格式
pub fn jiexi_kuozhan(kuozhan_str: Option<&str>) -> Value {
    let raw = match kuozhan_str.filter(|s| !s.trim().is_empty()) {
        Some(s) => s,
        None => return json!({}),
    };
    match serde_json::from_str::<Value>(raw) {
        Ok(v) if v.get("siweidaotu").is_some() => v,
        Ok(v) if v.is_object() => json!({"siweidaotu": v}),
        _ => json!({}),
    }
}
