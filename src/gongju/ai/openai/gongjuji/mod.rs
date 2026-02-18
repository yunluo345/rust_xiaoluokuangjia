use std::future::Future;
use std::pin::Pin;

mod gongju_shijianchaxun;
pub mod gongju_aiqudaoguanli;

use llm::chat::Tool;

/// 工具注册项：定义 + 执行函数
struct Gongjuzhuce {
    dinyi: Tool,
    zhixing: fn(&str, &str) -> Pin<Box<dyn Future<Output = String> + Send + 'static>>,
}

/// 所有已注册的工具列表
fn suoyouzhuce() -> Vec<Gongjuzhuce> {
    vec![
        Gongjuzhuce {
            dinyi: gongju_shijianchaxun::dinyi(),
            zhixing: gongju_shijianchaxun_wrapper,
        },
        Gongjuzhuce {
            dinyi: gongju_aiqudaoguanli::dinyi(),
            zhixing: gongju_aiqudaoguanli_wrapper,
        },
    ]
}

/// 包装函数，解决生命周期问题
fn gongju_shijianchaxun_wrapper(canshu: &str, lingpai: &str) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
    let canshu = canshu.to_string();
    let lingpai = lingpai.to_string();
    Box::pin(async move {
        gongju_shijianchaxun::zhixing(&canshu, &lingpai).await
    })
}

/// AI渠道管理工具包装函数
fn gongju_aiqudaoguanli_wrapper(canshu: &str, lingpai: &str) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
    let canshu = canshu.to_string();
    let lingpai = lingpai.to_string();
    Box::pin(async move {
        gongju_aiqudaoguanli::zhixing(&canshu, &lingpai).await
    })
}

/// 获取所有工具定义，供 Xiaoxiguanli 注册
pub fn huoqu_suoyougongju() -> Vec<Tool> {
    suoyouzhuce().into_iter().map(|z| z.dinyi).collect()
}

/// 按工具名称分发执行，返回结果字符串
pub async fn zhixing(gongjuming: &str, canshu: &str, lingpai: &str) -> String {
    for zhuce in suoyouzhuce() {
        if zhuce.dinyi.function.name == gongjuming {
            return (zhuce.zhixing)(canshu, lingpai).await;
        }
    }
    format!("未知工具: {}", gongjuming)
}
