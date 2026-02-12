use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    Condition, Filter, SearchPointsBuilder, SearchResponse,
    SearchBatchPointsBuilder, SearchBatchResponse,
    GetPointsBuilder, GetResponse,
    ScrollPointsBuilder, ScrollResponse,
    CountPointsBuilder,
    PointId,
};

fn zhuanhuanidlie(idlie: Vec<u64>) -> Vec<PointId> {
    idlie.into_iter().map(|id| id.into()).collect()
}

/// 向量搜索（可选过滤条件）
pub async fn sousuo(
    kehu: &Qdrant,
    mingcheng: &str,
    xianliang: Vec<f32>,
    xianzhi: u64,
    guolvtiaojian: Option<Filter>,
) -> Option<SearchResponse> {
    let mut goujianqi = SearchPointsBuilder::new(mingcheng, xianliang, xianzhi)
        .with_payload(true);
    if let Some(tiaojian) = guolvtiaojian {
        goujianqi = goujianqi.filter(tiaojian);
    }
    kehu.search_points(goujianqi).await.ok()
}

/// 按字段精确匹配搜索
pub async fn sousuoanziduanpipei(
    kehu: &Qdrant,
    mingcheng: &str,
    xianliang: Vec<f32>,
    xianzhi: u64,
    ziduanming: &str,
    ziduanzhi: &str,
) -> Option<SearchResponse> {
    sousuo(
        kehu, mingcheng, xianliang, xianzhi,
        Some(Filter::must([Condition::matches(ziduanming, ziduanzhi.to_string())])),
    ).await
}

/// 批量向量搜索
pub async fn piliangsousuo(
    kehu: &Qdrant,
    mingcheng: &str,
    sousuolie: Vec<(Vec<f32>, u64)>,
) -> Option<SearchBatchResponse> {
    let qingqiulie: Vec<_> = sousuolie
        .into_iter()
        .map(|(xianliang, xianzhi)| {
            SearchPointsBuilder::new(mingcheng, xianliang, xianzhi)
                .with_payload(true)
                .build()
        })
        .collect();

    kehu.search_batch_points(
        SearchBatchPointsBuilder::new(mingcheng, qingqiulie),
    )
    .await
    .ok()
}

/// 按 ID 列表获取数据点
pub async fn anidhuoqu(
    kehu: &Qdrant,
    mingcheng: &str,
    idlie: Vec<u64>,
) -> Option<GetResponse> {
    kehu.get_points(
        GetPointsBuilder::new(mingcheng, zhuanhuanidlie(idlie))
            .with_payload(true)
            .with_vectors(true),
    )
    .await
    .ok()
}

/// 滚动查询（分页遍历）
pub async fn gundongchaxun(
    kehu: &Qdrant,
    mingcheng: &str,
    xianzhi: u32,
    guolvtiaojian: Option<Filter>,
) -> Option<ScrollResponse> {
    let mut goujianqi = ScrollPointsBuilder::new(mingcheng)
        .limit(xianzhi)
        .with_payload(true)
        .with_vectors(true);
    if let Some(tiaojian) = guolvtiaojian {
        goujianqi = goujianqi.filter(tiaojian);
    }
    kehu.scroll(goujianqi).await.ok()
}

/// 统计数据点数量
pub async fn jishu(
    kehu: &Qdrant,
    mingcheng: &str,
    jingque: bool,
) -> Option<u64> {
    kehu.count(
        CountPointsBuilder::new(mingcheng).exact(jingque),
    )
    .await
    .ok()
    .and_then(|xiangying| xiangying.result.map(|jieguo| jieguo.count))
}
