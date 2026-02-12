use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    Condition, DeletePointsBuilder, Filter, PointsIdsList,
};

/// 按过滤条件删除数据点
pub async fn antiaojianshnachu(
    kehu: &Qdrant,
    mingcheng: &str,
    guolvtiaojian: Filter,
) -> bool {
    kehu.delete_points(
        DeletePointsBuilder::new(mingcheng)
            .points(guolvtiaojian)
            .wait(true),
    )
    .await
    .is_ok()
}

/// 按 ID 列表删除数据点
pub async fn anidshanchu(
    kehu: &Qdrant,
    mingcheng: &str,
    idlie: Vec<u64>,
) -> bool {
    kehu.delete_points(
        DeletePointsBuilder::new(mingcheng)
            .points(PointsIdsList {
                ids: idlie.into_iter().map(|id| id.into()).collect(),
            })
            .wait(true),
    )
    .await
    .is_ok()
}

/// 按字段精确匹配删除数据点
pub async fn anziduanshanchu(
    kehu: &Qdrant,
    mingcheng: &str,
    ziduanming: &str,
    ziduanzhi: &str,
) -> bool {
    antiaojianshnachu(
        kehu, mingcheng,
        Filter::must([Condition::matches(ziduanming, ziduanzhi.to_string())]),
    ).await
}
