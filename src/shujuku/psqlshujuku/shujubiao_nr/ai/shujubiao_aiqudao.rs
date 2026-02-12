#[allow(non_upper_case_globals)]

use super::super::super::psqlshujukuzhuti::{Shujubiaodinyi, Ziduandinyi};

pub struct Aiqudao;

#[allow(non_upper_case_globals)]
const ziduanlie: &[Ziduandinyi] = &[
    Ziduandinyi { mingcheng: "id", nicheng: "渠道ID", jieshao: "渠道唯一标识", leixing: "BIGSERIAL PRIMARY KEY", morenzhi: None },
    Ziduandinyi { mingcheng: "mingcheng", nicheng: "渠道名称", jieshao: "显示名称，如OpenAI、通义千问", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "leixing", nicheng: "渠道类型", jieshao: "用于代码中区分调用逻辑，如openai、claude、zhipu", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "jiekoudizhi", nicheng: "接口地址", jieshao: "API基础地址", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "miyao", nicheng: "密钥", jieshao: "API认证密钥", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "moxing", nicheng: "默认模型", jieshao: "该渠道默认使用的模型名", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "wendu", nicheng: "温度", jieshao: "生成随机性控制，0.0到2.0", leixing: "TEXT NOT NULL", morenzhi: Some("0") },
    Ziduandinyi { mingcheng: "zhuangtai", nicheng: "状态", jieshao: "启用或禁用，1启用0禁用", leixing: "TEXT NOT NULL", morenzhi: Some("1") },
    Ziduandinyi { mingcheng: "youxianji", nicheng: "优先级", jieshao: "多渠道调度顺序，数值越小优先级越高", leixing: "INTEGER NOT NULL", morenzhi: Some("0") },
    Ziduandinyi { mingcheng: "beizhu", nicheng: "备注", jieshao: "补充说明", leixing: "TEXT", morenzhi: None },
    Ziduandinyi { mingcheng: "chuangjianshijian", nicheng: "创建时间", jieshao: "记录创建时间", leixing: "TEXT NOT NULL", morenzhi: None },
    Ziduandinyi { mingcheng: "gengxinshijian", nicheng: "更新时间", jieshao: "记录最后更新时间", leixing: "TEXT NOT NULL", morenzhi: None },
];

impl Shujubiaodinyi for Aiqudao {
    fn biaoming() -> &'static str { "aiqudao" }
    fn biaonicheng() -> &'static str { "AI渠道表" }
    fn biaojieshao() -> &'static str { "管理AI服务提供商的接口配置，包括地址、密钥、模型等信息" }
    fn ziduanlie() -> &'static [Ziduandinyi] { ziduanlie }
}
