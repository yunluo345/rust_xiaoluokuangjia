pub mod ai;
pub mod shujubiao_shujubiaojilubiao;
pub mod yonghu;

use super::psqlshujukuzhuti::{Biaozhucexinxi, Shujubiaodinyi};
use shujubiao_shujubiaojilubiao::Shujubiaojilubiao;
use ai::shujubiao_aiqudao::Aiqudao;
use yonghu::shujubiao_yonghu::Yonghu;
use yonghu::shujubiao_yonghuzu::Yonghuzu;

/// 获取所有需要注册的表信息
pub fn huoqubiaolie() -> Vec<Biaozhucexinxi> {
    vec![
        Biaozhucexinxi { biaoming: Shujubiaojilubiao::biaoming(), biaonicheng: Shujubiaojilubiao::biaonicheng(), biaojieshao: Shujubiaojilubiao::biaojieshao(), ziduanlie: Shujubiaojilubiao::ziduanlie() },
        Biaozhucexinxi { biaoming: Aiqudao::biaoming(), biaonicheng: Aiqudao::biaonicheng(), biaojieshao: Aiqudao::biaojieshao(), ziduanlie: Aiqudao::ziduanlie() },
        Biaozhucexinxi { biaoming: Yonghuzu::biaoming(), biaonicheng: Yonghuzu::biaonicheng(), biaojieshao: Yonghuzu::biaojieshao(), ziduanlie: Yonghuzu::ziduanlie() },
        Biaozhucexinxi { biaoming: Yonghu::biaoming(), biaonicheng: Yonghu::biaonicheng(), biaojieshao: Yonghu::biaojieshao(), ziduanlie: Yonghu::ziduanlie() },
    ]
}
