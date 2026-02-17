mod gongju_shijianchaxun;

use llm::chat::Tool;

/// 工具注册项：定义 + 执行函数
struct Gongjuzhuce {
    dinyi: Tool,
    zhixing: fn(&str) -> std::pin::Pin<Box<dyn std::future::Future<Output = String> + Send + '_>>,
}

/// 所有已注册的工具列表
fn suoyouzhuce() -> Vec<Gongjuzhuce> {
    vec![
        Gongjuzhuce {
            dinyi: gongju_shijianchaxun::dinyi(),
            zhixing: |canshu| Box::pin(gongju_shijianchaxun::zhixing(canshu)),
        },
    ]
}

/// 获取所有工具定义，供 Xiaoxiguanli 注册
pub fn huoqu_suoyougongju() -> Vec<Tool> {
    suoyouzhuce().into_iter().map(|z| z.dinyi).collect()
}

/// 按工具名称分发执行，返回结果字符串
pub async fn zhixing(gongjuming: &str, canshu: &str) -> String {
    for zhuce in suoyouzhuce() {
        if zhuce.dinyi.function.name == gongjuming {
            return (zhuce.zhixing)(canshu).await;
        }
    }
    format!("未知工具: {}", gongjuming)
}
