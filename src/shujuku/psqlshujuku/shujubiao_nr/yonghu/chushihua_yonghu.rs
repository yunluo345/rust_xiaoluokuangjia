use super::shujucaozuo_yonghu;
use super::shujucaozuo_yonghuzu;
use crate::shujuku::psqlshujuku::shujubiao_nr::shujucaozuo_jiekoujilubiao;

#[allow(non_upper_case_globals)]
const root_zumingcheng: &str = "root";
#[allow(non_upper_case_globals)]
const user_zumingcheng: &str = "user";
#[allow(non_upper_case_globals)]
const moren_zhanghao: &str = "xiaoluo";
#[allow(non_upper_case_globals)]
const moren_mima: &str = "5201314";
#[allow(non_upper_case_globals)]
const moren_nicheng: &str = "小落";

async fn huoquhuochuangjian(mingcheng: &str, beizhu: &str) -> Option<String> {
    if let Some(zu) = shujucaozuo_yonghuzu::chaxun_mingcheng(mingcheng).await {
        return zu.get("id")?.as_str().map(String::from);
    }
    shujucaozuo_yonghuzu::xinzeng(mingcheng, Some(beizhu)).await
}

async fn goujianheimingdan() -> String {
    shujucaozuo_jiekoujilubiao::chaxun_jinyongputong().await
        .map(|lie| serde_json::to_string(&lie).unwrap_or_default())
        .unwrap_or_else(|| "[]".to_string())
}

/// 初始化用户组和默认管理员
pub async fn chushihua() -> bool {
    let root_id = match huoquhuochuangjian(root_zumingcheng, "超级管理员组，拥有所有权限").await {
        Some(id) => id,
        None => return false,
    };

    let user_id = if let Some(zu) = shujucaozuo_yonghuzu::chaxun_mingcheng(user_zumingcheng).await {
        match zu.get("id").and_then(|v| v.as_str()) {
            Some(id) => id.to_string(),
            None => return false,
        }
    } else {
        let heimingdan = goujianheimingdan().await;
        let id = match shujucaozuo_yonghuzu::xinzeng(user_zumingcheng, Some("普通用户组")).await {
            Some(id) => id,
            None => return false,
        };
        shujucaozuo_yonghuzu::gengxinjinjiekou(&id, &heimingdan).await;
        shujucaozuo_yonghuzu::shezhimorenzhu(&id).await;
        id
    };

    let _ = user_id;

    let yonghuliebie = shujucaozuo_yonghu::chaxun_yonghuzuid(&root_id).await;
    let xuyaochuangjian = yonghuliebie.as_ref().is_none_or(|lie| lie.is_empty());

    if xuyaochuangjian {
        if shujucaozuo_yonghu::xinzeng(moren_zhanghao, moren_mima, moren_nicheng, &root_id, Some("系统自动创建的管理员")).await.is_none() {
            return false;
        }
    }

    true
}
