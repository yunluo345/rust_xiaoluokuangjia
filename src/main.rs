#![allow(unused_imports, dead_code)]

mod gongju;
mod peizhixt;

use peizhixt::peizhi_nr::peizhi_zongpeizhi::Zongpeizhi;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !peizhixt::peizhixitongzhuti::chushihua() {
        eprintln!("配置系统初始化失败");
        std::process::exit(1);
    }
    
    let peizhi = peizhixt::peizhixitongzhuti::duqupeizhi::<Zongpeizhi>(
        Zongpeizhi::wenjianming()
    ).expect("读取配置失败");
    
    if !gongju::wangluogongju::shifangduankou(peizhi.houduanyunxingduankou) {
        eprintln!("释放端口 {} 失败", peizhi.houduanyunxingduankou);
        std::process::exit(1);
    }
    
    println!("启动服务器: http://127.0.0.1:{}", peizhi.houduanyunxingduankou);
    
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", peizhi.houduanyunxingduankou))?
        .run()
        .await
}
