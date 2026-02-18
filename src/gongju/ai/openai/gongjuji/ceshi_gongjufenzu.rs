use super::*;

/// 测试工具分组和关键词功能
pub fn ceshi_gongjufenzu() {
    println!("=== 工具分组和关键词测试 ===");
    
    // 测试获取所有工具信息
    let suoyou_xinxi = huoqu_suoyougongjuxinxi();
    println!("所有工具数量: {}", suoyou_xinxi.len());
    
    for xinxi in &suoyou_xinxi {
        println!("工具: {} | 分组: {:?} | 关键词: {:?}", 
                 xinxi.mingcheng, xinxi.fenzu, xinxi.guanjianci);
    }
    
    // 测试按分组获取工具
    println!("\n=== 管理组工具 ===");
    let guanli_gongju = huoqu_fenzu_gongju(Gongjufenzu::Guanli);
    for gongju in &guanli_gongju {
        println!("管理组工具: {}", gongju.function.name);
    }
    
    println!("\n=== 系统组工具 ===");
    let xitong_gongju = huoqu_fenzu_gongju(Gongjufenzu::Xitong);
    for gongju in &xitong_gongju {
        println!("系统组工具: {}", gongju.function.name);
    }
    
    // 测试关键词匹配
    println!("\n=== 关键词匹配测试 ===");
    let ceshi_guanjianci = vec!["时间", "渠道", "AI", "管理"];
    
    for ci in ceshi_guanjianci {
        let pipei_gongju = huoqu_guanjianci_gongju(ci);
        println!("关键词 '{}' 匹配到 {} 个工具:", ci, pipei_gongju.len());
        for gongju in pipei_gongju {
            println!("  - {}", gongju.function.name);
        }
    }
    
    // 测试获取特定工具信息
    println!("\n=== 工具信息查询测试 ===");
    if let Some(xinxi) = huoqu_gongju_xinxi("shijian_chaxun") {
        println!("时间查询工具信息: {:?}", xinxi);
    }
    
    if let Some(xinxi) = huoqu_gongju_xinxi("aiqudao_guanli") {
        println!("AI渠道管理工具信息: {:?}", xinxi);
    }
}

/// 测试索引功能
pub fn ceshi_suoyin() {
    println!("\n=== 索引功能测试 ===");
    
    // 创建索引管理器
    let suoyin = chuangjian_suoyin();
    println!("索引管理器创建成功");
    
    // 测试关键词查询工具名称
    println!("\n--- 关键词查询工具名称 ---");
    let ceshi_ci = vec!["时间", "渠道", "channel", "time"];
    for ci in ceshi_ci {
        let gongjuming = suoyin.chaxun_gongjuming(ci);
        println!("关键词 '{}' 对应工具: {:?}", ci, gongjuming);
    }
    
    // 测试工具名称查询关键词
    println!("\n--- 工具名称查询关键词 ---");
    let gongjuming_lie = vec!["shijian_chaxun", "aiqudao_guanli"];
    for ming in gongjuming_lie {
        let guanjianci = suoyin.chaxun_guanjianci(ming);
        println!("工具 '{}' 的关键词: {:?}", ming, guanjianci);
    }
    
    // 测试模糊匹配
    println!("\n--- 模糊匹配测试 ---");
    let mohu_ci = vec!["查询", "管理", "配置", "query"];
    for ci in mohu_ci {
        let gongjuming = suoyin.mohu_pipei(ci);
        println!("模糊匹配 '{}' 找到工具: {:?}", ci, gongjuming);
    }
    
    // 测试索引快速获取工具
    println!("\n--- 索引快速获取工具 ---");
    let gongju = suoyin_huoqu_gongju("渠道");
    println!("通过索引获取 '渠道' 相关工具数量: {}", gongju.len());
    for g in gongju {
        println!("  - {}: {}", g.function.name, g.function.description);
    }
    
    // 测试模糊获取工具
    println!("\n--- 模糊获取工具 ---");
    let gongju = mohu_huoqu_gongju("时间查询");
    println!("模糊匹配 '时间查询' 获取工具数量: {}", gongju.len());
    for g in gongju {
        println!("  - {}: {}", g.function.name, g.function.description);
    }
    
    // 测试关键词映射表
    println!("\n--- 关键词映射表 ---");
    let yingshe = huoqu_guanjianci_yingshe();
    for (gongjuming, guanjianci) in yingshe {
        println!("工具 '{}' 有 {} 个关键词", gongjuming, guanjianci.len());
    }
    
    // 测试分组映射表
    println!("\n--- 分组映射表 ---");
    let fenzu_yingshe = huoqu_fenzu_yingshe();
    for (fenzu, gongjulie) in fenzu_yingshe {
        println!("分组 '{}' 包含 {} 个工具: {:?}", fenzu, gongjulie.len(), gongjulie);
    }
}

/// 运行所有测试
pub fn yunxing_suoyou_ceshi() {
    ceshi_gongjufenzu();
    ceshi_suoyin();
    ceshi_trie_suoyin();
    println!("\n=== 所有测试完成 ===");
}

/// 测试 Trie 树智能索引功能
pub fn ceshi_trie_suoyin() {
    println!("\n=== Trie 树智能索引测试 ===");
    
    // 测试智能提取关键词
    println!("\n--- 智能提取关键词测试 ---");
    let ceshi_shuru = vec![
        "我想查询当前时间",
        "帮我管理AI渠道",
        "查询时间和管理渠道",
        "现在几点了",
        "新增一个渠道配置",
        "AI配置管理",
    ];
    
    for shuru in ceshi_shuru {
        let jieguo = zhineng_tiqu_gongjuming(shuru);
        println!("输入: '{}'", shuru);
        println!("  匹配结果:");
        for (gongjuming, defen) in jieguo {
            println!("    - {} (得分: {})", gongjuming, defen);
        }
    }
    
    // 测试智能提取工具
    println!("\n--- 智能提取工具测试 ---");
    let shuru = "查询时间和管理AI渠道配置";
    let gongju = zhineng_tiqu_gongju(shuru);
    println!("输入: '{}'", shuru);
    println!("提取到 {} 个工具:", gongju.len());
    for g in gongju {
        println!("  - {}: {}", g.function.name, g.function.description);
    }
    
    // 测试 Trie 树详细匹配
    println!("\n--- Trie 树详细匹配测试 ---");
    let peiqi = chuangjian_trie_suoyin();
    let shuru = "我要查询当前服务器时间和管理AI渠道";
    let xiangxi_jieguo = peiqi.tiqu_guanjianci(shuru);
    println!("输入: '{}'", shuru);
    println!("提取到的关键词:");
    for (guanjianci, gongjuming_lie) in xiangxi_jieguo {
        println!("  关键词 '{}' -> 工具: {:?}", guanjianci, gongjuming_lie);
    }
}