#![allow(unused_imports, dead_code)]

mod gongju;
mod peizhixt;
mod shujuku;

use peizhixt::peizhi_nr::peizhi_zongpeizhi::Zongpeizhi;
use peizhixt::peizhi_nr::peizhi_shujuku::Shujuku;
use peizhixt::peizhixitongzhuti;
use shujuku::qrshujuku::qrshujukuzhuti::{self, Qrpeizhi};
use shujuku::psqlshujuku::psqlshujukuzhuti::{self, Psqlpeizhi, Shujubiaodinyi, Biaozhucexinxi};
use shujuku::psqlshujuku::shujubiao_nr::shujubiao_shujubiaojilubiao::Shujubiaojilubiao;
use qdrant_client::qdrant::Distance;
use actix_web::{App, HttpServer};

fn tuichu(xinxi: &str) -> ! {
    eprintln!("{}", xinxi);
    std::process::exit(1);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !peizhixitongzhuti::chushihua() {
        tuichu("配置系统初始化失败");
    }
    
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

        let biaolie = &[
            Biaozhucexinxi {
                biaoming: Shujubiaojilubiao::biaoming(),
                biaonicheng: Shujubiaojilubiao::biaonicheng(),
                biaojieshao: Shujubiaojilubiao::biaojieshao(),
                ziduanlie: Shujubiaojilubiao::ziduanlie(),
            },
        ];

        if !psqlshujukuzhuti::lianjie(&psqlpeizhi, biaolie).await {
            tuichu("PostgreSQL 数据库连接失败");
        }
        println!("PostgreSQL 数据库连接成功");
    }
    
    if !gongju::wangluogongju::shifangduankou(zongpeizhi.houduanyunxingduankou) {
        tuichu(&format!("释放端口 {} 失败", zongpeizhi.houduanyunxingduankou));
    }
    
    println!("启动服务器: http://127.0.0.1:{}", zongpeizhi.houduanyunxingduankou);
    
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", zongpeizhi.houduanyunxingduankou))?
        .run()
        .await
}
