#[allow(non_upper_case_globals)]

use super::super::super::psqlshujukuzhuti::{Shujubiaodinyi, Ziduandinyi};

pub struct Ribaoguanxi;

#[allow(non_upper_case_globals)]
const ziduanlie: &[Ziduandinyi] = &[
    Ziduandinyi { mingcheng: "id", nicheng: "关系边ID", jieshao: "关系边唯一标识", leixing: "BIGSERIAL PRIMARY KEY", morenzhi: None },
    Ziduandinyi { mingcheng: "ribaoid", nicheng: "日报ID", jieshao: "来源日报ID", leixing: "BIGINT NOT NULL REFERENCES ribao(id) ON DELETE CASCADE", morenzhi: None },
    Ziduandinyi { mingcheng: "ren1", nicheng: "实体1", jieshao: "关系的第一个实体名称", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "ren2", nicheng: "实体2", jieshao: "关系的第二个实体名称", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "guanxi", nicheng: "关系类型", jieshao: "关系类型名称", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "miaoshu", nicheng: "描述", jieshao: "关系描述", leixing: "TEXT", morenzhi: Some("") },
    Ziduandinyi { mingcheng: "xindu", nicheng: "置信度", jieshao: "置信度数值（文本存储）", leixing: "TEXT", morenzhi: Some("0") },
    Ziduandinyi { mingcheng: "zhengjupianduan", nicheng: "证据片段", jieshao: "原文证据片段", leixing: "TEXT", morenzhi: Some("") },
    Ziduandinyi { mingcheng: "juese_ren1", nicheng: "角色1", jieshao: "实体1在关系中的角色", leixing: "TEXT", morenzhi: Some("") },
    Ziduandinyi { mingcheng: "juese_ren2", nicheng: "角色2", jieshao: "实体2在关系中的角色", leixing: "TEXT", morenzhi: Some("") },
    Ziduandinyi { mingcheng: "qinggan_qingxiang", nicheng: "情感倾向", jieshao: "关系的情感倾向：正面/负面/中性", leixing: "TEXT", morenzhi: Some("") },
    Ziduandinyi { mingcheng: "chuangjianshijian", nicheng: "创建时间", jieshao: "记录创建时间", leixing: "TEXT NOT NULL", morenzhi: None },
];

impl Shujubiaodinyi for Ribaoguanxi {
    fn biaoming() -> &'static str { "ribao_guanxi_bian" }
    fn biaonicheng() -> &'static str { "日报关系边表" }
    fn biaojieshao() -> &'static str { "存储AI关系分析的预计算结果，避免查询时全量扫描ribao.kuozhan" }
    fn ziduanlie() -> &'static [Ziduandinyi] { ziduanlie }
}
