#[allow(non_upper_case_globals)]

use super::super::super::psqlshujukuzhuti::{Shujubiaodinyi, Ziduandinyi};

pub struct Yonghuzu;

#[allow(non_upper_case_globals)]
const ziduanlie: &[Ziduandinyi] = &[
    Ziduandinyi { mingcheng: "id", nicheng: "用户组ID", jieshao: "用户组唯一标识", leixing: "BIGSERIAL PRIMARY KEY", morenzhi: None },
    Ziduandinyi { mingcheng: "mingcheng", nicheng: "组名称", jieshao: "用户组显示名称，如管理员、普通用户", leixing: "TEXT NOT NULL UNIQUE", morenzhi: None },
    Ziduandinyi { mingcheng: "jinjiekou", nicheng: "禁用接口", jieshao: "黑名单，不允许访问的接口路径列表，JSON数组格式", leixing: "TEXT NOT NULL", morenzhi: Some("[]") },
    Ziduandinyi { mingcheng: "morenzhu", nicheng: "默认组", jieshao: "是否为默认用户组，1是0否，全局仅一个", leixing: "TEXT NOT NULL", morenzhi: Some("0") },
    Ziduandinyi { mingcheng: "kuozhan", nicheng: "扩展信息", jieshao: "扩展JSON字段，用于存储额外信息", leixing: "TEXT NOT NULL", morenzhi: Some("{}") },
    Ziduandinyi { mingcheng: "beizhu", nicheng: "备注", jieshao: "补充说明", leixing: "TEXT", morenzhi: None },
    Ziduandinyi { mingcheng: "chuangjianshijian", nicheng: "创建时间", jieshao: "记录创建时间", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "gengxinshijian", nicheng: "更新时间", jieshao: "记录最后更新时间", leixing: "TEXT NOT NULL", morenzhi: None },
];

impl Shujubiaodinyi for Yonghuzu {
    fn biaoming() -> &'static str { "yonghuzu" }
    fn biaonicheng() -> &'static str { "用户组表" }
    fn biaojieshao() -> &'static str { "管理用户组的权限配置，通过黑名单控制接口访问权限" }
    fn ziduanlie() -> &'static [Ziduandinyi] { ziduanlie }
}
