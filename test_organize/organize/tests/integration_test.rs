use organize;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, organize::add_two(2));
}
//例 11-13：集成测试


//*集成测试中的子模块*/
