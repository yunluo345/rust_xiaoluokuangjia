use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use serde_json::{json, Value};
use std::collections::HashMap;

use super::gongyong::{ai_putongqingqiu_wenben, jinghua_json_huifu, anzi_fenduan};

/// 从AI回复中提取关系列表
fn tiqu_guanxilie(huifu: &str) -> Vec<Value> {
    let jinghua = jinghua_json_huifu(huifu);
    let json_obj = match serde_json::from_str::<Value>(jinghua) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    json_obj
        .get("guanxi")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
}

/// 关系聚合项
struct GuanxiJuheXiang {
    miaoshulie: Vec<String>,
    zuigao_xindu: f64,
    zhengjulie: Vec<String>,
    juese_ren1: Option<String>,
    juese_ren2: Option<String>,
    qinggan_qingxiang: Option<String>,
}

/// 合并去重关系列表
fn hebing_guanxilie(guanxilie: Vec<Value>) -> Vec<Value> {
    let mut juhe: HashMap<(String, String, String), GuanxiJuheXiang> = HashMap::new();
    for gx in guanxilie {
        let ren1 = match gx.get("ren1").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) {
            Some(s) => s.to_string(),
            None => continue,
        };
        let ren2 = match gx.get("ren2").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) {
            Some(s) => s.to_string(),
            None => continue,
        };
        let guanxi = match gx.get("guanxi").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) {
            Some(s) => s.to_string(),
            None => continue,
        };
        let miaoshu = gx.get("miaoshu").and_then(|v| v.as_str()).map(str::trim).unwrap_or("").to_string();
        let xindu = gx.get("xindu").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let zhengju = gx.get("zhengjupianduan").and_then(|v| v.as_str()).map(str::trim).unwrap_or("").to_string();
        let juese_r1 = gx.get("juese").and_then(|j| j.get("ren1")).and_then(|v| v.as_str()).map(str::trim)
            .filter(|s| !s.is_empty()).map(String::from);
        let juese_r2 = gx.get("juese").and_then(|j| j.get("ren2")).and_then(|v| v.as_str()).map(str::trim)
            .filter(|s| !s.is_empty()).map(String::from);
        let qinggan = gx.get("qinggan_qingxiang").and_then(|v| v.as_str()).map(str::trim)
            .filter(|s| !s.is_empty()).map(String::from);

        let (a, b, jr1, jr2) = if ren1 <= ren2 {
            (ren1, ren2, juese_r1, juese_r2)
        } else {
            (ren2, ren1, juese_r2, juese_r1)
        };
        let entry = juhe.entry((a, b, guanxi)).or_insert_with(|| GuanxiJuheXiang {
            miaoshulie: Vec::new(),
            zuigao_xindu: 0.0,
            zhengjulie: Vec::new(),
            juese_ren1: None,
            juese_ren2: None,
            qinggan_qingxiang: None,
        });
        if !miaoshu.is_empty() && !entry.miaoshulie.contains(&miaoshu) {
            entry.miaoshulie.push(miaoshu);
        }
        if xindu > entry.zuigao_xindu {
            entry.zuigao_xindu = xindu;
        }
        if !zhengju.is_empty() && !entry.zhengjulie.contains(&zhengju) {
            entry.zhengjulie.push(zhengju);
        }
        if entry.juese_ren1.is_none() {
            entry.juese_ren1 = jr1;
        }
        if entry.juese_ren2.is_none() {
            entry.juese_ren2 = jr2;
        }
        if entry.qinggan_qingxiang.is_none() {
            entry.qinggan_qingxiang = qinggan;
        } else if let Some(ref xin) = qinggan {
            // 负面情感优先保留
            if xin == "负面" {
                entry.qinggan_qingxiang = Some(xin.clone());
            }
        }
    }

    juhe.into_iter().map(|((ren1, ren2, guanxi), xiang)| {
        let mut jieguo = json!({
            "ren1": ren1,
            "ren2": ren2,
            "guanxi": guanxi,
            "miaoshu": xiang.miaoshulie.join("；"),
            "xindu": xiang.zuigao_xindu,
            "zhengjupianduan": xiang.zhengjulie.join("；"),
        });
        if xiang.juese_ren1.is_some() || xiang.juese_ren2.is_some() {
            jieguo["juese"] = json!({
                "ren1": xiang.juese_ren1.unwrap_or_default(),
                "ren2": xiang.juese_ren2.unwrap_or_default(),
            });
        }
        if let Some(qg) = xiang.qinggan_qingxiang {
            jieguo["qinggan_qingxiang"] = Value::String(qg);
        }
        jieguo
    }).collect()
}

/// AI关系分析（支持长文本分段）
pub async fn ai_shengcheng_guanxifenxi(neirong: &str, peizhi: &Ai) -> Option<String> {
    let neirong_changdu = neirong.chars().count();
    let danpian_shangxian = peizhi.guanxifenxi_danpian_zifushangxian.max(500);
    if neirong_changdu <= danpian_shangxian {
        let huifu = ai_putongqingqiu_wenben(
            &peizhi.guanxifenxi_tishici,
            format!("请分析以下日报中的人物关系：\n\n{}", neirong),
            60,
        ).await?;
        let guanxilie = hebing_guanxilie(tiqu_guanxilie(&huifu));
        if guanxilie.is_empty() {
            println!("[关系分析] 单篇模式未提取到有效关系");
            return None;
        }
        let jieguo = json!({"guanxi": guanxilie}).to_string();
        println!("[关系分析] 单篇模式成功 长度={}", jieguo.len());
        return Some(jieguo);
    }

    let duanlie = anzi_fenduan(
        neirong,
        peizhi.guanxifenxi_fenduan_daxiao.max(200),
        peizhi.guanxifenxi_fenduan_zhongdie.min(peizhi.guanxifenxi_fenduan_daxiao.saturating_sub(1)),
        peizhi.guanxifenxi_zuida_fenduanshu.max(1),
    );
    println!(
        "[关系分析] 超长日报启用分段模式 原始长度={} 分段数={}",
        neirong_changdu,
        duanlie.len()
    );

    let mut zong_guanxilie: Vec<Value> = Vec::new();
    for (idx, duan) in duanlie.iter().enumerate() {
        let huifu = match ai_putongqingqiu_wenben(
            &peizhi.guanxifenxi_tishici,
            format!("请分析以下日报中的人物关系（第{}/{}段）：\n\n{}", idx + 1, duanlie.len(), duan),
            60,
        ).await {
            Some(h) => h,
            None => {
                println!("[关系分析] 分段{}调用失败，已跳过", idx + 1);
                continue;
            }
        };
        let guanxilie = tiqu_guanxilie(&huifu);
        if guanxilie.is_empty() {
            println!("[关系分析] 分段{}未提取到关系", idx + 1);
            continue;
        }
        zong_guanxilie.extend(guanxilie);
    }

    let hebinghou = hebing_guanxilie(zong_guanxilie);
    if hebinghou.is_empty() {
        println!("[关系分析] 分段模式未提取到有效关系");
        return None;
    }
    let jieguo = json!({"guanxi": hebinghou}).to_string();
    println!("[关系分析] 分段模式成功 长度={}", jieguo.len());
    Some(jieguo)
}
