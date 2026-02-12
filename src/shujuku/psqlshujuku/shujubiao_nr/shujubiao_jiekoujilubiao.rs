#[allow(non_upper_case_globals)]

use super::super::psqlshujukuzhuti::{Shujubiaodinyi, Ziduandinyi};

pub struct Jiekoujilubiao;

#[allow(non_upper_case_globals)]
const ziduanlie: &[Ziduandinyi] = &[
    Ziduandinyi { mingcheng: "lujing", nicheng: "路径", jieshao: "接口完整路由路径，唯一标识", leixing: "TEXT PRIMARY KEY", morenzhi: None },
    Ziduandinyi { mingcheng: "nicheng", nicheng: "昵称", jieshao: "接口中文显示名称", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "jieshao", nicheng: "介绍", jieshao: "接口用途说明", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "fangshi", nicheng: "请求方式", jieshao: "GET/POST/SSE", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "jiami", nicheng: "加密", jieshao: "是否为加密接口，1是0否", leixing: "TEXT NOT NULL", morenzhi: Some("0") },
    Ziduandinyi { mingcheng: "xudenglu", nicheng: "需登录", jieshao: "是否需要登录才能访问，1是0否", leixing: "TEXT NOT NULL", morenzhi: Some("0") },
    Ziduandinyi { mingcheng: "xuyonghuzu", nicheng: "需用户组", jieshao: "是否需要特定用户组才能访问，1是0否", leixing: "TEXT NOT NULL", morenzhi: Some("0") },
    Ziduandinyi { mingcheng: "yunxuputong", nicheng: "允许普通用户", jieshao: "普通用户是否可访问，1是0否", leixing: "TEXT NOT NULL", morenzhi: Some("1") },
    Ziduandinyi { mingcheng: "chuangjianshijian", nicheng: "创建时间", jieshao: "记录创建时间", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "gengxinshijian", nicheng: "更新时间", jieshao: "记录最后更新时间", leixing: "TEXT NOT NULL", morenzhi: None },
];

impl Shujubiaodinyi for Jiekoujilubiao {
    fn biaoming() -> &'static str { "jiekoujilubiao" }
    fn biaonicheng() -> &'static str { "接口记录表" }
    fn biaojieshao() -> &'static str { "管理所有接口的元信息，包括路径、权限配置等" }
    fn ziduanlie() -> &'static [Ziduandinyi] { ziduanlie }
}
