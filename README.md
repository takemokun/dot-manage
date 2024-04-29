# dot-manage

dotfileの管理をちょっとだけ楽しく楽にするツール🛠️

![sample.gif](sample.gif)

## install

```
git clone --depth=1 https://github.com/takemokun/dot-manage.git
```

## 使い方

### 注意事項
- サンプルの設定ファイルが入ってるのでsrc配下以外のファイルはよしなに変えてください。
  - [mapping.json](https://github.com/takemokun/dot-manage/blob/main/mapping.json)を変えれば、設定ファイルの情報を変更できます。
- cargo（rust）が使える環境が必要です
  - [cargo install](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### .envの設定
```zsh
> cp .env.sample .env
> vi .env

# HOME_PATHに自分のユーザーのルートパスを設定する
# `echo $HOME`のやつ
HOME_PATH=/Users/username
```

### dotfilesコマンドのビルド
    
```zsh
> cargo install --path .

# ビルドされたコマンドの実行（ヘルプが表示されます）
> dotfiles
```

※ビルドしなくても、`cargo run`で`dotfiles`コマンド同様に使えます。

### コマンドの実行

```zsh
# dotfilesから設定したパスへコピーする場合
> dotfiles apply

# 設定したパスからdotfilesへ`.zshrc`のみコピーする場合
> dotfiles sync zshrc

# applyやsyncを実行するとバックアップファイルが作成されます（`.zshrc.20230505`てきな）
# バックアップファイルを削除する場合は
#   - `dotfiles clean`（設定したパスのバックアップファイル削除）
#   - `dotfiles clean-me`（dotfiles内のバックアップファイル削除）
# を実行してください
```

