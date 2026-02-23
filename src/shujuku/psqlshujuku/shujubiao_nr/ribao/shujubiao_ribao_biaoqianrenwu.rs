#[allow(non_upper_case_globals)]

use super::super::super::psqlshujukuzhuti::{Shujubiaodinyi, Ziduandinyi};

pub struct Ribaobiaoqianrenwu;

#[allow(non_upper_case_globals)]
const ziduanlie: &[Ziduandinyi] = &[
    Ziduandinyi { mingcheng: "id", nicheng: "任务ID", jieshao: "队列任务唯一标识", leixing: "BIGSERIAL PRIMARY KEY", morenzhi: None },
    Ziduandinyi { mingcheng: "ribaoid", nicheng: "日报ID", jieshao: "关联的日报ID", leixing: "BIGINT NOT NULL UNIQUE REFERENCES ribao(id) ON DELETE CASCADE", morenzhi: None },
    Ziduandinyi { mingcheng: "yonghuid", nicheng: "用户ID", jieshao: "关联的用户ID", leixing: "BIGINT NOT NULL REFERENCES yonghu(id) ON DELETE CASCADE", morenzhi: None },
    Ziduandinyi { mingcheng: "zhuangtai", nicheng: "任务状态", jieshao: "任务状态，true已完成，processing处理中，false待处理", leixing: "TEXT NOT NULL CHECK (zhuangtai IN ('true','false','processing'))", morenzhi: Some("false") },
    Ziduandinyi { mingcheng: "changshicishu", nicheng: "已尝试次数", jieshao: "当前已执行次数", leixing: "INT NOT NULL CHECK (changshicishu >= 0)", morenzhi: Some("0") },
    Ziduandinyi { mingcheng: "zuidachangshicishu", nicheng: "最大尝试次数", jieshao: "达到后不再自动重试", leixing: "INT NOT NULL CHECK (zuidachangshicishu >= 0)", morenzhi: Some("3") },
    Ziduandinyi { mingcheng: "biaoqianjieguo", nicheng: "标签结果", jieshao: "AI产出的标签结果JSON字符串", leixing: "TEXT", morenzhi: None },
    Ziduandinyi { mingcheng: "chuangjianshijian", nicheng: "创建时间", jieshao: "记录创建时间", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "gengxinshijian", nicheng: "更新时间", jieshao: "记录最后更新时间", leixing: "TEXT NOT NULL", morenzhi: None },
];

impl Shujubiaodinyi for Ribaobiaoqianrenwu {
    fn biaoming() -> &'static str { "ribao_biaoqianrenwu" }
    fn biaonicheng() -> &'static str { "日报标签任务队列表" }
    fn biaojieshao() -> &'static str { "存储日报AI打标签任务，支持排队与重试" }
    fn ziduanlie() -> &'static [Ziduandinyi] { ziduanlie }
}
