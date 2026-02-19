pub struct Zhuangtai {
    pub ma: u16,
    pub xiaoxi: &'static str,
}

#[allow(non_upper_case_globals)]
pub mod cuowu {
    use super::Zhuangtai;
    
    pub const qingqiugeshibuzhengque: Zhuangtai = Zhuangtai { ma: 400, xiaoxi: "请求参数格式错误" };
    pub const canshugeshibuzhengque: Zhuangtai = Zhuangtai { ma: 400, xiaoxi: "参数格式错误" };
    pub const bucaozuoleixing: Zhuangtai = Zhuangtai { ma: 400, xiaoxi: "不支持的操作类型" };
    pub const putongkezhixianshibai: Zhuangtai = Zhuangtai { ma: 400, xiaoxi: "普通用户仅允许提交日报和查询自己的日报" };
    
    pub const queshouquanlingpai: Zhuangtai = Zhuangtai { ma: 401, xiaoxi: "缺少授权令牌" };
    pub const lingpaiwuxiao: Zhuangtai = Zhuangtai { ma: 401, xiaoxi: "令牌无效或已过期" };
    
    pub const quanxianbuzu: Zhuangtai = Zhuangtai { ma: 403, xiaoxi: "权限不足，无法访问该接口" };
    pub const quanxianbuzucaozuo: Zhuangtai = Zhuangtai { ma: 403, xiaoxi: "权限不足，无法访问该操作" };
    
    pub const biaoqianleixingbucunzai: Zhuangtai = Zhuangtai { ma: 404, xiaoxi: "标签类型不存在" };
    pub const biaoqianbucunzai: Zhuangtai = Zhuangtai { ma: 404, xiaoxi: "标签不存在" };
    pub const ribaobucunzai: Zhuangtai = Zhuangtai { ma: 404, xiaoxi: "日报不存在" };
    pub const guanlianbucunzai: Zhuangtai = Zhuangtai { ma: 404, xiaoxi: "关联不存在" };
    
    pub const chuangjianshi: Zhuangtai = Zhuangtai { ma: 500, xiaoxi: "创建失败" };
    pub const chaxunshibai: Zhuangtai = Zhuangtai { ma: 500, xiaoxi: "查询失败" };
    pub const tongjishibai: Zhuangtai = Zhuangtai { ma: 500, xiaoxi: "统计失败" };
    pub const guanlianshibai: Zhuangtai = Zhuangtai { ma: 500, xiaoxi: "关联失败" };
    pub const shanchushibai: Zhuangtai = Zhuangtai { ma: 500, xiaoxi: "删除失败" };
    pub const piliangguanlianshibai: Zhuangtai = Zhuangtai { ma: 500, xiaoxi: "批量关联失败" };
    pub const qingqiuchulishibai: Zhuangtai = Zhuangtai { ma: 500, xiaoxi: "请求处理失败" };
}
