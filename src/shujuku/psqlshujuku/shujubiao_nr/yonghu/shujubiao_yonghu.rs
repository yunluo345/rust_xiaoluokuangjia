#[allow(non_upper_case_globals)]

use super::super::super::psqlshujukuzhuti::{Shujubiaodinyi, Ziduandinyi};

pub struct Yonghu;

#[allow(non_upper_case_globals)]
const ziduanlie: &[Ziduandinyi] = &[
    Ziduandinyi { mingcheng: "id", nicheng: "用户ID", jieshao: "用户唯一标识", leixing: "BIGSERIAL PRIMARY KEY", morenzhi: None },
    Ziduandinyi { mingcheng: "zhanghao", nicheng: "账号", jieshao: "登录账号，唯一", leixing: "TEXT NOT NULL UNIQUE", morenzhi: None },
    Ziduandinyi { mingcheng: "mima", nicheng: "密码", jieshao: "登录密码，明文存储", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "nicheng", nicheng: "昵称", jieshao: "用户显示名称", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "touxiang", nicheng: "头像", jieshao: "头像地址", leixing: "TEXT", morenzhi: None },
    Ziduandinyi { mingcheng: "youxiang", nicheng: "邮箱", jieshao: "联系邮箱", leixing: "TEXT", morenzhi: None },
    Ziduandinyi { mingcheng: "yonghuzuid", nicheng: "用户组ID", jieshao: "关联的用户组", leixing: "BIGINT NOT NULL REFERENCES yonghuzu(id) ON DELETE RESTRICT", morenzhi: None },
    Ziduandinyi { mingcheng: "fengjin", nicheng: "封禁状态", jieshao: "是否被封禁，1封禁0正常", leixing: "TEXT NOT NULL", morenzhi: Some("0") },
    Ziduandinyi { mingcheng: "fengjinyuanyin", nicheng: "封禁原因", jieshao: "封禁的具体原因说明", leixing: "TEXT", morenzhi: None },
    Ziduandinyi { mingcheng: "fengjinjieshu", nicheng: "封禁结束时间", jieshao: "封禁到期的时间戳，为空表示永久封禁", leixing: "TEXT", morenzhi: None },
    Ziduandinyi { mingcheng: "kuozhan", nicheng: "扩展信息", jieshao: "扩展JSON字段，用于存储额外信息", leixing: "TEXT NOT NULL", morenzhi: Some("{}") },
    Ziduandinyi { mingcheng: "zuihoudenglu", nicheng: "最后登录", jieshao: "最后一次登录时间戳", leixing: "TEXT", morenzhi: None },
    Ziduandinyi { mingcheng: "beizhu", nicheng: "备注", jieshao: "补充说明", leixing: "TEXT", morenzhi: None },
    Ziduandinyi { mingcheng: "chuangjianshijian", nicheng: "创建时间", jieshao: "记录创建时间", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "gengxinshijian", nicheng: "更新时间", jieshao: "记录最后更新时间", leixing: "TEXT NOT NULL", morenzhi: None },
];

impl Shujubiaodinyi for Yonghu {
    fn biaoming() -> &'static str { "yonghu" }
    fn biaonicheng() -> &'static str { "用户表" }
    fn biaojieshao() -> &'static str { "管理系统用户的账号信息，通过用户组ID关联权限控制" }
    fn ziduanlie() -> &'static [Ziduandinyi] { ziduanlie }
}
