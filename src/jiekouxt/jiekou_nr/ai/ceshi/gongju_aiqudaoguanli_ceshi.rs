use crate::gongju::ai::openai::gongjuji::gongju_aiqudaoguanli;
use crate::gongju::jwtgongju;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao;
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::{shujucaozuo_yonghu, shujucaozuo_yonghuzu};
use serde_json::{json, Value};

pub async fn yunxingceshi() {
    println!("\n========== 开始AI渠道管理工具测试 ==========");
    
    // 准备测试环境：创建测试用户和令牌
    println!("\n[准备] 创建测试用户和令牌...");
    let ceshi_zu_id = match shujucaozuo_yonghuzu::xinzeng("测试工具组", Some("用于测试AI渠道管理工具")).await {
        Some(id) => {
            println!("✓ 创建用户组成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建用户组失败");
            return;
        }
    };
    
    let ceshi_yonghu_id = match shujucaozuo_yonghu::xinzeng(
        "ceshigongju",
        "123456",
        "测试工具用户",
        &ceshi_zu_id,
        None
    ).await {
        Some(id) => {
            println!("✓ 创建用户成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建用户失败");
            shujucaozuo_yonghuzu::shanchu(&ceshi_zu_id).await;
            return;
        }
    };
    
    let lingpai = match jwtgongju::qianfa(&ceshi_yonghu_id, "ceshigongju", &ceshi_zu_id).await {
        Some(token) => {
            println!("✓ 生成令牌成功");
            token
        }
        None => {
            println!("✗ 生成令牌失败");
            shujucaozuo_yonghu::shanchu(&ceshi_yonghu_id).await;
            shujucaozuo_yonghuzu::shanchu(&ceshi_zu_id).await;
            return;
        }
    };
    
    // 测试1: 令牌验证失败
    println!("\n[测试1] 无效令牌验证...");
    let canshu = json!({"caozuo": "chaxun_quanbu"}).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, "invalid_token").await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").is_some() => println!("✓ 正确拦截无效令牌"),
        _ => println!("✗ 应该拦截无效令牌"),
    };
    
    // 测试2: 参数格式错误
    println!("\n[测试2] 参数格式错误...");
    let jieguo = gongju_aiqudaoguanli::zhixing("invalid json", &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").and_then(|e| e.as_str()) == Some("参数格式错误") => {
            println!("✓ 正确识别参数格式错误");
        }
        _ => println!("✗ 应该返回参数格式错误"),
    };
    
    // 测试3: 不支持的操作类型
    println!("\n[测试3] 不支持的操作类型...");
    let canshu = json!({"caozuo": "invalid_operation"}).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").and_then(|e| e.as_str()) == Some("不支持的操作类型") => {
            println!("✓ 正确识别不支持的操作");
        }
        _ => println!("✗ 应该返回不支持的操作类型"),
    };
    
    // 测试4: 查询全部渠道
    println!("\n[测试4] 查询全部渠道...");
    let canshu = json!({"caozuo": "chaxun_quanbu"}).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("chenggong").and_then(|c| c.as_bool()) == Some(true) => {
            println!("✓ 查询全部成功");
        }
        _ => println!("✗ 查询全部失败"),
    };
    
    // 测试5: 查询启用渠道
    println!("\n[测试5] 查询启用渠道...");
    let canshu = json!({"caozuo": "chaxun_qiyong"}).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("chenggong").and_then(|c| c.as_bool()) == Some(true) => {
            println!("✓ 查询启用渠道成功");
        }
        _ => println!("✗ 查询启用渠道失败"),
    };
    
    // 测试6: 新增渠道 - 缺少参数
    println!("\n[测试6] 新增渠道 - 缺少参数...");
    let canshu = json!({"caozuo": "xinzeng"}).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").and_then(|e| e.as_str()).map(|s| s.contains("缺少参数")) == Some(true) => {
            println!("✓ 正确识别缺少参数");
        }
        _ => println!("✗ 应该返回缺少参数错误"),
    };
    
    // 测试7: 新增渠道 - 类型验证失败
    println!("\n[测试7] 新增渠道 - 类型验证失败...");
    let canshu = json!({
        "caozuo": "xinzeng",
        "mingcheng": "测试工具渠道",
        "leixing": "invalid_type",
        "jiekoudizhi": "https://api.test.com",
        "miyao": "test-key",
        "moxing": "test-model",
        "wendu": "0.7"
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").and_then(|e| e.as_str()).map(|s| s.contains("类型只能是")) == Some(true) => {
            println!("✓ 正确验证类型参数");
        }
        _ => println!("✗ 应该返回类型验证错误"),
    };
    
    // 测试8: 新增渠道 - 成功
    println!("\n[测试8] 新增渠道 - 成功...");
    let canshu = json!({
        "caozuo": "xinzeng",
        "mingcheng": "测试工具渠道1",
        "leixing": "openapi",
        "jiekoudizhi": "https://api.test.com",
        "miyao": "test-key-123",
        "moxing": "test-model",
        "wendu": "0.7",
        "beizhu": "工具测试渠道"
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    let ceshi_qudao_id = match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("chenggong").and_then(|c| c.as_bool()) == Some(true) => {
            let id = v.get("shuju").and_then(|s| s.get("id")).and_then(|i| i.as_str()).unwrap_or("");
            println!("✓ 新增渠道成功，ID: {}", id);
            id.to_string()
        }
        _ => {
            println!("✗ 新增渠道失败");
            String::new()
        }
    };
    
    if ceshi_qudao_id.is_empty() {
        println!("\n[清理] 清理测试数据...");
        shujucaozuo_yonghu::shanchu(&ceshi_yonghu_id).await;
        shujucaozuo_yonghuzu::shanchu(&ceshi_zu_id).await;
        return;
    }
    
    // 测试9: 按ID查询渠道
    println!("\n[测试9] 按ID查询渠道...");
    let canshu = json!({
        "caozuo": "chaxun_id",
        "id": &ceshi_qudao_id
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("chenggong").and_then(|c| c.as_bool()) == Some(true) => {
            let mingcheng = v.get("shuju").and_then(|s| s.get("mingcheng")).and_then(|m| m.as_str()).unwrap_or("");
            println!("✓ 查询成功，名称: {}", mingcheng);
        }
        _ => println!("✗ 查询失败"),
    };
    
    // 测试10: 查询不存在的ID
    println!("\n[测试10] 查询不存在的ID...");
    let canshu = json!({
        "caozuo": "chaxun_id",
        "id": "999999999"
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").and_then(|e| e.as_str()) == Some("渠道不存在") => {
            println!("✓ 正确识别渠道不存在");
        }
        _ => println!("✗ 应该返回渠道不存在"),
    };
    
    // 测试11: 更新渠道 - 缺少字段列表
    println!("\n[测试11] 更新渠道 - 缺少字段列表...");
    let canshu = json!({
        "caozuo": "gengxin",
        "id": &ceshi_qudao_id
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").and_then(|e| e.as_str()).map(|s| s.contains("缺少参数")) == Some(true) => {
            println!("✓ 正确识别缺少字段列表");
        }
        _ => println!("✗ 应该返回缺少参数错误"),
    };
    
    // 测试12: 更新渠道 - 字段列表为空
    println!("\n[测试12] 更新渠道 - 字段列表为空...");
    let canshu = json!({
        "caozuo": "gengxin",
        "id": &ceshi_qudao_id,
        "ziduanlie": []
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").and_then(|e| e.as_str()) == Some("更新字段不能为空") => {
            println!("✓ 正确识别字段列表为空");
        }
        _ => println!("✗ 应该返回字段列表为空错误"),
    };
    
    // 测试13: 更新渠道 - 字段格式错误
    println!("\n[测试13] 更新渠道 - 字段格式错误...");
    let canshu = json!({
        "caozuo": "gengxin",
        "id": &ceshi_qudao_id,
        "ziduanlie": [["mingcheng"]]
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").and_then(|e| e.as_str()).map(|s| s.contains("字段格式错误")) == Some(true) => {
            println!("✓ 正确识别字段格式错误");
        }
        _ => println!("✗ 应该返回字段格式错误"),
    };
    
    // 测试14: 更新渠道 - 类型验证
    println!("\n[测试14] 更新渠道 - 类型验证...");
    let canshu = json!({
        "caozuo": "gengxin",
        "id": &ceshi_qudao_id,
        "ziduanlie": [["leixing", "invalid_type"]]
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").and_then(|e| e.as_str()).map(|s| s.contains("类型只能是")) == Some(true) => {
            println!("✓ 正确验证更新时的类型参数");
        }
        _ => println!("✗ 应该返回类型验证错误"),
    };
    
    // 测试15: 更新渠道 - 成功
    println!("\n[测试15] 更新渠道 - 成功...");
    let canshu = json!({
        "caozuo": "gengxin",
        "id": &ceshi_qudao_id,
        "ziduanlie": [
            ["mingcheng", "测试工具渠道1-已修改"],
            ["moxing", "test-model-v2"]
        ]
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("chenggong").and_then(|c| c.as_bool()) == Some(true) => {
            println!("✓ 更新渠道成功");
        }
        _ => println!("✗ 更新渠道失败"),
    };
    
    // 测试16: 切换渠道状态
    println!("\n[测试16] 切换渠道状态...");
    let canshu = json!({
        "caozuo": "qiehuanzhuangtai",
        "id": &ceshi_qudao_id
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("chenggong").and_then(|c| c.as_bool()) == Some(true) => {
            println!("✓ 切换状态成功");
        }
        _ => println!("✗ 切换状态失败"),
    };
    
    // 测试17: 更新优先级 - 缺少参数
    println!("\n[测试17] 更新优先级 - 缺少参数...");
    let canshu = json!({
        "caozuo": "gengxinyouxianji",
        "id": &ceshi_qudao_id
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").and_then(|e| e.as_str()).map(|s| s.contains("缺少参数")) == Some(true) => {
            println!("✓ 正确识别缺少优先级参数");
        }
        _ => println!("✗ 应该返回缺少参数错误"),
    };
    
    // 测试18: 更新优先级 - 成功
    println!("\n[测试18] 更新优先级 - 成功...");
    let canshu = json!({
        "caozuo": "gengxinyouxianji",
        "id": &ceshi_qudao_id,
        "youxianji": "10"
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("chenggong").and_then(|c| c.as_bool()) == Some(true) => {
            println!("✓ 更新优先级成功");
        }
        _ => println!("✗ 更新优先级失败"),
    };
    
    // 测试19: 删除渠道 - 缺少ID
    println!("\n[测试19] 删除渠道 - 缺少ID...");
    let canshu = json!({"caozuo": "shanchu"}).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").and_then(|e| e.as_str()).map(|s| s.contains("缺少参数")) == Some(true) => {
            println!("✓ 正确识别缺少ID参数");
        }
        _ => println!("✗ 应该返回缺少参数错误"),
    };
    
    // 测试20: 删除渠道 - 成功
    println!("\n[测试20] 删除渠道 - 成功...");
    let canshu = json!({
        "caozuo": "shanchu",
        "id": &ceshi_qudao_id
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("chenggong").and_then(|c| c.as_bool()) == Some(true) => {
            println!("✓ 删除渠道成功");
        }
        _ => println!("✗ 删除渠道失败"),
    };
    
    // 测试21: 删除不存在的渠道
    println!("\n[测试21] 删除不存在的渠道...");
    let canshu = json!({
        "caozuo": "shanchu",
        "id": "999999999"
    }).to_string();
    let jieguo = gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await;
    match serde_json::from_str::<Value>(&jieguo) {
        Ok(v) if v.get("cuowu").and_then(|e| e.as_str()) == Some("渠道不存在") => {
            println!("✓ 正确识别渠道不存在");
        }
        _ => println!("✗ 应该返回渠道不存在，实际返回: {}", jieguo),
    };
    
    // 清理测试数据
    println!("\n[清理] 清理测试数据...");
    shujucaozuo_yonghu::shanchu(&ceshi_yonghu_id).await;
    shujucaozuo_yonghuzu::shanchu(&ceshi_zu_id).await;
    println!("✓ 清理完成");
    
    println!("\n========== AI渠道管理工具测试完成 ==========\n");
}
