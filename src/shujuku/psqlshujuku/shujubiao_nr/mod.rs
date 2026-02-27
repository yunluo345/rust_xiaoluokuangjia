pub mod ai;
pub mod ribao;
pub mod shujubiao_jiekoujilubiao;
pub mod shujubiao_shujubiaojilubiao;
pub mod shujucaozuo_jiekoujilubiao;
pub mod yonghu;

use super::psqlshujukuzhuti::{Biaozhucexinxi, Shujubiaodinyi};
use shujubiao_jiekoujilubiao::Jiekoujilubiao;
use shujubiao_shujubiaojilubiao::Shujubiaojilubiao;
use ai::shujubiao_aiqudao::Aiqudao;
use ribao::shujubiao_ribao::Ribao;
use ribao::shujubiao_biaoqianleixing::Biaoqianleixing;
use ribao::shujubiao_biaoqian::Biaoqian;
use ribao::shujubiao_ribao_biaoqian::Ribaobiaoqian;
use ribao::shujubiao_ribao_biaoqianrenwu::Ribaobiaoqianrenwu;
use ribao::shujubiao_ribao_guanxi::Ribaoguanxi;
use yonghu::shujubiao_yonghu::Yonghu;
use yonghu::shujubiao_yonghuzu::Yonghuzu;

/// 获取所有需要注册的表信息
pub fn huoqubiaolie() -> Vec<Biaozhucexinxi> {
    vec![
        Biaozhucexinxi { biaoming: Shujubiaojilubiao::biaoming(), biaonicheng: Shujubiaojilubiao::biaonicheng(), biaojieshao: Shujubiaojilubiao::biaojieshao(), ziduanlie: Shujubiaojilubiao::ziduanlie() },
        Biaozhucexinxi { biaoming: Jiekoujilubiao::biaoming(), biaonicheng: Jiekoujilubiao::biaonicheng(), biaojieshao: Jiekoujilubiao::biaojieshao(), ziduanlie: Jiekoujilubiao::ziduanlie() },
        Biaozhucexinxi { biaoming: Aiqudao::biaoming(), biaonicheng: Aiqudao::biaonicheng(), biaojieshao: Aiqudao::biaojieshao(), ziduanlie: Aiqudao::ziduanlie() },
        Biaozhucexinxi { biaoming: Yonghuzu::biaoming(), biaonicheng: Yonghuzu::biaonicheng(), biaojieshao: Yonghuzu::biaojieshao(), ziduanlie: Yonghuzu::ziduanlie() },
        Biaozhucexinxi { biaoming: Yonghu::biaoming(), biaonicheng: Yonghu::biaonicheng(), biaojieshao: Yonghu::biaojieshao(), ziduanlie: Yonghu::ziduanlie() },
        Biaozhucexinxi { biaoming: Biaoqianleixing::biaoming(), biaonicheng: Biaoqianleixing::biaonicheng(), biaojieshao: Biaoqianleixing::biaojieshao(), ziduanlie: Biaoqianleixing::ziduanlie() },
        Biaozhucexinxi { biaoming: Biaoqian::biaoming(), biaonicheng: Biaoqian::biaonicheng(), biaojieshao: Biaoqian::biaojieshao(), ziduanlie: Biaoqian::ziduanlie() },
        Biaozhucexinxi { biaoming: Ribao::biaoming(), biaonicheng: Ribao::biaonicheng(), biaojieshao: Ribao::biaojieshao(), ziduanlie: Ribao::ziduanlie() },
        Biaozhucexinxi { biaoming: Ribaobiaoqian::biaoming(), biaonicheng: Ribaobiaoqian::biaonicheng(), biaojieshao: Ribaobiaoqian::biaojieshao(), ziduanlie: Ribaobiaoqian::ziduanlie() },
        Biaozhucexinxi { biaoming: Ribaobiaoqianrenwu::biaoming(), biaonicheng: Ribaobiaoqianrenwu::biaonicheng(), biaojieshao: Ribaobiaoqianrenwu::biaojieshao(), ziduanlie: Ribaobiaoqianrenwu::ziduanlie() },
        Biaozhucexinxi { biaoming: Ribaoguanxi::biaoming(), biaonicheng: Ribaoguanxi::biaonicheng(), biaojieshao: Ribaoguanxi::biaojieshao(), ziduanlie: Ribaoguanxi::ziduanlie() },
    ]
}
