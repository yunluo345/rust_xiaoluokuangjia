use super::{shujucaozuo_ribao, shujucaozuo_biaoqianleixing, shujucaozuo_biaoqian, shujucaozuo_ribao_biaoqian};
use crate::gongju::jichugongju;

pub async fn yunxingceshi() {
    println!("\n========== 开始日报模块测试 ==========");
    
    // ==================== 标签类型测试 ====================
    println!("\n【标签类型测试】");
    
    // 测试1: 新增标签类型
    println!("\n[测试1] 新增标签类型...");
    let leixing1_id = match shujucaozuo_biaoqianleixing::xinzeng("地名").await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            return;
        }
    };
    
    let leixing2_id = match shujucaozuo_biaoqianleixing::xinzeng("项目名").await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            shujucaozuo_biaoqianleixing::shanchu(&leixing1_id).await;
            return;
        }
    };
    
    // 测试2: 根据ID查询标签类型
    println!("\n[测试2] 根据ID查询标签类型...");
    match shujucaozuo_biaoqianleixing::chaxun_id(&leixing1_id).await {
        Some(leixing) => {
            let mingcheng = leixing.get("mingcheng").and_then(|v| v.as_str()).unwrap_or("");
            println!("✓ 查询成功，名称: {}", mingcheng);
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试3: 根据名称查询标签类型
    println!("\n[测试3] 根据名称查询标签类型...");
    match shujucaozuo_biaoqianleixing::chaxun_mingcheng("地名").await {
        Some(leixing) => {
            let id = leixing.get("id").and_then(|v| v.as_str()).unwrap_or("");
            println!("✓ 查询成功，ID: {}", id);
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试4: 查询所有标签类型
    println!("\n[测试4] 查询所有标签类型...");
    match shujucaozuo_biaoqianleixing::chaxun_quanbu().await {
        Some(liebiao) => println!("✓ 查询成功，共 {} 个类型", liebiao.len()),
        None => println!("✗ 查询失败"),
    };
    
    // 测试5: 更新标签类型
    println!("\n[测试5] 更新标签类型...");
    match shujucaozuo_biaoqianleixing::gengxin(&leixing1_id, "地名-已修改").await {
        Some(n) if n > 0 => println!("✓ 更新成功"),
        _ => println!("✗ 更新失败"),
    };
    
    // 测试6: 检查类型名称是否存在
    println!("\n[测试6] 检查类型名称是否存在...");
    if shujucaozuo_biaoqianleixing::mingchengcunzai("地名-已修改").await {
        println!("✓ 名称存在检查正确");
    } else {
        println!("✗ 名称存在检查失败");
    }
    
    // ==================== 标签测试 ====================
    println!("\n【标签测试】");
    
    // 测试7: 新增标签
    println!("\n[测试7] 新增标签...");
    let biaoqian1_id = match shujucaozuo_biaoqian::xinzeng(&leixing1_id, "广东").await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            shujucaozuo_biaoqianleixing::shanchu(&leixing1_id).await;
            shujucaozuo_biaoqianleixing::shanchu(&leixing2_id).await;
            return;
        }
    };
    
    let biaoqian2_id = match shujucaozuo_biaoqian::xinzeng(&leixing1_id, "青岛").await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            shujucaozuo_biaoqian::shanchu(&biaoqian1_id).await;
            shujucaozuo_biaoqianleixing::shanchu(&leixing1_id).await;
            shujucaozuo_biaoqianleixing::shanchu(&leixing2_id).await;
            return;
        }
    };
    
    let biaoqian3_id = match shujucaozuo_biaoqian::xinzeng(&leixing2_id, "项目A").await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            shujucaozuo_biaoqian::shanchu(&biaoqian1_id).await;
            shujucaozuo_biaoqian::shanchu(&biaoqian2_id).await;
            shujucaozuo_biaoqianleixing::shanchu(&leixing1_id).await;
            shujucaozuo_biaoqianleixing::shanchu(&leixing2_id).await;
            return;
        }
    };
    
    // 测试8: 根据ID查询标签
    println!("\n[测试8] 根据ID查询标签...");
    match shujucaozuo_biaoqian::chaxun_id(&biaoqian1_id).await {
        Some(biaoqian) => {
            let zhi = biaoqian.get("zhi").and_then(|v| v.as_str()).unwrap_or("");
            println!("✓ 查询成功，值: {}", zhi);
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试9: 根据类型ID查询标签列表
    println!("\n[测试9] 根据类型ID查询标签列表...");
    match shujucaozuo_biaoqian::chaxun_leixingid(&leixing1_id).await {
        Some(liebiao) => println!("✓ 查询成功，共 {} 个标签", liebiao.len()),
        None => println!("✗ 查询失败"),
    };
    
    // 测试10: 根据类型ID和值查询标签
    println!("\n[测试10] 根据类型ID和值查询标签...");
    match shujucaozuo_biaoqian::chaxun_leixingid_zhi(&leixing1_id, "广东").await {
        Some(biaoqian) => {
            let id = biaoqian.get("id").and_then(|v| v.as_str()).unwrap_or("");
            println!("✓ 查询成功，ID: {}", id);
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试11: 更新标签值
    println!("\n[测试11] 更新标签值...");
    match shujucaozuo_biaoqian::gengxin(&biaoqian1_id, "广东-已修改").await {
        Some(n) if n > 0 => println!("✓ 更新成功"),
        _ => println!("✗ 更新失败"),
    };
    
    // 测试12: 检查标签值是否存在
    println!("\n[测试12] 检查标签值是否存在...");
    if shujucaozuo_biaoqian::zhicunzai(&leixing1_id, "广东-已修改").await {
        println!("✓ 值存在检查正确");
    } else {
        println!("✗ 值存在检查失败");
    }
    
    // ==================== 日报测试 ====================
    println!("\n【日报测试】");
    
    // 测试13: 新增日报
    println!("\n[测试13] 新增日报...");
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let ribao1_id = match shujucaozuo_ribao::xinzeng("1", "今天完成了项目A的开发工作", &shijian).await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            shujucaozuo_biaoqian::shanchu(&biaoqian1_id).await;
            shujucaozuo_biaoqian::shanchu(&biaoqian2_id).await;
            shujucaozuo_biaoqian::shanchu(&biaoqian3_id).await;
            shujucaozuo_biaoqianleixing::shanchu(&leixing1_id).await;
            shujucaozuo_biaoqianleixing::shanchu(&leixing2_id).await;
            return;
        }
    };
    
    let ribao2_id = match shujucaozuo_ribao::xinzeng("1", "今天在青岛参加了会议", &shijian).await {
        Some(id) => {
            println!("✓ 创建成功，ID: {}", id);
            id
        }
        None => {
            println!("✗ 创建失败");
            shujucaozuo_ribao::shanchu(&ribao1_id).await;
            shujucaozuo_biaoqian::shanchu(&biaoqian1_id).await;
            shujucaozuo_biaoqian::shanchu(&biaoqian2_id).await;
            shujucaozuo_biaoqian::shanchu(&biaoqian3_id).await;
            shujucaozuo_biaoqianleixing::shanchu(&leixing1_id).await;
            shujucaozuo_biaoqianleixing::shanchu(&leixing2_id).await;
            return;
        }
    };
    
    // 测试14: 根据ID查询日报
    println!("\n[测试14] 根据ID查询日报...");
    match shujucaozuo_ribao::chaxun_id(&ribao1_id).await {
        Some(ribao) => {
            let neirong = ribao.get("neirong").and_then(|v| v.as_str()).unwrap_or("");
            println!("✓ 查询成功，内容: {}", neirong);
        }
        None => println!("✗ 查询失败"),
    };
    
    // 测试15: 根据用户ID查询日报列表
    println!("\n[测试15] 根据用户ID查询日报列表...");
    match shujucaozuo_ribao::chaxun_yonghuid("1").await {
        Some(liebiao) => println!("✓ 查询成功，共 {} 条日报", liebiao.len()),
        None => println!("✗ 查询失败"),
    };
    
    // 测试16: 查询所有日报
    println!("\n[测试16] 查询所有日报...");
    match shujucaozuo_ribao::chaxun_quanbu().await {
        Some(liebiao) => println!("✓ 查询成功，共 {} 条日报", liebiao.len()),
        None => println!("✗ 查询失败"),
    };
    
    // 测试17: 更新日报内容
    println!("\n[测试17] 更新日报内容...");
    match shujucaozuo_ribao::gengxin(&ribao1_id, &[("neirong", "今天完成了项目A的开发工作-已修改")]).await {
        Some(n) if n > 0 => println!("✓ 更新成功"),
        _ => println!("✗ 更新失败"),
    };
    
    // 测试18: 分页查询日报
    println!("\n[测试18] 分页查询日报...");
    match shujucaozuo_ribao::chaxun_fenye(1, 10).await {
        Some(liebiao) => println!("✓ 查询成功，共 {} 条日报", liebiao.len()),
        None => println!("✗ 查询失败"),
    };
    
    // 测试19: 统计日报总数
    println!("\n[测试19] 统计日报总数...");
    match shujucaozuo_ribao::tongji_zongshu().await {
        Some(zongshu) => println!("✓ 统计成功，总数: {}", zongshu),
        None => println!("✗ 统计失败"),
    };
    
    // 测试20: 统计用户日报总数
    println!("\n[测试20] 统计用户日报总数...");
    match shujucaozuo_ribao::tongji_yonghuid_zongshu("1").await {
        Some(zongshu) => println!("✓ 统计成功，总数: {}", zongshu),
        None => println!("✗ 统计失败"),
    };
    
    // ==================== 日报标签关联测试 ====================
    println!("\n【日报标签关联测试】");
    
    // 测试21: 新增日报标签关联
    println!("\n[测试21] 新增日报标签关联...");
    match shujucaozuo_ribao_biaoqian::xinzeng(&ribao1_id, &biaoqian1_id).await {
        Some(n) if n > 0 => println!("✓ 创建成功"),
        _ => println!("✗ 创建失败"),
    };
    
    match shujucaozuo_ribao_biaoqian::xinzeng(&ribao1_id, &biaoqian3_id).await {
        Some(n) if n > 0 => println!("✓ 创建成功"),
        _ => println!("✗ 创建失败"),
    };
    
    match shujucaozuo_ribao_biaoqian::xinzeng(&ribao2_id, &biaoqian2_id).await {
        Some(n) if n > 0 => println!("✓ 创建成功"),
        _ => println!("✗ 创建失败"),
    };
    
    // 测试22: 查询日报的所有标签
    println!("\n[测试22] 查询日报的所有标签...");
    match shujucaozuo_ribao_biaoqian::chaxun_ribaoid(&ribao1_id).await {
        Some(liebiao) => println!("✓ 查询成功，共 {} 个标签", liebiao.len()),
        None => println!("✗ 查询失败"),
    };
    
    // 测试23: 查询标签关联的所有日报
    println!("\n[测试23] 查询标签关联的所有日报...");
    match shujucaozuo_ribao_biaoqian::chaxun_biaoqianid(&biaoqian1_id).await {
        Some(liebiao) => println!("✓ 查询成功，共 {} 条日报", liebiao.len()),
        None => println!("✗ 查询失败"),
    };
    
    // 测试24: 检查关联是否存在
    println!("\n[测试24] 检查关联是否存在...");
    if shujucaozuo_ribao_biaoqian::guanliancunzai(&ribao1_id, &biaoqian1_id).await {
        println!("✓ 关联存在检查正确");
    } else {
        println!("✗ 关联存在检查失败");
    }
    
    // 测试25: 批量新增日报标签关联
    println!("\n[测试25] 批量新增日报标签关联...");
    match shujucaozuo_ribao_biaoqian::piliang_xinzeng(&ribao2_id, &[&biaoqian1_id, &biaoqian3_id]).await {
        Some(n) => println!("✓ 批量创建成功，共 {} 条", n),
        None => println!("✗ 批量创建失败"),
    };
    
    // 测试26: 删除特定的日报标签关联
    println!("\n[测试26] 删除特定的日报标签关联...");
    match shujucaozuo_ribao_biaoqian::shanchu_guanlian(&ribao1_id, &biaoqian1_id).await {
        Some(n) if n > 0 => println!("✓ 删除成功"),
        _ => println!("✗ 删除失败"),
    };
    
    // ==================== 清理测试数据 ====================
    println!("\n【清理测试数据】");
    shujucaozuo_ribao_biaoqian::shanchu_ribaoid(&ribao1_id).await;
    shujucaozuo_ribao_biaoqian::shanchu_ribaoid(&ribao2_id).await;
    shujucaozuo_ribao::shanchu(&ribao1_id).await;
    shujucaozuo_ribao::shanchu(&ribao2_id).await;
    shujucaozuo_biaoqian::shanchu(&biaoqian1_id).await;
    shujucaozuo_biaoqian::shanchu(&biaoqian2_id).await;
    shujucaozuo_biaoqian::shanchu(&biaoqian3_id).await;
    shujucaozuo_biaoqianleixing::shanchu(&leixing1_id).await;
    shujucaozuo_biaoqianleixing::shanchu(&leixing2_id).await;
    println!("✓ 清理完成");
    
    println!("\n========== 日报模块测试完成 ==========\n");
}
