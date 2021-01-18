# Substrate Node Template

```sh
1.编写存证块的单元测试代码，包括：
    创建存证
    测试存证
    转移存证
    
2.创建存证时，为存证内容的哈希值 Vec<u8>
  设置长度上限，超过限制时返回错误
  并编写测试用例    
```


``
make build
``

``
make run
``

``
cargo test -p pallet-poe
``