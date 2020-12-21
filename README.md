# proto_desc_printer
研究`protobuf`的`descriptor_set`结构，方便后续进行扩展工具开发。

本项目作为一个`protoc`插件运行。

`protoc`会先将`proto`文件解析，然后传递`descriptor`结构给插件继续进行处理。

这个`descriptor`结构定义在[descriptor.proto](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/descriptor.proto)。

这里使用[protobuf](https://crates.io/crates/protobuf)中已经实现好的解析函数，来解析其中的信息，并以可读的方式打印出其中的内容。

### 命令
```
protoc --plugin=protoc-gen-printer=./target/debug/proto_desc_printer --printer_out=. example.proto > output 2>&1
```

### 注意事项
`protoc`插件会通过`stdout`输出生成的代码，因此本项目中的打印都是通过`stderr`输出的。