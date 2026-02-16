use super::{shujucaozuo_yonghu, shujucaozuo_yonghuzu, yonghuyanzheng};
use crate::gongju::jichugongju;

pub async fn yunxingceshi() {
    println!("\n========== 开始用户模块测试 ==========");
    
    // ==================== 用户组测试 ====================
    println!("\n【用户组测试】");
    
    // 测试1: 新增用户组
    println!("\n[测试1] 新增用户组...");
    let zu1_id = match shujucaozuo_yonghuzu::xinzeng("测试组1", Some("第一个测试组")).await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            return;
        }
    };
    
    let zu2_id = match shujucaozuo_yonghuzu::xinzeng("测试组2", None).await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            shujucaozuo_yonghuzu::shanchu(&zu1_id).await;
            return;
        }
    };
    
    // 测试2: 根据ID查询用户组
    println!("\n[测试2] 根据ID查询用户组...");
    match shujucaozuo_yonghuzu::chaxun_id(&zu1_id).await {
        Some(zu) => {
            let mingcheng = zu.get("mingcheng").and_then(|v| v.as_str()).unwrap_or("");
            println!("✓ 查询成功，名称: {}", mingcheng);
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试3: 查询所有用户组
    println!("\n[测试3] 查询所有用户组...");
    match shujucaozuo_yonghuzu::chaxun_quanbu().await {
        Some(liebiao) => println!("✓ 查询成功，共 {} 个用户组", liebiao.len()),
        None => println!("✗ 查询失败"),
    };
    
    // 测试4: 更新用户组信息
    println!("\n[测试4] 更新用户组信息...");
    match shujucaozuo_yonghuzu::gengxin(&zu1_id, &[("mingcheng", "测试组1-已修改")]).await {
        Some(n) if n > 0 => println!("✓ 更新成功"),
        _ => println!("✗ 更新失败"),
    };
    
    // 测试5: 检查组名称是否存在
    println!("\n[测试5] 检查组名称是否存在...");
    if shujucaozuo_yonghuzu::mingchengcunzai("测试组1-已修改").await {
        println!("✓ 名称存在检查正确");
    } else {
        println!("✗ 名称存在检查失败");
    }
    
    // 测试6: 根据名称查询用户组
    println!("\n[测试6] 根据名称查询用户组...");
    match shujucaozuo_yonghuzu::chaxun_mingcheng("测试组2").await {
        Some(zu) => {
            let id = zu.get("id").and_then(|v| v.as_str()).unwrap_or("");
            println!("✓ 查询成功，ID: {}", id);
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试7: 设置默认用户组
    println!("\n[测试7] 设置默认用户组...");
    match shujucaozuo_yonghuzu::shezhimorenzhu(&zu1_id).await {
        Some(n) if n > 0 => println!("✓ 设置成功"),
        _ => println!("✗ 设置失败"),
    };
    
    // 测试8: 查询默认用户组
    println!("\n[测试8] 查询默认用户组...");
    match shujucaozuo_yonghuzu::chaxunmorenzhu().await {
        Some(zu) => {
            let id = zu.get("id").and_then(|v| v.as_str()).unwrap_or("");
            println!("✓ 查询成功，默认组ID: {}", id);
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试9: 更新禁用接口列表
    println!("\n[测试9] 更新禁用接口列表...");
    let jinjiekou = r#"["/test/api1", "/test/api2"]"#;
    match shujucaozuo_yonghuzu::gengxinjinjiekou(&zu1_id, jinjiekou).await {
        Some(n) if n > 0 => println!("✓ 更新成功"),
        _ => println!("✗ 更新失败"),
    };
    
    // ==================== 用户测试 ====================
    println!("\n【用户测试】");
    
    // 测试10: 新增用户
    println!("\n[测试10] 新增用户...");
    let yonghu1_id = match shujucaozuo_yonghu::xinzeng(
        "ceshizhanghao1",
        "mima123456",
        "测试用户1",
        &zu1_id,
        Some("第一个测试用户")
    ).await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            shujucaozuo_yonghuzu::shanchu(&zu1_id).await;
            shujucaozuo_yonghuzu::shanchu(&zu2_id).await;
            return;
        }
    };
    
    let yonghu2_id = match shujucaozuo_yonghu::xinzeng(
        "ceshizhanghao2",
        "mima654321",
        "测试用户2",
        &zu2_id,
        None
    ).await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            shujucaozuo_yonghu::shanchu(&yonghu1_id).await;
            shujucaozuo_yonghuzu::shanchu(&zu1_id).await;
            shujucaozuo_yonghuzu::shanchu(&zu2_id).await;
            return;
        }
    };
    
    // 测试11: 根据ID查询用户
    println!("\n[测试11] 根据ID查询用户...");
    match shujucaozuo_yonghu::chaxun_id(&yonghu1_id).await {
        Some(yonghu) => {
            let zhanghao = yonghu.get("zhanghao").and_then(|v| v.as_str()).unwrap_or("");
            println!("✓ 查询成功，账号: {}", zhanghao);
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试12: 根据账号查询用户
    println!("\n[测试12] 根据账号查询用户...");
    match shujucaozuo_yonghu::chaxun_zhanghao("ceshizhanghao1").await {
        Some(yonghu) => {
            let nicheng = yonghu.get("nicheng").and_then(|v| v.as_str()).unwrap_or("");
            println!("✓ 查询成功，昵称: {}", nicheng);
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试13: 查询所有用户
    println!("\n[测试13] 查询所有用户...");
    match shujucaozuo_yonghu::chaxun_quanbu().await {
        Some(liebiao) => println!("✓ 查询成功，共 {} 个用户", liebiao.len()),
        None => println!("✗ 查询失败"),
    };
    
    // 测试14: 根据用户组ID查询用户列表
    println!("\n[测试14] 根据用户组ID查询用户列表...");
    match shujucaozuo_yonghu::chaxun_yonghuzuid(&zu1_id).await {
        Some(liebiao) => println!("✓ 查询成功，该组有 {} 个用户", liebiao.len()),
        None => println!("✗ 查询失败"),
    };
    
    // 测试15: 更新用户信息
    println!("\n[测试15] 更新用户信息...");
    match shujucaozuo_yonghu::gengxin(&yonghu1_id, &[("nicheng", "测试用户1-已修改")]).await {
        Some(n) if n > 0 => println!("✓ 更新成功"),
        _ => println!("✗ 更新失败"),
    };
    
    // 测试16: 检查账号是否存在
    println!("\n[测试16] 检查账号是否存在...");
    if shujucaozuo_yonghu::zhanghaocunzai("ceshizhanghao1").await {
        println!("✓ 账号存在检查正确");
    } else {
        println!("✗ 账号存在检查失败");
    }
    
    // 测试17: 查询用户总数
    println!("\n[测试17] 查询用户总数...");
    match shujucaozuo_yonghu::chaxun_zongshu().await {
        Some(jieguo) => {
            // 尝试多种方式解析 COUNT 结果
            let shuliang = jieguo.get("shuliang")
                .and_then(|v| v.as_i64())
                .or_else(|| jieguo.get("count").and_then(|v| v.as_i64()))
                .or_else(|| jieguo.get("shuliang").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()))
                .unwrap_or(0);
            println!("✓ 查询成功，总数: {} (原始数据: {:?})", shuliang, jieguo);
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试18: 分页查询用户
    println!("\n[测试18] 分页查询用户...");
    match shujucaozuo_yonghu::chaxun_fenye("0", "10").await {
        Some(liebiao) => println!("✓ 查询成功，返回 {} 条记录", liebiao.len()),
        None => println!("✗ 查询失败"),
    };
    
    // 测试19: 更新最后登录时间
    println!("\n[测试19] 更新最后登录时间...");
    match shujucaozuo_yonghu::gengxindenglu(&yonghu1_id).await {
        Some(n) if n > 0 => println!("✓ 更新成功"),
        _ => println!("✗ 更新失败"),
    };
    
    // 测试20: 封禁用户
    println!("\n[测试20] 封禁用户...");
    let jieshu_shijian = (jichugongju::huoqushijianchuo() + 3600).to_string();
    match shujucaozuo_yonghu::fengjin(&yonghu2_id, "测试封禁", Some(&jieshu_shijian)).await {
        Some(n) if n > 0 => println!("✓ 封禁成功"),
        _ => println!("✗ 封禁失败"),
    };
    
    // 测试21: 解封用户
    println!("\n[测试21] 解封用户...");
    match shujucaozuo_yonghu::jiefeng(&yonghu2_id).await {
        Some(n) if n > 0 => println!("✓ 解封成功"),
        _ => println!("✗ 解封失败"),
    };
    
    // 测试22: 查询用户组下的用户数量
    println!("\n[测试22] 查询用户组下的用户数量...");
    match shujucaozuo_yonghuzu::yonghushuliang(&zu1_id).await {
        Some(jieguo) => {
            let shuliang = jieguo.get("shuliang").and_then(|v| v.as_i64()).unwrap_or(0);
            println!("✓ 查询成功，该组有 {} 个用户", shuliang);
        }
        None => println!("✗ 查询失败"),
    };
    
    // ==================== 用户验证测试 ====================
    println!("\n【用户验证测试】");
    
    // 测试23: 登录验证（成功）
    println!("\n[测试23] 登录验证（成功）...");
    let lingpai = match yonghuyanzheng::denglu("ceshizhanghao1", "mima123456").await {
        Ok(jieguo) => {
            println!("✓ 登录成功，用户ID: {}", jieguo.yonghuid);
            jieguo.lingpai
        }
        Err(yonghuyanzheng::Denglucuowu::Zhanghaomimacuowu) => {
            println!("✗ 账号或密码错误");
            String::new()
        }
        Err(yonghuyanzheng::Denglucuowu::Yibeifengjin(yuanyin)) => {
            println!("✗ 账号已被封禁: {}", yuanyin);
            String::new()
        }
        Err(yonghuyanzheng::Denglucuowu::Lingpaishibai) => {
            println!("✗ 令牌签发失败");
            String::new()
        }
    };
    
    // 测试24: 登录验证（失败-密码错误）
    println!("\n[测试24] 登录验证（失败-密码错误）...");
    match yonghuyanzheng::denglu("ceshizhanghao1", "cuowumima").await {
        Ok(_) => println!("✗ 不应该登录成功"),
        Err(yonghuyanzheng::Denglucuowu::Zhanghaomimacuowu) => println!("✓ 正确拦截错误密码"),
        Err(_) => println!("✗ 错误类型不匹配"),
    };
    
    // 测试25: 验证令牌
    println!("\n[测试25] 验证令牌...");
    if !lingpai.is_empty() {
        match yonghuyanzheng::yanzhenglingpai(&lingpai).await {
            Ok(zaiti) => println!("✓ 令牌验证成功，用户ID: {}", zaiti.yonghuid),
            Err(_) => println!("✗ 令牌验证失败"),
        };
    } else {
        println!("⊘ 跳过（无有效令牌）");
    }
    
    // 测试26: 检查接口权限（禁用接口）
    println!("\n[测试26] 检查接口权限（禁用接口）...");
    match yonghuyanzheng::jianchajiekouquanxian(&zu1_id, "/test/api1").await {
        Ok(_) => println!("✗ 应该被禁止但通过了"),
        Err(yonghuyanzheng::Lingpaicuowu::Quanxianbuzu) => println!("✓ 正确拦截禁用接口"),
        Err(_) => println!("✗ 错误类型不匹配"),
    };
    
    // 测试27: 检查接口权限（允许接口）
    println!("\n[测试27] 检查接口权限（允许接口）...");
    match yonghuyanzheng::jianchajiekouquanxian(&zu1_id, "/test/allowed").await {
        Ok(_) => println!("✓ 正确放行允许的接口"),
        Err(_) => println!("✗ 不应该被拦截"),
    };
    
    // 测试28: 验证令牌并检查权限
    println!("\n[测试28] 验证令牌并检查权限...");
    if !lingpai.is_empty() {
        match yonghuyanzheng::yanzhenglingpaijiquanxian(&lingpai, "/test/allowed").await {
            Ok(zaiti) => println!("✓ 验证成功，用户ID: {}", zaiti.yonghuid),
            Err(_) => println!("✗ 验证失败"),
        };
    } else {
        println!("⊘ 跳过（无有效令牌）");
    }
    
    // 测试29: 封禁用户后登录验证
    println!("\n[测试29] 封禁用户后登录验证...");
    let jieshu_shijian = (jichugongju::huoqushijianchuo() + 3600).to_string();
    shujucaozuo_yonghu::fengjin(&yonghu1_id, "测试封禁验证", Some(&jieshu_shijian)).await;
    match yonghuyanzheng::denglu("ceshizhanghao1", "mima123456").await {
        Ok(_) => println!("✗ 封禁用户不应该登录成功"),
        Err(yonghuyanzheng::Denglucuowu::Yibeifengjin(yuanyin)) => {
            println!("✓ 正确拦截封禁用户，原因: {}", yuanyin);
        }
        Err(_) => println!("✗ 错误类型不匹配"),
    };
    shujucaozuo_yonghu::jiefeng(&yonghu1_id).await;
    
    // ==================== 清理测试数据 ====================
    println!("\n【清理测试数据】");
    shujucaozuo_yonghu::shanchu(&yonghu1_id).await;
    shujucaozuo_yonghu::shanchu(&yonghu2_id).await;
    shujucaozuo_yonghuzu::shanchu(&zu1_id).await;
    shujucaozuo_yonghuzu::shanchu(&zu2_id).await;
    println!("✓ 清理完成");
    
    println!("\n========== 用户模块测试完成 ==========\n");
}
