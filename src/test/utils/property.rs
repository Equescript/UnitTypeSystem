
/// **单例**
///
/// 这个 trait 没有实际约束力，只是作为一个标签
///
/// 实现 Singleton 的类型需要保证在整个程序中访问的所有该类型实例都是同一个
///
/// 推荐使用 static mut 全局变量实现
pub trait Singleton: Clone + Copy + 'static {

    // fn instantiate() -> Result<T, E>;
    fn instance() -> &'static Self;
    fn instance_mut() -> &'static mut Self;
}

pub trait MultipleSingleton: Singleton {
    type Select;
    fn instance(select: Self::Select) -> &'static Self;
    fn instance_mut(select: Self::Select) -> &'static mut Self;
}

