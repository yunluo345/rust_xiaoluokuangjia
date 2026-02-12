#[allow(non_upper_case_globals)]

use super::super::psqlshujukuzhuti::{Shujubiaodinyi, Ziduandinyi};

pub struct Shujubiaojilubiao;

#[allow(non_upper_case_globals)]
const ziduanlie: &[Ziduandinyi] = &[
    Ziduandinyi { mingcheng: "biaoming", nicheng: "表名", jieshao: "数据库中的实际表名", leixing: "TEXT PRIMARY KEY", morenzhi: None },
    Ziduandinyi { mingcheng: "biaonicheng", nicheng: "表昵称", jieshao: "表的中文显示名称", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "biaojieshao", nicheng: "表介绍", jieshao: "表的用途说明", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "ziduanxinxi", nicheng: "字段信息", jieshao: "所有字段定义的JSON数组", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "chuangjianshijian", nicheng: "创建时间", jieshao: "记录创建时间", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "gengxinshijian", nicheng: "更新时间", jieshao: "记录最后更新时间", leixing: "TEXT NOT NULL", morenzhi: None },
];

impl Shujubiaodinyi for Shujubiaojilubiao {
    fn biaoming() -> &'static str { "shujubiaojilubiao" }
    fn biaonicheng() -> &'static str { "数据表记录表" }
    fn biaojieshao() -> &'static str { "管理所有数据表的元信息，包括表名、字段定义等" }
    fn ziduanlie() -> &'static [Ziduandinyi] { ziduanlie }
}
