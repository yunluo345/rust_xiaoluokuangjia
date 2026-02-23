use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::shujucaozuo_yonghu;
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::shujucaozuo_yonghuzu;
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::yonghuyanzheng;
use crate::shujuku::psqlshujuku::shujubiao_nr::shujucaozuo_jiekoujilubiao;
use crate::shujuku::redisshujuku::rediscaozuo;
use crate::gongju::jwtgongju;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/yonghuguanli",
    nicheng: "用户管理",
    jieshao: "管理员接口，支持用户和用户组的完整增删改查操作",
    fangshi: Qingqiufangshi::Post,
    jiami: true,
    xudenglu: true,
    xuyonghuzu: true,
    yunxuputong: false,
};

#[derive(Deserialize)]
struct Qingqiuti {
    caozuo: String,
    dangqianyeshu: Option<i64>,
    meiyeshuliang: Option<i64>,
    guanjianci: Option<String>,
    id: Option<String>,
    zhanghao: Option<String>,
    nicheng: Option<String>,
    mima: Option<String>,
    beizhu: Option<String>,
    yuanyin: Option<String>,
    jieshu: Option<String>,
    yonghuzuid: Option<String>,
    yonghuzu: Option<String>,
    yonghuzu_id: Option<String>,
    mingcheng: Option<String>,
    jinjiekou: Option<Vec<String>>,
    jichengyonghuzuid: Option<String>,
}

#[derive(Serialize)]
struct Xiangyingti {
    liebiao: Vec<serde_json::Value>,
    zongshu: i64,
}

enum Caozuoleixing {
    Fenye { dangqianyeshu: i64, meiyeshuliang: i64 },
    Sousuo { guanjianci: String, dangqianyeshu: i64, meiyeshuliang: i64 },
    Xiangqing(String),
    Xiugaizhanghao { id: String, zhanghao: String },
    Xiugainicheng { id: String, nicheng: String },
    Xiugaimima { id: String, mima: String },
    Xiugaibeizhu { id: String, beizhu: String },
    Fengjin { id: String, yuanyin: String, jieshu: Option<String> },
    Jiefeng(String),
    Xinzeng { zhanghao: String, mima: String, nicheng: String, yonghuzuid: String, beizhu: Option<String> },
    Shanchu(String),
    Yonghuzufenye { dangqianyeshu: i64, meiyeshuliang: i64 },
    Yonghuzusousuo { guanjianci: String, dangqianyeshu: i64, meiyeshuliang: i64 },
    Yonghuzuxiangqing(String),
    Yonghuzuxinzeng { mingcheng: String, beizhu: Option<String>, jichengyonghuzuid: Option<String> },
    Yonghuzuxiugai { id: String, mingcheng: String, beizhu: Option<String> },
    Yonghuzushanchu(String),
    Yonghuzujiekouliebiao,
    Yonghuzuhuoqujinjiekou(String),
    Yonghuzugengxinjinjiekou { id: String, jinjiekou: Vec<String> },
}

fn jiamishibai(zhuangtaima: u16, xiaoxi: impl Into<String>, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(zhuangtaima, xiaoxi), miyao)
}

fn tiqucansu(zhi: Option<String>, mingcheng: &str, miyao: &[u8]) -> Result<String, HttpResponse> {
    zhi.ok_or_else(|| jiamishibai(400, format!("缺少参数: {}", mingcheng), miyao))
}

fn yanzhengfenye(dangqianyeshu: Option<i64>, meiyeshuliang: Option<i64>, miyao: &[u8]) -> Result<(i64, i64), HttpResponse> {
    let ye = dangqianyeshu.ok_or_else(|| jiamishibai(400, "缺少参数: dangqianyeshu", miyao))?;
    let liang = meiyeshuliang.ok_or_else(|| jiamishibai(400, "缺少参数: meiyeshuliang", miyao))?;
    match (ye > 0, liang > 0) {
        (true, true) => Ok((ye, liang)),
        _ => Err(jiamishibai(400, "页数和数量必须大于0", miyao)),
    }
}

fn tiquzongshu(jieguo: &serde_json::Value) -> i64 {
    jieguo.get("shuliang").and_then(|v| v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse().ok()))).unwrap_or(0)
}

fn jiexi_caozuo(qingqiu: Qingqiuti, miyao: &[u8]) -> Result<Caozuoleixing, HttpResponse> {
    match qingqiu.caozuo.as_str() {
        "fenye" => {
            let (dangqianyeshu, meiyeshuliang) = yanzhengfenye(qingqiu.dangqianyeshu, qingqiu.meiyeshuliang, miyao)?;
            Ok(Caozuoleixing::Fenye { dangqianyeshu, meiyeshuliang })
        }
        "sousuo" => {
            let guanjianci = tiqucansu(qingqiu.guanjianci, "guanjianci", miyao)?;
            let (dangqianyeshu, meiyeshuliang) = yanzhengfenye(qingqiu.dangqianyeshu, qingqiu.meiyeshuliang, miyao)?;
            Ok(Caozuoleixing::Sousuo { guanjianci, dangqianyeshu, meiyeshuliang })
        }
        "xiangqing" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            Ok(Caozuoleixing::Xiangqing(id))
        }
        "xiugai_zhanghao" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            let zhanghao = tiqucansu(qingqiu.zhanghao, "zhanghao", miyao)?;
            if zhanghao.is_empty() { return Err(jiamishibai(400, "账号不能为空", miyao)); }
            Ok(Caozuoleixing::Xiugaizhanghao { id, zhanghao })
        }
        "xiugai_nicheng" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            let nicheng = tiqucansu(qingqiu.nicheng, "nicheng", miyao)?;
            if nicheng.is_empty() { return Err(jiamishibai(400, "昵称不能为空", miyao)); }
            Ok(Caozuoleixing::Xiugainicheng { id, nicheng })
        }
        "xiugai_mima" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            let mima = tiqucansu(qingqiu.mima, "mima", miyao)?;
            if mima.is_empty() { return Err(jiamishibai(400, "密码不能为空", miyao)); }
            Ok(Caozuoleixing::Xiugaimima { id, mima })
        }
        "xiugai_beizhu" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            let beizhu = qingqiu.beizhu.unwrap_or_default();
            Ok(Caozuoleixing::Xiugaibeizhu { id, beizhu })
        }
        "fengjin" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            let yuanyin = tiqucansu(qingqiu.yuanyin, "yuanyin", miyao)?;
            if yuanyin.is_empty() { return Err(jiamishibai(400, "封禁原因不能为空", miyao)); }
            Ok(Caozuoleixing::Fengjin { id, yuanyin, jieshu: qingqiu.jieshu })
        }
        "jiefeng" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            Ok(Caozuoleixing::Jiefeng(id))
        }
        "xinzeng" => {
            let zhanghao = tiqucansu(qingqiu.zhanghao, "zhanghao", miyao)?;
            if zhanghao.is_empty() { return Err(jiamishibai(400, "账号不能为空", miyao)); }
            let mima = tiqucansu(qingqiu.mima, "mima", miyao)?;
            if mima.is_empty() { return Err(jiamishibai(400, "密码不能为空", miyao)); }
            let nicheng = tiqucansu(qingqiu.nicheng, "nicheng", miyao)?;
            if nicheng.is_empty() { return Err(jiamishibai(400, "昵称不能为空", miyao)); }
            let yonghuzuid = tiqucansu(
                qingqiu.yonghuzuid.or(qingqiu.yonghuzu).or(qingqiu.yonghuzu_id),
                "yonghuzuid",
                miyao
            )?;
            if yonghuzuid.is_empty() { return Err(jiamishibai(400, "用户组ID不能为空", miyao)); }
            Ok(Caozuoleixing::Xinzeng { zhanghao, mima, nicheng, yonghuzuid, beizhu: qingqiu.beizhu })
        }
        "shanchu" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            Ok(Caozuoleixing::Shanchu(id))
        }
        "yonghuzu_fenye" => {
            let (dangqianyeshu, meiyeshuliang) = yanzhengfenye(qingqiu.dangqianyeshu, qingqiu.meiyeshuliang, miyao)?;
            Ok(Caozuoleixing::Yonghuzufenye { dangqianyeshu, meiyeshuliang })
        }
        "yonghuzu_sousuo" => {
            let guanjianci = tiqucansu(qingqiu.guanjianci, "guanjianci", miyao)?;
            let (dangqianyeshu, meiyeshuliang) = yanzhengfenye(qingqiu.dangqianyeshu, qingqiu.meiyeshuliang, miyao)?;
            Ok(Caozuoleixing::Yonghuzusousuo { guanjianci, dangqianyeshu, meiyeshuliang })
        }
        "yonghuzu_xiangqing" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            Ok(Caozuoleixing::Yonghuzuxiangqing(id))
        }
        "yonghuzu_xinzeng" => {
            let mingcheng = tiqucansu(qingqiu.mingcheng, "mingcheng", miyao)?;
            if mingcheng.is_empty() { return Err(jiamishibai(400, "组名称不能为空", miyao)); }
            Ok(Caozuoleixing::Yonghuzuxinzeng { mingcheng, beizhu: qingqiu.beizhu, jichengyonghuzuid: qingqiu.jichengyonghuzuid })
        }
        "yonghuzu_xiugai" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            let mingcheng = tiqucansu(qingqiu.mingcheng, "mingcheng", miyao)?;
            if mingcheng.is_empty() { return Err(jiamishibai(400, "组名称不能为空", miyao)); }
            Ok(Caozuoleixing::Yonghuzuxiugai { id, mingcheng, beizhu: qingqiu.beizhu })
        }
        "yonghuzu_shanchu" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            Ok(Caozuoleixing::Yonghuzushanchu(id))
        }
        "yonghuzu_jiekouliebiao" => Ok(Caozuoleixing::Yonghuzujiekouliebiao),
        "yonghuzu_huoqujinjiekou" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            Ok(Caozuoleixing::Yonghuzuhuoqujinjiekou(id))
        }
        "yonghuzu_gengxinjinjiekou" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            let jinjiekou = qingqiu.jinjiekou.unwrap_or_default();
            Ok(Caozuoleixing::Yonghuzugengxinjinjiekou { id, jinjiekou })
        }
        _ => Err(jiamishibai(400, "无效的操作类型", miyao)),
    }
}

async fn zhixing_caozuo(caozuo: Caozuoleixing, miyao: &[u8]) -> HttpResponse {
    match caozuo {
        Caozuoleixing::Fenye { dangqianyeshu, meiyeshuliang } => {
            let pianyi = ((dangqianyeshu - 1) * meiyeshuliang).to_string();
            let shuliang = meiyeshuliang.to_string();
            let liebiao = match shujucaozuo_yonghu::chaxun_fenye(&pianyi, &shuliang).await {
                Some(l) => l,
                None => return jiamishibai(500, "查询用户列表失败", miyao),
            };
            let zongshu_jieguo = match shujucaozuo_yonghu::chaxun_zongshu().await {
                Some(j) => j,
                None => return jiamishibai(500, "查询用户总数失败", miyao),
            };
            let xiangying = Xiangyingti { liebiao, zongshu: tiquzongshu(&zongshu_jieguo) };
            jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("查询成功", xiangying), miyao)
        }
        Caozuoleixing::Sousuo { guanjianci, dangqianyeshu, meiyeshuliang } => {
            let pianyi = ((dangqianyeshu - 1) * meiyeshuliang).to_string();
            let shuliang = meiyeshuliang.to_string();
            let liebiao = match shujucaozuo_yonghu::sousuo_mohu(&guanjianci, &pianyi, &shuliang).await {
                Some(l) => l,
                None => return jiamishibai(500, "搜索用户失败", miyao),
            };
            let zongshu_jieguo = match shujucaozuo_yonghu::sousuo_zongshu(&guanjianci).await {
                Some(j) => j,
                None => return jiamishibai(500, "查询搜索总数失败", miyao),
            };
            let xiangying = Xiangyingti { liebiao, zongshu: tiquzongshu(&zongshu_jieguo) };
            jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("搜索成功", xiangying), miyao)
        }
        Caozuoleixing::Xiangqing(id) => {
            match shujucaozuo_yonghu::chaxun_id(&id).await {
                Some(yonghu) => jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("查询成功", yonghu), miyao),
                None => jiamishibai(404, "用户不存在", miyao),
            }
        }
        Caozuoleixing::Xiugaizhanghao { id, zhanghao } => {
            if shujucaozuo_yonghu::zhanghaocunzai(&zhanghao).await {
                return jiamishibai(400, "该账号已存在", miyao);
            }
            match shujucaozuo_yonghu::gengxin(&id, &[("zhanghao", &zhanghao)]).await {
                Some(_) => jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong_wushuju("账号修改成功"), miyao),
                None => jiamishibai(500, "账号修改失败", miyao),
            }
        }
        Caozuoleixing::Xiugainicheng { id, nicheng } => {
            match shujucaozuo_yonghu::gengxin(&id, &[("nicheng", &nicheng)]).await {
                Some(_) => jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong_wushuju("昵称修改成功"), miyao),
                None => jiamishibai(500, "昵称修改失败", miyao),
            }
        }
        Caozuoleixing::Xiugaimima { id, mima } => {
            match shujucaozuo_yonghu::gengxin(&id, &[("mima", &mima)]).await {
                Some(_) => {
                    let _ = jwtgongju::zhuxiao(&id).await;
                    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong_wushuju("密码修改成功"), miyao)
                }
                None => jiamishibai(500, "密码修改失败", miyao),
            }
        }
        Caozuoleixing::Xiugaibeizhu { id, beizhu } => {
            match shujucaozuo_yonghu::gengxin(&id, &[("beizhu", &beizhu)]).await {
                Some(_) => jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong_wushuju("备注修改成功"), miyao),
                None => jiamishibai(500, "备注修改失败", miyao),
            }
        }
        Caozuoleixing::Fengjin { id, yuanyin, jieshu } => {
            let yonghu = match shujucaozuo_yonghu::chaxun_id(&id).await {
                Some(y) => y,
                None => return jiamishibai(404, "用户不存在", miyao),
            };
            let yonghuzuid = yonghu.get("yonghuzuid").and_then(|v| v.as_str()).unwrap_or("");
            if yonghuyanzheng::shifouroot(yonghuzuid).await {
                return jiamishibai(403, "root用户组不可被封禁", miyao);
            }
            match shujucaozuo_yonghu::fengjin(&id, &yuanyin, jieshu.as_deref()).await {
                Some(_) => {
                    let _ = jwtgongju::zhuxiao(&id).await;
                    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong_wushuju("封禁成功"), miyao)
                }
                None => jiamishibai(500, "封禁失败", miyao),
            }
        }
        Caozuoleixing::Jiefeng(id) => {
            match shujucaozuo_yonghu::jiefeng(&id).await {
                Some(_) => jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong_wushuju("解封成功"), miyao),
                None => jiamishibai(500, "解封失败", miyao),
            }
        }
        Caozuoleixing::Xinzeng { zhanghao, mima, nicheng, yonghuzuid, beizhu } => {
            if shujucaozuo_yonghu::zhanghaocunzai(&zhanghao).await {
                return jiamishibai(400, "该账号已存在", miyao);
            }
            match shujucaozuo_yonghu::xinzeng(&zhanghao, &mima, &nicheng, &yonghuzuid, beizhu.as_deref()).await {
                Some(new_id) => jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("新增用户成功", serde_json::json!({ "id": new_id })), miyao),
                None => jiamishibai(500, "新增用户失败", miyao),
            }
        }
        Caozuoleixing::Shanchu(id) => {
            match shujucaozuo_yonghu::chaxun_id(&id).await {
                None => return jiamishibai(404, "用户不存在", miyao),
                Some(yonghu) => {
                    let yonghuzuid = yonghu.get("yonghuzuid").and_then(|v| v.as_str()).unwrap_or("");
                    if yonghuyanzheng::shifouroot(yonghuzuid).await {
                        return jiamishibai(403, "root用户组不可被删除", miyao);
                    }
                }
            }
            match shujucaozuo_yonghu::shanchu(&id).await {
                Some(_) => {
                    let _ = jwtgongju::zhuxiao(&id).await;
                    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong_wushuju("删除用户成功"), miyao)
                }
                None => jiamishibai(500, "删除用户失败", miyao),
            }
        }
        Caozuoleixing::Yonghuzufenye { dangqianyeshu, meiyeshuliang } => {
            let pianyi = ((dangqianyeshu - 1) * meiyeshuliang).to_string();
            let shuliang = meiyeshuliang.to_string();
            let liebiao = match shujucaozuo_yonghuzu::chaxun_fenye(&pianyi, &shuliang).await {
                Some(l) => l,
                None => return jiamishibai(500, "查询用户组列表失败", miyao),
            };
            let zongshu_jieguo = match shujucaozuo_yonghuzu::chaxun_zongshu().await {
                Some(j) => j,
                None => return jiamishibai(500, "查询用户组总数失败", miyao),
            };
            let xiangying = Xiangyingti { liebiao, zongshu: tiquzongshu(&zongshu_jieguo) };
            jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("查询成功", xiangying), miyao)
        }
        Caozuoleixing::Yonghuzusousuo { guanjianci, dangqianyeshu, meiyeshuliang } => {
            let pianyi = ((dangqianyeshu - 1) * meiyeshuliang).to_string();
            let shuliang = meiyeshuliang.to_string();
            let liebiao = match shujucaozuo_yonghuzu::sousuo_mohu(&guanjianci, &pianyi, &shuliang).await {
                Some(l) => l,
                None => return jiamishibai(500, "搜索用户组失败", miyao),
            };
            let zongshu_jieguo = match shujucaozuo_yonghuzu::sousuo_zongshu(&guanjianci).await {
                Some(j) => j,
                None => return jiamishibai(500, "查询搜索总数失败", miyao),
            };
            let xiangying = Xiangyingti { liebiao, zongshu: tiquzongshu(&zongshu_jieguo) };
            jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("搜索成功", xiangying), miyao)
        }
        Caozuoleixing::Yonghuzuxiangqing(id) => {
            match shujucaozuo_yonghuzu::chaxun_id(&id).await {
                Some(zu) => jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("查询成功", zu), miyao),
                None => jiamishibai(404, "用户组不存在", miyao),
            }
        }
        Caozuoleixing::Yonghuzuxinzeng { mingcheng, beizhu, jichengyonghuzuid } => {
            println!("[用户组继承] 收到继承参数: {:?}", jichengyonghuzuid);
            if shujucaozuo_yonghuzu::mingchengcunzai(&mingcheng).await {
                return jiamishibai(400, "该组名称已存在", miyao);
            }
            let xin_id = match shujucaozuo_yonghuzu::xinzeng(&mingcheng, beizhu.as_deref()).await {
                Some(id) => id,
                None => return jiamishibai(500, "新增用户组失败", miyao),
            };
            if let Some(fuid) = jichengyonghuzuid.as_deref() {
                if !fuid.is_empty() {
                    println!("[用户组继承] 查询父组: {}", fuid);
                    if let Some(fuzu) = shujucaozuo_yonghuzu::chaxun_id(fuid).await {
                        let jinjiekou_str = fuzu.get("jinjiekou").and_then(|v| v.as_str()).unwrap_or("[]");
                        println!("[用户组继承] 父组禁用接口: {}", jinjiekou_str);
                        let jieguo = shujucaozuo_yonghuzu::gengxinjinjiekou(&xin_id, jinjiekou_str).await;
                        println!("[用户组继承] 写入结果: {:?}", jieguo);
                    } else {
                        println!("[用户组继承] 未找到父组: {}", fuid);
                    }
                }
            }
            jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("新增用户组成功", serde_json::json!({ "id": xin_id })), miyao)
        }
        Caozuoleixing::Yonghuzuxiugai { id, mingcheng, beizhu } => {
            let beizhu_zhi = beizhu.unwrap_or_default();
            match shujucaozuo_yonghuzu::gengxin(&id, &[("mingcheng", &mingcheng), ("beizhu", &beizhu_zhi)]).await {
                Some(_) => jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong_wushuju("用户组修改成功"), miyao),
                None => jiamishibai(500, "用户组修改失败", miyao),
            }
        }
        Caozuoleixing::Yonghuzushanchu(id) => {
            let zu = match shujucaozuo_yonghuzu::chaxun_id(&id).await {
                Some(z) => z,
                None => return jiamishibai(404, "用户组不存在", miyao),
            };
            if zu.get("mingcheng").and_then(|v| v.as_str()).is_some_and(|m| m == "root") {
                return jiamishibai(403, "root用户组不可删除", miyao);
            }
            let yonghushu = shujucaozuo_yonghuzu::yonghushuliang(&id).await.map(|v| tiquzongshu(&v)).unwrap_or(0);
            if yonghushu > 0 {
                return jiamishibai(400, format!("该用户组下还有{}个用户，无法删除", yonghushu), miyao);
            }
            match shujucaozuo_yonghuzu::shanchu(&id).await {
                Some(_) => jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong_wushuju("删除用户组成功"), miyao),
                None => jiamishibai(500, "删除用户组失败", miyao),
            }
        }
        Caozuoleixing::Yonghuzujiekouliebiao => {
            match shujucaozuo_jiekoujilubiao::chaxun_quanbu().await {
                Some(liebiao) => jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("查询成功", liebiao), miyao),
                None => jiamishibai(500, "查询接口列表失败", miyao),
            }
        }
        Caozuoleixing::Yonghuzuhuoqujinjiekou(id) => {
            let zu = match shujucaozuo_yonghuzu::chaxun_id(&id).await {
                Some(z) => z,
                None => return jiamishibai(404, "用户组不存在", miyao),
            };
            let jinjiekou: Vec<String> = zu.get("jinjiekou")
                .and_then(|v| v.as_str())
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or_default();
            jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("查询成功", jinjiekou), miyao)
        }
        Caozuoleixing::Yonghuzugengxinjinjiekou { id, jinjiekou } => {
            let zu = match shujucaozuo_yonghuzu::chaxun_id(&id).await {
                Some(z) => z,
                None => return jiamishibai(404, "用户组不存在", miyao),
            };
            if zu.get("mingcheng").and_then(|v| v.as_str()).is_some_and(|m| m == "root") {
                return jiamishibai(403, "root用户组不可修改接口权限", miyao);
            }
            let jinjiekou_json = match serde_json::to_string(&jinjiekou) {
                Ok(j) => j,
                Err(_) => return jiamishibai(400, "禁用接口列表格式错误", miyao),
            };
            match shujucaozuo_yonghuzu::gengxinjinjiekou(&id, &jinjiekou_json).await {
                Some(_) => {
                    let _ = rediscaozuo::shanchu(&format!("yonghuzu:jinjiekou:{}", id)).await;
                    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong_wushuju("接口权限更新成功"), miyao)
                }
                None => jiamishibai(500, "接口权限更新失败", miyao),
            }
        }
    }
}

pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let miyao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };
    let mingwen = match jiamichuanshuzhongjian::jiemiqingqiuti(&ti, &miyao) {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(400, "解密请求体失败"),
    };
    let qingqiu = match serde_json::from_slice::<Qingqiuti>(&mingwen) {
        Ok(q) => q,
        Err(_) => return jiamishibai(400, "请求参数格式错误", &miyao),
    };
    let caozuo = match jiexi_caozuo(qingqiu, &miyao) {
        Ok(c) => c,
        Err(e) => return e,
    };
    zhixing_caozuo(caozuo, &miyao).await
}
