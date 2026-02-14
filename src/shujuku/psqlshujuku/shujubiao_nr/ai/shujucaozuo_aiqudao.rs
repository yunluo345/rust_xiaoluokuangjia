use serde_json::Value;
use rand::prelude::*;
use crate::gongju::jichugongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "aiqudao";

#[allow(non_upper_case_globals)]
pub const yunxuleixing: &[&str] = &["openai", "xiangliang"];

// 常量定义：状态值
#[allow(non_upper_case_globals)]
const zhuangtai_qiyong: &str = "1";
#[allow(non_upper_case_globals)]
const zhuangtai_jinyong: &str = "0";

// 常量定义：整数类型字段列表（需要在SQL中添加::INTEGER类型转换）
#[allow(non_upper_case_globals)]
const zhengshu_ziduan: &[&str] = &["zuidatoken", "youxianji"];

/// 判断字段是否为整数类型
fn shi_zhengshu_ziduan(ziduanming: &str) -> bool {
    zhengshu_ziduan.contains(&ziduanming)
}

/// 构建SQL SET子句，自动为整数类型字段添加类型转换
fn goujian_shezhi_ziju(ziduanming: &str, canshu_suoyin: usize) -> String {
    if shi_zhengshu_ziduan(ziduanming) {
        format!("{} = ${}::INTEGER", ziduanming, canshu_suoyin)
    } else {
        format!("{} = ${}", ziduanming, canshu_suoyin)
    }
}

/// 新增AI渠道，返回自增ID
pub async fn xinzeng(mingcheng: &str, leixing: &str, jiekoudizhi: &str, miyao: &str, moxing: &str, wendu: &str, beizhu: Option<&str>, zuidatoken: Option<&str>) -> Option<String> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let zuidatoken_zhi = zuidatoken.unwrap_or("0");
    let jieguo = psqlcaozuo::chaxun(
        &format!("INSERT INTO {} (mingcheng, leixing, jiekoudizhi, miyao, moxing, wendu, zhuangtai, youxianji, beizhu, zuidatoken, chuangjianshijian, gengxinshijian) VALUES ($1,$2,$3,$4,$5,$6,$7,$8::INTEGER,$9,$10::INTEGER,$11,$12) RETURNING id::TEXT", biaoming),
        &[Some(mingcheng), Some(leixing), Some(jiekoudizhi), Some(miyao), Some(moxing), Some(wendu), Some(zhuangtai_qiyong), Some("0"), beizhu, Some(zuidatoken_zhi), Some(&shijian), Some(&shijian)],
    ).await?;
    jieguo.first().and_then(|v| v.get("id")?.as_str().map(String::from))
}

/// 根据ID删除渠道
pub async fn shanchu(id: &str) -> Option<u64> {
    psqlcaozuo::zhixing(
        &format!("DELETE FROM {} WHERE id = $1::BIGINT", biaoming),
        &[Some(id)],
    ).await
}

/// 根据ID更新渠道信息（仅更新传入的非None字段）
pub async fn gengxin(id: &str, ziduanlie: &[(&str, &str)]) -> Option<u64> {
    if ziduanlie.is_empty() {
        return None;
    }
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let mut shezhi: Vec<String> = ziduanlie.iter().enumerate()
        .map(|(i, (ming, _))| goujian_shezhi_ziju(ming, i + 2))
        .collect();
    shezhi.push(format!("gengxinshijian = ${}", ziduanlie.len() + 2));
    let sql = format!("UPDATE {} SET {} WHERE id = $1::BIGINT", biaoming, shezhi.join(", "));
    let mut canshu: Vec<Option<&str>> = vec![Some(id)];
    canshu.extend(ziduanlie.iter().map(|(_, zhi)| Some(*zhi)));
    canshu.push(Some(&shijian));
    psqlcaozuo::zhixing(&sql, &canshu).await
}

/// 根据ID查询单个渠道
pub async fn chaxun_id(id: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE id = $1::BIGINT", biaoming),
        &[Some(id)],
    ).await?;
    jieguo.into_iter().next()
}

/// 查询所有渠道（按优先级降序）
pub async fn chaxun_quanbu() -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} ORDER BY youxianji DESC, chuangjianshijian DESC", biaoming),
        &[],
    ).await
}

/// 查询所有启用的渠道（按优先级降序）
pub async fn chaxun_qiyong() -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE zhuangtai = $1 ORDER BY youxianji DESC", biaoming),
        &[Some(zhuangtai_qiyong)],
    ).await
}

/// 根据渠道类型查询
pub async fn chaxun_leixing(leixing: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE leixing = $1 AND zhuangtai = $2 ORDER BY COALESCE(youxianji, 0) DESC", biaoming),
        &[Some(leixing), Some(zhuangtai_qiyong)],
    ).await
}

/// 切换渠道启用/禁用状态
pub async fn qiehuanzhuangtai(id: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET zhuangtai = CASE WHEN zhuangtai = $2 THEN $3 ELSE $2 END, gengxinshijian = $4 WHERE id = $1::BIGINT", biaoming),
        &[Some(id), Some(zhuangtai_qiyong), Some(zhuangtai_jinyong), Some(&shijian)],
    ).await
}

/// 更新渠道优先级
pub async fn gengxinyouxianji(id: &str, youxianji: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET youxianji = $2::INTEGER, gengxinshijian = $3 WHERE id = $1::BIGINT", biaoming),
        &[Some(id), Some(youxianji), Some(&shijian)],
    ).await
}

/// 检查渠道名称是否已存在
pub async fn mingchengcunzai(mingcheng: &str) -> bool {
    psqlcaozuo::chaxun(
        &format!("SELECT 1 FROM {} WHERE mingcheng = $1 LIMIT 1", biaoming),
        &[Some(mingcheng)],
    ).await
    .is_some_and(|jieguo| !jieguo.is_empty())
}

/// 统计渠道总数
pub async fn tongjishuliang() -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT COUNT(*) as shuliang FROM {}", biaoming),
        &[],
    ).await?;
    jieguo.into_iter().next()
}

pub fn leixingyunxu(leixing: &str) -> bool {
    yunxuleixing.contains(&leixing)
}

/// 按类型轮训选取渠道：启用状态、优先级最高、同优先级随机
pub async fn lunxun(leixing: &str) -> Option<Value> {
    let liebie = chaxun_leixing(leixing).await?;
    if liebie.is_empty() {
        return None;
    }
    let zuigao = liebie[0].get("youxianji").and_then(quzhengshu).unwrap_or(0);
    let houxuanlie: Vec<&Value> = liebie.iter()
        .take_while(|v| v.get("youxianji").and_then(quzhengshu).unwrap_or(0) == zuigao)
        .collect();
    houxuanlie.choose(&mut rand::rng()).map(|v| (*v).clone())
}

/// 带重试机制的渠道获取：根据配置自动重试
pub async fn lunxun_daichongshi(leixing: &str) -> Result<Value, QudaoCuowu> {
    use crate::peizhixt::peizhixitongzhuti;
    use crate::peizhixt::peizhi_nr::peizhi_ai::Aipeizhi;
    
    // 读取配置
    let peizhi = peizhixitongzhuti::duqupeizhi::<Aipeizhi>(Aipeizhi::wenjianming())
        .unwrap_or_default();
    let qudaopeizhi = &peizhi.qudaohuoqu;
    
    // 如果未启用重试，直接尝试一次
    if !qudaopeizhi.qiyongchongshi {
        return lunxun(leixing).await.ok_or(QudaoCuowu::MeiyouKeyongQudao);
    }
    
    // 启用重试机制
    let mut zuihouyici_cuowu = QudaoCuowu::MeiyouKeyongQudao;
    for cishu in 1..=qudaopeizhi.chongshicishu {
        match lunxun(leixing).await {
            Some(qudao) => {
                if cishu > 1 {
                    println!("[渠道获取] 第{}次重试成功，类型: {}", cishu, leixing);
                }
                return Ok(qudao);
            }
            None => {
                zuihouyici_cuowu = QudaoCuowu::MeiyouKeyongQudao;
                if cishu < qudaopeizhi.chongshicishu {
                    println!("[渠道获取] 第{}次失败，{}毫秒后重试，类型: {}", cishu, qudaopeizhi.chongshijiange, leixing);
                    tokio::time::sleep(tokio::time::Duration::from_millis(qudaopeizhi.chongshijiange)).await;
                }
            }
        }
    }
    
    println!("[渠道获取] 重试{}次后仍失败，类型: {}", qudaopeizhi.chongshicishu, leixing);
    Err(zuihouyici_cuowu)
}

/// 渠道获取错误类型
#[derive(Debug, Clone)]
pub enum QudaoCuowu {
    /// 没有可用的渠道
    MeiyouKeyongQudao,
}

impl QudaoCuowu {
    /// 获取错误消息
    pub fn xiaoxi(&self) -> &'static str {
        match self {
            QudaoCuowu::MeiyouKeyongQudao => "没有可用的AI渠道，请稍后重试",
        }
    }
    
    /// 获取HTTP状态码
    pub fn zhuangtaima(&self) -> u16 {
        match self {
            QudaoCuowu::MeiyouKeyongQudao => 503,
        }
    }
}

fn quzhengshu(v: &Value) -> Option<i64> {
    v.as_i64().or_else(|| v.as_str()?.parse().ok())
}
