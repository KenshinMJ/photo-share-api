# photo-share-api

[RustでGraphQLサーバの実装を試してみる](https://zenn.dev/mkazutaka/articles/9b9228da5a741a)を参考に Rust で GraphQL Server を作ってみる。

## run 
`cargo run` でGraphQL Playgroundのサーバが立ち上がります。クエリを投げると正しく動作することがわかります。

```graphql
// query
{
    totalPhotos
}
// response
{
  "data": {
    "totalPhotos": 42
  }
}
```
