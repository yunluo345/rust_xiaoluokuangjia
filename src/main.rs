#![allow(unused_imports, dead_code)]

mod gongju;
mod jiekouxt;
mod peizhixt;
mod shujuku;
mod yewu;

use peizhixt::peizhi_nr::peizhi_zongpeizhi::Zongpeizhi;
use peizhixt::peizhi_nr::peizhi_shujuku::Shujuku;
use peizhixt::peizhixitongzhuti;
use shujuku::qrshujuku::qrshujukuzhuti::{self, Qrpeizhi};
use shujuku::psqlshujuku::psqlshujukuzhuti::{self, Psqlpeizhi};
use shujuku::psqlshujuku::shujubiao_nr;
use shujuku::redisshujuku::redisshujukuzhuti::{self, Redislianjiepeizhi};
use qdrant_client::qdrant::Distance;
use actix_web::{App, HttpServer};
use actix_cors::Cors;

fn tuichu(xinxi: &str) -> ! {
    eprintln!("{}", xinxi);
    std::process::exit(1);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !peizhixitongzhuti::chushihua() {
        tuichu("配置系统初始化失败");
    }

    // 初始化AI调度器（读取配置中的全局并发上限）
    gongju::ai::openai::diaoduqi::chushihua_cong_peizhi();
    
    let zongpeizhi = peizhixitongzhuti::duqupeizhi::<Zongpeizhi>(
        Zongpeizhi::wenjianming()
    ).unwrap_or_else(|| tuichu("读取总配置失败"));
    
    let shujukupeizhi = peizhixitongzhuti::duqupeizhi::<Shujuku>(
        Shujuku::wenjianming()
    ).unwrap_or_else(|| tuichu("读取数据库配置失败"));
    
    if shujukupeizhi.xiangliangku.qiyong {
        let qrpeizhi = Qrpeizhi {
            zhiji: shujukupeizhi.xiangliangku.zhiji,
            duankou: shujukupeizhi.xiangliangku.grpc_duankou,
            miyao: shujukupeizhi.xiangliangku.miyao,
            jheqianzhui: shujukupeizhi.xiangliangku.jheqianzhui,
        };
        
        if !qrshujukuzhuti::lianjie(&qrpeizhi, "moren", 1536, Distance::Cosine).await {
            tuichu("Qdrant 向量数据库连接失败");
        }
        println!("Qdrant 向量数据库连接成功");
    }
    
    if shujukupeizhi.psql.qiyong {
        let psqlpeizhi = Psqlpeizhi {
            zhiji: shujukupeizhi.psql.zhiji,
            duankou: shujukupeizhi.psql.duankou,
            yonghuming: shujukupeizhi.psql.yonghuming,
            mima: shujukupeizhi.psql.mima,
            shujukuming: shujukupeizhi.psql.shujukuming,
        };

        let biaolie = shujubiao_nr::huoqubiaolie();

        if !psqlshujukuzhuti::lianjie(&psqlpeizhi, &biaolie).await {
            tuichu("PostgreSQL 数据库连接失败");
        }

        let jiekoulie = jiekouxt::jiekou_nr::huoqujiekoulie();
        if !jiekouxt::jiekouxtzhuti::tongbujiekoulie(&jiekoulie).await {
            tuichu("接口记录同步失败");
        }

        if !shujubiao_nr::yonghu::chushihua_yonghu::chushihua().await {
            tuichu("用户系统初始化失败");
        }

        println!("PostgreSQL 数据库连接成功");
    }
    
    let redispeizhi = Redislianjiepeizhi {
        zhujidizhi: shujukupeizhi.redis.zhujidizhi,
        duankou: shujukupeizhi.redis.duankou,
        zhanghao: shujukupeizhi.redis.zhanghao,
        mima: shujukupeizhi.redis.mima,
    };

    if !redisshujukuzhuti::lianjie(&redispeizhi).await {
        if shujukupeizhi.redis.bixuchushihua {
            tuichu("Redis 连接失败");
        }
        eprintln!("Redis 连接失败，已跳过");
    } else {
        println!("Redis 连接成功: {}", shujukupeizhi.redis.fuwuqimingcheng);
    }
    
    if !gongju::wangluogongju::shifangduankou(zongpeizhi.houduanyunxingduankou) {
        eprintln!("警告: 释放端口 {} 失败，但继续运行", zongpeizhi.houduanyunxingduankou);
    }
    
    println!("启动服务器: http://127.0.0.1:{}", zongpeizhi.houduanyunxingduankou);
    
    HttpServer::new(|| {
        let kuayu = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(kuayu)
            .configure(jiekouxt::jiekouxtzhuti::peizhi)
    })
        .bind(("127.0.0.1", zongpeizhi.houduanyunxingduankou))?
        .run()
        .await
}
