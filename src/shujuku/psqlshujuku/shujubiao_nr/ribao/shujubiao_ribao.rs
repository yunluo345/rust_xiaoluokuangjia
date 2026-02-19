#[allow(non_upper_case_globals)]

use super::super::super::psqlshujukuzhuti::{Shujubiaodinyi, Ziduandinyi};

pub struct Ribao;

#[allow(non_upper_case_globals)]
const ziduanlie: &[Ziduandinyi] = &[
    Ziduandinyi { mingcheng: "id", nicheng: "日报ID", jieshao: "日报唯一标识", leixing: "BIGSERIAL PRIMARY KEY", morenzhi: None },
    Ziduandinyi { mingcheng: "yonghuid", nicheng: "用户ID", jieshao: "发送日报的用户ID", leixing: "BIGINT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "neirong", nicheng: "日报内容", jieshao: "日报的具体内容", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "fabushijian", nicheng: "发布时间", jieshao: "日报发布的时间戳", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "chuangjianshijian", nicheng: "创建时间", jieshao: "记录创建时间", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "gengxinshijian", nicheng: "更新时间", jieshao: "记录最后更新时间", leixing: "TEXT NOT NULL", morenzhi: None },
];

impl Shujubiaodinyi for Ribao {
    fn biaoming() -> &'static str { "ribao" }
    fn biaonicheng() -> &'static str { "日报表" }
    fn biaojieshao() -> &'static str { "存储用户发送的日报内容和时间信息" }
    fn ziduanlie() -> &'static [Ziduandinyi] { ziduanlie }
}
