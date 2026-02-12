use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;

fn quebaofumulu(lujing: &Path) -> bool {
    lujing.parent().map_or(true, |fuji| fs::create_dir_all(fuji).is_ok())
}

/// 检查文件是否存在
#[allow(dead_code)]
pub fn wenjiancunzai(lujing: &str) -> bool {
    Path::new(lujing).is_file()
}

/// 检查文件是否存在，不存在则创建（含沿途文件夹）
#[allow(dead_code)]
pub fn querenbingchuangjian(lujing: &str) -> bool {
    wenjiancunzai(lujing) || xieruwenjian(lujing, "")
}

/// 以文本形式读取任意文件，失败返回 None
#[allow(dead_code)]
pub fn duquwenjian(lujing: &str) -> Option<String> {
    fs::read_to_string(lujing).ok()
}

/// 将文本内容写入任意文件，沿途文件夹不存在则自动创建
#[allow(dead_code)]
pub fn xieruwenjian(lujing: &str, neirong: &str) -> bool {
    quebaofumulu(Path::new(lujing)) && fs::write(lujing, neirong).is_ok()
}

/// 追加内容到文件末尾，文件不存在则自动创建（含沿途文件夹）
#[allow(dead_code)]
pub fn zhuijianeirong(lujing: &str, neirong: &str) -> bool {
    quebaofumulu(Path::new(lujing))
        && fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(lujing)
            .and_then(|mut wenjian| wenjian.write_all(neirong.as_bytes()))
            .is_ok()
}

/// 删除文件
#[allow(dead_code)]
pub fn shanchuwenjian(lujing: &str) -> bool {
    fs::remove_file(lujing).is_ok()
}

/// 复制文件到目标路径，目标父目录不存在则自动创建
#[allow(dead_code)]
pub fn fuzhiwenjian(yuan: &str, mubiao: &str) -> bool {
    quebaofumulu(Path::new(mubiao)) && fs::copy(yuan, mubiao).is_ok()
}

/// 移动文件到目标路径，优先原子重命名，跨设备时回退为复制后删除
#[allow(dead_code)]
pub fn yidongwenjian(yuan: &str, mubiao: &str) -> bool {
    quebaofumulu(Path::new(mubiao))
        && (fs::rename(yuan, mubiao).is_ok()
            || fuzhiwenjian(yuan, mubiao) && shanchuwenjian(yuan))
}

/// 列出目录下所有文件路径
#[allow(dead_code)]
pub fn liebiaowenjian(mulu: &str) -> Option<Vec<PathBuf>> {
    fs::read_dir(mulu).ok().map(|duqu| {
        duqu.filter_map(|xiang| xiang.ok().map(|x| x.path()))
            .filter(|lujing| lujing.is_file())
            .collect()
    })
}
