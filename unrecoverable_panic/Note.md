# Rust 错误处理

##### Rust 将错误组合成两个主要类别： 

##### **可恢复错误**（ *recoverable* ）和**不可恢复错误**（ *unrecoverable* ）。

##### 可恢复错误通常代表向用户报告错误和重试操作是合理的情况，比如未找到文件。不可恢复错误通常是 bug 的同义词，比如尝试访问超过数组结尾的位置。

---