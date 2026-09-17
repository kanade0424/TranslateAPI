# TranslateAPI
**tred**を用いた翻訳APIです。

## API
`/translate`
HTTPメソッド:`POST`
リクエストbody
```json
{
    "sauce":"ja",
    "target":"en",
    "text": "こんにちは"
}
```
レスポンス例
```json
{
    "text":"Hello"
}
```
`/status`
翻訳APIのステータスを取得します。Getメソッドです。