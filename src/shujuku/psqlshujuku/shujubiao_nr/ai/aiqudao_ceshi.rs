use super::shujucaozuo_aiqudao;

pub async fn yunxingceshi() {
    println!("\n========== 开始AI渠道模块测试 ==========");
    
    // 测试1: 新增AI渠道
    println!("\n[测试1] 新增AI渠道...");
    let qudao1_id = match shujucaozuo_aiqudao::xinzeng(
        "测试渠道1",
        "openai",
        "https://api.openai.com/v1",
        "sk-test-key-123456",
        "gpt-4",
        "0.7",
        Some("第一个测试渠道")
    ).await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            return;
        }
    };
    
    let qudao2_id = match shujucaozuo_aiqudao::xinzeng(
        "测试渠道2",
        "claude",
        "https://api.anthropic.com/v1",
        "sk-ant-test-key",
        "claude-3-opus",
        "0.5",
        None
    ).await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            shujucaozuo_aiqudao::shanchu(&qudao1_id).await;
            return;
        }
    };
    
    let qudao3_id = match shujucaozuo_aiqudao::xinzeng(
        "测试渠道3",
        "openai",
        "https://api.openai.com/v1",
        "sk-test-key-789",
        "gpt-3.5-turbo",
        "0.8",
        Some("第三个测试渠道")
    ).await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            shujucaozuo_aiqudao::shanchu(&qudao1_id).await;
            shujucaozuo_aiqudao::shanchu(&qudao2_id).await;
            return;
        }
    };
    
    // 测试2: 根据ID查询渠道
    println!("\n[测试2] 根据ID查询渠道...");
    match shujucaozuo_aiqudao::chaxun_id(&qudao1_id).await {
        Some(qudao) => {
            let mingcheng = qudao.get("mingcheng").and_then(|v| v.as_str()).unwrap_or("");
            let leixing = qudao.get("leixing").and_then(|v| v.as_str()).unwrap_or("");
            println!("✓ 查询成功，名称: {}，类型: {}", mingcheng, leixing);
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试3: 查询所有渠道
    println!("\n[测试3] 查询所有渠道...");
    match shujucaozuo_aiqudao::chaxun_quanbu().await {
        Some(liebiao) => println!("✓ 查询成功，共 {} 个渠道", liebiao.len()),
        None => println!("✗ 查询失败"),
    };
    
    // 测试4: 更新渠道信息
    println!("\n[测试4] 更新渠道信息...");
    match shujucaozuo_aiqudao::gengxin(&qudao1_id, &[
        ("mingcheng", "测试渠道1-已修改"),
        ("moxing", "gpt-4-turbo")
    ]).await {
        Some(n) if n > 0 => println!("✓ 更新成功"),
        _ => println!("✗ 更新失败"),
    };
    
    // 测试5: 检查渠道名称是否存在
    println!("\n[测试5] 检查渠道名称是否存在...");
    if shujucaozuo_aiqudao::mingchengcunzai("测试渠道1-已修改").await {
        println!("✓ 名称存在检查正确");
    } else {
        println!("✗ 名称存在检查失败");
    }
    
    if !shujucaozuo_aiqudao::mingchengcunzai("不存在的渠道").await {
        println!("✓ 名称不存在检查正确");
    } else {
        println!("✗ 名称不存在检查失败");
    }
    
    // 测试6: 查询所有启用的渠道
    println!("\n[测试6] 查询所有启用的渠道...");
    match shujucaozuo_aiqudao::chaxun_qiyong().await {
        Some(liebiao) => {
            println!("✓ 查询成功，共 {} 个启用渠道", liebiao.len());
            for qudao in &liebiao {
                let zhuangtai = qudao.get("zhuangtai").and_then(|v| v.as_str()).unwrap_or("");
                if zhuangtai != "1" {
                    println!("✗ 发现非启用状态的渠道");
                }
            }
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试7: 根据渠道类型查询
    println!("\n[测试7] 根据渠道类型查询...");
    match shujucaozuo_aiqudao::chaxun_leixing("openai").await {
        Some(liebiao) => {
            println!("✓ 查询成功，openai类型有 {} 个渠道", liebiao.len());
            for qudao in &liebiao {
                let leixing = qudao.get("leixing").and_then(|v| v.as_str()).unwrap_or("");
                if leixing != "openai" {
                    println!("✗ 发现非openai类型的渠道");
                }
            }
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试8: 切换渠道状态（禁用）
    println!("\n[测试8] 切换渠道状态（禁用）...");
    match shujucaozuo_aiqudao::qiehuanzhuangtai(&qudao2_id).await {
        Some(n) if n > 0 => {
            println!("✓ 切换成功");
            // 验证状态已改变
            if let Some(qudao) = shujucaozuo_aiqudao::chaxun_id(&qudao2_id).await {
                let zhuangtai = qudao.get("zhuangtai").and_then(|v| v.as_str()).unwrap_or("");
                if zhuangtai == "0" {
                    println!("✓ 状态已变为禁用");
                } else {
                    println!("✗ 状态未正确改变");
                }
            }
        }
        _ => println!("✗ 切换失败"),
    };
    
    // 测试9: 切换渠道状态（重新启用）
    println!("\n[测试9] 切换渠道状态（重新启用）...");
    match shujucaozuo_aiqudao::qiehuanzhuangtai(&qudao2_id).await {
        Some(n) if n > 0 => {
            println!("✓ 切换成功");
            // 验证状态已改变
            if let Some(qudao) = shujucaozuo_aiqudao::chaxun_id(&qudao2_id).await {
                let zhuangtai = qudao.get("zhuangtai").and_then(|v| v.as_str()).unwrap_or("");
                if zhuangtai == "1" {
                    println!("✓ 状态已变为启用");
                } else {
                    println!("✗ 状态未正确改变");
                }
            }
        }
        _ => println!("✗ 切换失败"),
    };
    
    // 测试10: 更新渠道优先级
    println!("\n[测试10] 更新渠道优先级...");
    match shujucaozuo_aiqudao::gengxinyouxianji(&qudao1_id, "10").await {
        Some(n) if n > 0 => {
            println!("✓ 更新成功");
            // 验证优先级已改变
            if let Some(qudao) = shujucaozuo_aiqudao::chaxun_id(&qudao1_id).await {
                let youxianji = qudao.get("youxianji")
                    .and_then(|v| v.as_i64())
                    .or_else(|| qudao.get("youxianji").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()))
                    .unwrap_or(0);
                if youxianji == 10 {
                    println!("✓ 优先级已更新为10");
                } else {
                    println!("✗ 优先级未正确更新，当前值: {}", youxianji);
                }
            }
        }
        _ => println!("✗ 更新失败"),
    };
    
    match shujucaozuo_aiqudao::gengxinyouxianji(&qudao3_id, "5").await {
        Some(n) if n > 0 => println!("✓ 渠道3优先级更新成功"),
        _ => println!("✗ 渠道3优先级更新失败"),
    };
    
    // 测试11: 验证优先级排序
    println!("\n[测试11] 验证优先级排序...");
    match shujucaozuo_aiqudao::chaxun_quanbu().await {
        Some(liebiao) => {
            println!("✓ 查询成功");
            let mut shangyouxianji: Option<i64> = None;
            for qudao in &liebiao {
                let youxianji = qudao.get("youxianji")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<i64>().ok())
                    .unwrap_or(0);
                if let Some(shang) = shangyouxianji {
                    if youxianji < shang {
                        println!("✗ 优先级排序不正确");
                        break;
                    }
                }
                shangyouxianji = Some(youxianji);
            }
            if shangyouxianji.is_some() {
                println!("✓ 优先级排序正确");
            }
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试12: 统计渠道总数
    println!("\n[测试12] 统计渠道总数...");
    match shujucaozuo_aiqudao::tongjishuliang().await {
        Some(jieguo) => {
            let shuliang = jieguo.get("shuliang")
                .and_then(|v| v.as_i64())
                .or_else(|| jieguo.get("shuliang").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()))
                .unwrap_or(0);
            println!("✓ 统计成功，总数: {}", shuliang);
        }
        None => println!("✗ 统计失败"),
    };
    
    // 测试13: 删除渠道
    println!("\n[测试13] 删除渠道...");
    match shujucaozuo_aiqudao::shanchu(&qudao1_id).await {
        Some(n) if n > 0 => println!("✓ 删除渠道1成功"),
        _ => println!("✗ 删除渠道1失败"),
    };
    
    // 验证删除后无法查询
    match shujucaozuo_aiqudao::chaxun_id(&qudao1_id).await {
        None => println!("✓ 删除后无法查询，验证成功"),
        Some(_) => println!("✗ 删除后仍能查询，验证失败"),
    };
    
    // 测试14: 根据类型按优先级随机获取渠道
    println!("\n[测试14] 根据类型按优先级随机获取渠道...");
    shujucaozuo_aiqudao::gengxinyouxianji(&qudao2_id, "5").await;
    match shujucaozuo_aiqudao::suiji_huoqu_qudao("openai").await {
        Some(qudao) => {
            let mingcheng = qudao.get("mingcheng").and_then(|v| v.as_str()).unwrap_or("");
            let miyao = qudao.get("miyao").and_then(|v| v.as_str()).unwrap_or("");
            let moxing = qudao.get("moxing").and_then(|v| v.as_str()).unwrap_or("");
            let jiekoudizhi = qudao.get("jiekoudizhi").and_then(|v| v.as_str()).unwrap_or("");
            let youxianji = qudao.get("youxianji").and_then(|v| v.as_i64()).unwrap_or(-1);
            println!("✓ 获取成功，名称: {}，模型: {}，密钥: {}，地址: {}，优先级: {}", mingcheng, moxing, miyao, jiekoudizhi, youxianji);
        }
        None => println!("✗ 获取失败"),
    };
    match shujucaozuo_aiqudao::suiji_huoqu_qudao("bucunzaileixing").await {
        None => println!("✓ 不存在的类型正确返回空"),
        Some(_) => println!("✗ 不存在的类型不应返回数据"),
    };

    // ==================== 清理测试数据 ====================
    println!("\n【清理测试数据】");
    shujucaozuo_aiqudao::shanchu(&qudao2_id).await;
    shujucaozuo_aiqudao::shanchu(&qudao3_id).await;
    println!("✓ 清理完成");
    
    println!("\n========== AI渠道模块测试完成 ==========\n");
}
