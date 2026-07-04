# Garnet

Garnet は、小さな Ruby 風 DSL でアプリケーション UI を記述するための実験的な UI runtime です。

現在のプロトタイプは `.rb` ファイルを読み込み、UI ツリーを構築し、それを `egui` / `eframe` で描画します。まだ Ruby インタプリタではありません。Garnet は「Ruby のような形で UI 構造を書き、それを Rust runtime が描画する」とどんな感触になるかを探っています。

## Garnet とは何か

Garnet は、いくつかの小さな層でできた Rust アプリケーションです。

- `garnet-source` はソースファイルやローカルアセットを読み込みます。
- `garnet-runtime` は Garnet の Ruby 風 DSL を UI ツリーへ変換します。
- `garnet-ui` はその UI ツリーを定義します。
- `garnet-renderer` は UI ツリーを `egui` で描画します。
- `garnet-app` はそれらをデスクトップアプリとしてつなぎます。

中心にある考え方はシンプルです。

```ruby
page do
    column do
        image "examples/assets/logo.png"

        text "Garnet"

        row do
            button "Open"
            button "Exit"
        end
    end
end
```

これは `Page`、`Column`、`Image`、`Text`、`Row`、`Button` の UI ツリーになり、renderer がそれを描画します。

## なぜ HTML ではなく Ruby なのか

HTML はドキュメントや Web にとても優れています。一方で Garnet は、少し違う形を探っています。つまり、UI を小さく表現力のあるプログラムとして扱う形です。

Ruby は Garnet にいくつかの良い性質を与えます。

- **読みやすい構造**: `page do ... end`、`column do ... end`、`row do ... end` によって、山括弧なしでネストを明示できます。
- **言語として育てられる UI 面**: DSL は、テンプレート言語になる前に、ロジック、合成、再利用可能なコンポーネントへ伸ばせます。
- **RWP への道筋**: 将来的に source が file、HTTP、RWP、cache のどれであっても、同じ読み込み境界から扱える設計を目指しています。
- **ドキュメント前提から離れること**: Garnet は HTML を再実装しようとしていません。アプリケーション UI の primitive から始め、どう描画するかは runtime に任せます。

Ruby が HTML より常に優れている、という主張ではありません。Garnet にとって Ruby は、構造を持つ UI をあたたかく、プログラム可能な構文で書くための入口です。

## 今できること

現在の Garnet は、次の DSL を解析して描画できます。

- `page do ... end`
- `column do ... end`
- `row do ... end`
- `text "..."`
- `button "..."`
- `image "..."`

Row と Column は自由にネストできます。Button は今のところ表示のみで、クリックしても何も起きません。Image はローカルの PNG/JPEG ファイルを読み込みます。

runtime は、未対応構文、余分な `end`、閉じ忘れ、block 外の component などもエラーとして報告します。

## 例

```ruby
page do
    column do
        text "Hello Garnet"

        row do
            button "Open"
            button "Save"
            button "Exit"
        end

        text "Version 0.1"
    end
end
```

期待される形:

```text
Hello Garnet

[ Open ] [ Save ] [ Exit ]

Version 0.1
```

## 実行方法

```sh
cargo run -p garnet-app
```

現在、アプリは `examples/sample.rb` を読み込みます。

## 現在の制限

Garnet はまだ次の機能に対応していません。

- クリックイベント
- 状態管理
- コンポーネント定義
- Ruby の実行
- HTTP 画像
- SVG / GIF / WebP
- 画像サイズ指定や fit
- スタイリング
- 単純な row / column 描画を超える layout constraint

今はまず、Ruby 風 source、Rust runtime、UI tree、native rendering という中核の流れを確かめる段階です。
