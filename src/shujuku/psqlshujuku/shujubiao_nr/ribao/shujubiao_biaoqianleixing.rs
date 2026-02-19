#[allow(non_upper_case_globals)]

use super::super::super::psqlshujukuzhuti::{Shujubiaodinyi, Ziduandinyi};

pub struct Biaoqianleixing;

#[allow(non_upper_case_globals)]
const ziduanlie: &[Ziduandinyi] = &[
    Ziduandinyi { mingcheng: "id", nicheng: "类型ID", jieshao: "标签类型唯一标识", leixing: "BIGSERIAL PRIMARY KEY", morenzhi: None },
    Ziduandinyi { mingcheng: "mingcheng", nicheng: "类型名称", jieshao: "标签类型的名称，如地名", leixing: "TEXT NOT NULL UNIQUE", morenzhi: None },
    Ziduandinyi { mingcheng: "chuangjianshijian", nicheng: "创建时间", jieshao: "记录创建时间", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "gengxinshijian", nicheng: "更新时间", jieshao: "记录最后更新时间", leixing: "TEXT NOT NULL", morenzhi: None },
];

impl Shujubiaodinyi for Biaoqianleixing {
    fn biaoming() -> &'static str { "biaoqianleixing" }
    fn biaonicheng() -> &'static str { "标签类型表" }
    fn biaojieshao() -> &'static str { "定义标签的分类类型，如地名、项目名等" }
    fn ziduanlie() -> &'static [Ziduandinyi] { ziduanlie }
}
