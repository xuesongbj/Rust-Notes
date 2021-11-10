# Pin 和 Unpin

> Pin主要是为了解决 `async/await` 自动生成 Future的问题。问题就是自引用，移动自引用结构体会造成指针失效。

