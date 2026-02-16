use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::{shujucaozuo_yonghu, shujucaozuo_yonghuzu, yonghuyanzheng};
use crate::shujuku::redisshujuku::rediscaozuo;

pub async fn yunxingceshi() {
    println!("\n========== 开始权限验证测试 ==========");
    
    // 测试1: 创建测试用户组
    println!("\n[测试1] 创建测试用户组...");
    let ceshi_zu_id = match shujucaozuo_yonghuzu::xinzeng("测试权限组", Some("用于测试权限验证")).await {
        Some(id) => {
            println!("✓ 创建成功，用户组ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            return;
        }
    };
    
    // 测试2: 设置禁用接口列表
    println!("\n[测试2] 设置禁用接口列表...");
    let jinjiekou = r#"["/jiekou/yonghu/shanchu", "/jiekou/xitong/guanli"]"#;
    match shujucaozuo_yonghuzu::gengxinjinjiekou(&ceshi_zu_id, jinjiekou).await {
        Some(_) => println!("✓ 设置成功"),
        None => {
            println!("✗ 设置失败");
            return;
        }
    };
    
    // 测试3: 创建测试用户
    println!("\n[测试3] 创建测试用户...");
    let ceshi_yonghu_id = match shujucaozuo_yonghu::xinzeng(
        "ceshiyonghu",
        "123456",
        "测试用户",
        &ceshi_zu_id,
        None
    ).await {
        Some(id) => {
            println!("✓ 创建成功，用户ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            shujucaozuo_yonghuzu::shanchu(&ceshi_zu_id).await;
            return;
        }
    };
    
    // 测试4: 检查接口权限（应该被禁止）
    println!("\n[测试4] 检查禁用接口权限...");
    match yonghuyanzheng::jianchajiekouquanxian(&ceshi_zu_id, "/jiekou/yonghu/shanchu").await {
        Ok(_) => println!("✗ 应该被禁止但通过了"),
        Err(yonghuyanzheng::Lingpaicuowu::Quanxianbuzu) => println!("✓ 正确拦截禁用接口"),
        Err(e) => println!("✗ 错误类型不匹配: {:?}", e),
    };
    
    // 测试5: 检查允许的接口（应该通过）
    println!("\n[测试5] 检查允许的接口权限...");
    match yonghuyanzheng::jianchajiekouquanxian(&ceshi_zu_id, "/jiekou/yonghu/chaxun").await {
        Ok(_) => println!("✓ 正确放行允许的接口"),
        Err(e) => println!("✗ 不应该被拦截: {:?}", e),
    };
    
    // 测试6: 验证Redis缓存
    println!("\n[测试6] 验证Redis缓存...");
    let redis_jian = format!("yonghuzu:jinjiekou:{}", ceshi_zu_id);
    match rediscaozuo::huoqu::<String>(&redis_jian).await {
        Some(huancun) => {
            println!("✓ Redis缓存存在: {}", huancun);
            match serde_json::from_str::<Vec<String>>(&huancun) {
                Ok(liebiao) => println!("✓ 缓存解析成功，禁用接口数量: {}", liebiao.len()),
                Err(_) => println!("✗ 缓存解析失败"),
            }
        }
        None => println!("✗ Redis缓存不存在"),
    };
    
    // 测试7: 清理测试数据
    println!("\n[测试7] 清理测试数据...");
    shujucaozuo_yonghu::shanchu(&ceshi_yonghu_id).await;
    shujucaozuo_yonghuzu::shanchu(&ceshi_zu_id).await;
    rediscaozuo::shanchu(&redis_jian).await;
    println!("✓ 清理完成");
    
    println!("\n========== 权限验证测试完成 ==========\n");
}