#[allow(non_upper_case_globals)]

use super::super::super::psqlshujukuzhuti::{Shujubiaodinyi, Ziduandinyi};

pub struct Ribaobiaoqian;

#[allow(non_upper_case_globals)]
const ziduanlie: &[Ziduandinyi] = &[
    Ziduandinyi { mingcheng: "ribaoid", nicheng: "日报ID", jieshao: "关联的日报ID", leixing: "BIGINT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "biaoqianid", nicheng: "标签ID", jieshao: "关联的标签ID", leixing: "BIGINT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "chuangjianshijian", nicheng: "创建时间", jieshao: "记录创建时间", leixing: "TEXT NOT NULL", morenzhi: None },
];

impl Shujubiaodinyi for Ribaobiaoqian {
    fn biaoming() -> &'static str { "ribao_biaoqian" }
    fn biaonicheng() -> &'static str { "日报标签关联表" }
    fn biaojieshao() -> &'static str { "建立日报与标签的多对多关联关系" }
    fn ziduanlie() -> &'static [Ziduandinyi] { ziduanlie }
}
