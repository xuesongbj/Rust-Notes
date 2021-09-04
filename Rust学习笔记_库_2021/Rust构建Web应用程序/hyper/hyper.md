# hyper

hyper软件包可以解析HTTP消息，并且具有优雅的设计、侧重于强类型的API。它被设计为原始HTTP请求类型安全的抽象，而不像常见的HTTP程序库那样：将所有内容描述为字符串。

hyper将客户端和服务端功能拆分为单独的模块。客户端允许你使用可配置的请求主体、首部及其他底层配置来构建和发送HTTP请求。服务端允许你打开侦听套接字，并将请求处理程序附加给它。但是它不包括任何请求路由处理程序实现—— 这些留给Web框架处理。它只在构建更高级Web框架的基础软件包。它在底层使用相同的tokio和futures异步抽象。

hyper的核心 `Service` 特征：

```rust
pub trait Service {
    type ReqBody: Payload;
    type ResBody: Payload;
    type Error: Into<Box<dyn StdError + Send + Sync>>;
    type Future: Future<Item = Response<Self::ResBody>, Error = self::Error>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> self::Future;
}
```

* `Service` 特征表示一种类型，它处理从任何客户端发送的HTTP请求，并返回Response响应，这是一个future。
* 该特征需要实现 `call`，它接收一个泛型 `Body`上参数化的 `Request`，并结合解析为 `Response` 的 `future`，该 `Response`通过关联类型 `ResBody`进行参数化。我们不需要手工实现此特征，因为 hyper包含一系列可以为用户实现Service特征的工厂方法。

&nbsp;

## hyper服务器端API —— 构建一个短网址服务

详见具体项目
