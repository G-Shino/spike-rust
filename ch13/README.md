## 関数型の言語機能　イテレータとクロージャ
- クロージャ: 変数に保存できる関数に似た文法要素 環境をキャプチャできる匿名関数
- 呼び出し結果を保持する構造体を作れる memo化みたいな
- rustではfn定義の関数は閉じられているが、クロージャー定義だと環境で定義された変数を巻き込める
- クロージャーで変数の所有権を奪いたいならmoveを用いる
- イテレータ: 一連の要素を処理する方法
- イテレータは使用されることで真価を発揮する
- Iteratorトレイト　nextメソッドでsomeを返し、終了時にnoneを返す
- .iter():　参照 .inter_mut: 可変参照　.into_iter:　所有権を渡す
- 関数型プログラミングスタイルは可変な状態を最小化することを好む

