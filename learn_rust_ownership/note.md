# 所有权规则（重要）

* 1.Rust中的每一个值都有一个对应的称为所有者（owner）的变量
* 2.在一个时间点上，一个值只能有一个所有者
* 3.当所有者退出作用域，对应的值就会被丢弃                   ！！！重要3条

# 数据在栈和堆上的分配

* 1.编译确定大小且大小不变的数据存放在栈上 Integer/Reference
* 2.编译时不确定大小（或运行期大小可变）的数据存放在堆上，String/Vector
* 3.访问性能栈比堆快

# 字符串字面量string literal和字符串String类型

* 1.字符串字面量存放在可执行程序中，编译时值就确定，不可变
* 2.String运行期动态可变

# 拷贝和移入

* 1.将一个整数变量（或者其他数量类型）赋值给另外一个变量，拷贝copy行为
* 2.将一个字符串变量（或者其他引用类型）赋值给另外一个变量，移入Move行为

# 引用规则小结

* 1.对于特定的数据，在某个特定的时间，只能有一个可变的引用或者多个不可变的引用。
* 2.引用必须是有效的。
* 3.借用数据，不获得所有权

```rs
fn main () {
        println!("hello rust");
}
```