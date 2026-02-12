use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, VectorParamsBuilder,
};

/// 创建集合（已存在则跳过）
pub async fn chuangjian(
    kehu: &Qdrant,
    mingcheng: &str,
    weidu: u64,
    julicedufangshi: Distance,
) -> bool {
    match kehu.collection_exists(mingcheng).await {
        Ok(true) => true,
        Ok(false) => kehu
            .create_collection(
                CreateCollectionBuilder::new(mingcheng)
                    .vectors_config(VectorParamsBuilder::new(weidu, julicedufangshi)),
            )
            .await
            .is_ok(),
        Err(_) => false,
    }
}

/// 删除集合
pub async fn shanchu(kehu: &Qdrant, mingcheng: &str) -> bool {
    kehu.delete_collection(mingcheng).await.is_ok()
}

/// 检查集合是否存在
pub async fn cunzai(kehu: &Qdrant, mingcheng: &str) -> bool {
    kehu.collection_exists(mingcheng).await.unwrap_or(false)
}

/// 获取所有集合名称
pub async fn liebiao(kehu: &Qdrant) -> Option<Vec<String>> {
    kehu.list_collections()
        .await
        .ok()
        .map(|xiangying| {
            xiangying.collections.into_iter()
                .map(|jihe| jihe.name)
                .collect()
        })
}
