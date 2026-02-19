#[allow(non_upper_case_globals)]

use super::super::super::psqlshujukuzhuti::{Shujubiaodinyi, Ziduandinyi};

pub struct Biaoqian;

#[allow(non_upper_case_globals)]
const ziduanlie: &[Ziduandinyi] = &[
    Ziduandinyi { mingcheng: "id", nicheng: "标签ID", jieshao: "标签唯一标识", leixing: "BIGSERIAL PRIMARY KEY", morenzhi: None },
    Ziduandinyi { mingcheng: "leixingid", nicheng: "类型ID", jieshao: "所属的标签类型ID", leixing: "BIGINT NOT NULL REFERENCES biaoqianleixing(id) ON DELETE CASCADE", morenzhi: None },
    Ziduandinyi { mingcheng: "zhi", nicheng: "标签值", jieshao: "标签的具体值，如广东、青岛", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "chuangjianshijian", nicheng: "创建时间", jieshao: "记录创建时间", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "gengxinshijian", nicheng: "更新时间", jieshao: "记录最后更新时间", leixing: "TEXT NOT NULL", morenzhi: None },
];

impl Shujubiaodinyi for Biaoqian {
    fn biaoming() -> &'static str { "biaoqian" }
    fn biaonicheng() -> &'static str { "标签表" }
    fn biaojieshao() -> &'static str { "存储具体的标签值，归属于某个标签类型" }
    fn ziduanlie() -> &'static [Ziduandinyi] { ziduanlie }
}
