use qdrant_client::Qdrant;
use qdrant_client::Payload;
use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder};
use serde_json::Value;

#[allow(non_upper_case_globals)]
const fenpiandaxiao: usize = 100;

fn goujianshujudian(id: u64, xianliang: Vec<f32>, fuzai: Value) -> Option<PointStruct> {
    Some(PointStruct::new(
        id,
        xianliang,
        Payload::try_from(fuzai).ok()?,
    ))
}

/// 插入或更新单条数据
pub async fn charu(
    kehu: &Qdrant,
    mingcheng: &str,
    id: u64,
    xianliang: Vec<f32>,
    fuzai: Value,
) -> bool {
    let Some(shujudian) = goujianshujudian(id, xianliang, fuzai) else {
        return false;
    };
    kehu.upsert_points(
        UpsertPointsBuilder::new(mingcheng, vec![shujudian]).wait(true),
    )
    .await
    .is_ok()
}

/// 批量插入或更新数据
pub async fn piliangcharu(
    kehu: &Qdrant,
    mingcheng: &str,
    shujulie: Vec<(u64, Vec<f32>, Value)>,
) -> bool {
    let shujudianlie: Vec<PointStruct> = shujulie
        .into_iter()
        .filter_map(|(id, xianliang, fuzai)| goujianshujudian(id, xianliang, fuzai))
        .collect();

    if shujudianlie.is_empty() {
        return false;
    }

    kehu.upsert_points_chunked(
        UpsertPointsBuilder::new(mingcheng, shujudianlie).wait(true),
        fenpiandaxiao,
    )
    .await
    .is_ok()
}
