#![allow(unused_imports, dead_code)]

mod gongju;
mod peizhixt;
mod shujuku;

use peizhixt::peizhi_nr::peizhi_zongpeizhi::Zongpeizhi;
use peizhixt::peizhi_nr::peizhi_shujuku::Shujuku;
use shujuku::qrshujuku::qrshujukuzhuti::{self, Qrpeizhi};
use qdrant_client::qdrant::Distance;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !peizhixt::peizhixitongzhuti::chushihua() {
        eprintln!("配置系统初始化失败");
        std::process::exit(1);
    }
    
    let zongpeizhi = peizhixt::peizhixitongzhuti::duqupeizhi::<Zongpeizhi>(
        Zongpeizhi::wenjianming()
    ).expect("读取总配置失败");
    
    let shujukupeizhi = peizhixt::peizhixitongzhuti::duqupeizhi::<Shujuku>(
        Shujuku::wenjianming()
    ).expect("读取数据库配置失败");
    
    if shujukupeizhi.xiangliangku.qiyong {
        let qrpeizhi = Qrpeizhi {
            zhiji: shujukupeizhi.xiangliangku.zhiji,
            duankou: shujukupeizhi.xiangliangku.grpc_duankou,
            miyao: shujukupeizhi.xiangliangku.miyao,
            jheqianzhui: shujukupeizhi.xiangliangku.jheqianzhui,
        };
        
        if qrshujukuzhuti::lianjie(&qrpeizhi, "moren", 1536, Distance::Cosine).await {
            println!("Qdrant 向量数据库连接成功");
        } else {
            eprintln!("Qdrant 向量数据库连接失败");
        }
    }
    
    if !gongju::wangluogongju::shifangduankou(zongpeizhi.houduanyunxingduankou) {
        eprintln!("释放端口 {} 失败", zongpeizhi.houduanyunxingduankou);
        std::process::exit(1);
    }
    
    println!("启动服务器: http://127.0.0.1:{}", zongpeizhi.houduanyunxingduankou);
    
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", zongpeizhi.houduanyunxingduankou))?
        .run()
        .await
}
