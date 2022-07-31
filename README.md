# photo-share-api

[RustでGraphQLサーバの実装を試してみる](https://zenn.dev/mkazutaka/articles/9b9228da5a741a)を参考に Rust で GraphQL Server を作ってみる。

## run 
`cargo run` でGraphQL Playgroundのサーバが立ち上がります。クエリを投げると正しく動作することがわかります。


### 画像合計枚数取得
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

### 写真投稿

```graphql
// query
mutation newPhoto($name: String!, $description: String) {
  postPhoto(name: $name, description: $description)
}
// query variables
{
  "name": "sample photo A",
  "description": "A sample photo for our dataset"
}
// response
{
  "data": {
    "postPhoto": true
  }
}
```
