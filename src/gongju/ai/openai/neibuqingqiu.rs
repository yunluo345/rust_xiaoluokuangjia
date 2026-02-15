use super::aipeizhi::Aipeizhi;
use super::aixiaoxiguanli::Xiaoxiguanli;
use super::openaizhuti;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao as qudaocaozuo;

/// 通过渠道轮询获取AI配置
async fn huoqu_peizhi(leixing: &str) -> Option<Aipeizhi> {
    let shuju = qudaocaozuo::lunxun(leixing).await?;
    Aipeizhi::cong_qudaoshuju(&shuju)
}

/// 后端内部非流式AI对话，自动走渠道轮询
pub async fn duihua(leixing: &str, xitongtishici: &str, yonghuxiaoxi: &str) -> Option<String> {
    let peizhi = huoqu_peizhi(leixing).await?;
    let mut guanli = Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(xitongtishici);
    guanli.zhuijia_yonghuxiaoxi(yonghuxiaoxi);
    openaizhuti::putongqingqiu(&peizhi, &mut guanli).await
}

/// 后端内部非流式AI对话，支持多轮消息列表
pub async fn duihua_duolun(leixing: &str, xitongtishici: &str, xiaoxilie: Vec<(&str, &str)>) -> Option<String> {
    let peizhi = huoqu_peizhi(leixing).await?;
    let mut guanli = Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(xitongtishici);
    for (jiaose, neirong) in xiaoxilie {
        match jiaose {
            "yonghu" => guanli.zhuijia_yonghuxiaoxi(neirong),
            "zhushou" => guanli.zhuijia_zhushouneirong(neirong),
            _ => {}
        }
    }
    openaizhuti::putongqingqiu(&peizhi, &mut guanli).await
}
