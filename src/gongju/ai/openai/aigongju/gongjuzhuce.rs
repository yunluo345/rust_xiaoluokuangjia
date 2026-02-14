use llm::chat::Tool;
use std::sync::OnceLock;

/// 工具元数据：每个工具注册时提供的信息
#[derive(Clone)]
pub struct Gongjuyuanshuju {
    /// 工具唯一标识（与Tool.function.name一致）
    pub mingcheng: &'static str,
    /// 工具的自然语言描述（用于搜索匹配）
    pub miaoshu: &'static str,
    /// 所属分组
    pub fenzu: &'static str,
    /// 关键词标签（用于关键词搜索）
    pub guanjianci: &'static [&'static str],
    /// 场景触发词：描述什么场景下应该使用此工具（用于语义匹配）
    pub changjingci: &'static [&'static str],
    /// 构建llm::chat::Tool的函数
    pub goujianqi: fn() -> Tool,
    /// 执行工具的函数
    pub zhixingqi: fn(&str) -> String,
    /// 是否为核心工具（核心工具始终加载，不需要发现）
    pub hexingongju: bool,
}

/// 全局工具注册表
static ZHUCEBIAO: OnceLock<Vec<Gongjuyuanshuju>> = OnceLock::new();

/// 初始化注册表（在应用启动时调用一次）
pub fn chushihua(gongjulie: Vec<Gongjuyuanshuju>) {
    let _ = ZHUCEBIAO.set(gongjulie);
}

/// 获取所有已注册工具
pub fn huoqu_quanbu() -> &'static [Gongjuyuanshuju] {
    ZHUCEBIAO.get().map(|v| v.as_slice()).unwrap_or(&[])
}

/// 获取所有核心工具（始终加载）
pub fn huoqu_hexingongju() -> Vec<&'static Gongjuyuanshuju> {
    huoqu_quanbu().iter().filter(|g| g.hexingongju).collect()
}

/// 获取所有非核心工具（需要发现）
pub fn huoqu_faxiangongju() -> Vec<&'static Gongjuyuanshuju> {
    huoqu_quanbu().iter().filter(|g| !g.hexingongju).collect()
}

/// 按名称查找工具
pub fn anming_chazhao(mingcheng: &str) -> Option<&'static Gongjuyuanshuju> {
    huoqu_quanbu().iter().find(|g| g.mingcheng == mingcheng)
}

/// 按分组查找工具
pub fn anfenzu_chazhao(fenzu: &str) -> Vec<&'static Gongjuyuanshuju> {
    huoqu_quanbu().iter().filter(|g| g.fenzu == fenzu).collect()
}

/// 获取所有分组名称（去重）
pub fn huoqu_suoyoufenzu() -> Vec<&'static str> {
    let mut fenzu: Vec<&str> = huoqu_quanbu().iter().map(|g| g.fenzu).collect();
    fenzu.sort();
    fenzu.dedup();
    fenzu
}

/// 按名称列表批量获取工具
pub fn piliang_chazhao(mingchenglie: &[&str]) -> Vec<&'static Gongjuyuanshuju> {
    huoqu_quanbu()
        .iter()
        .filter(|g| mingchenglie.contains(&g.mingcheng))
        .collect()
}
