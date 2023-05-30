# dotfiles

## install

```
git clone --depth=1 https://github.com/takemokun/dotfiles.git ~/.config/dotfiles
```

## 使い方

※ cargo（rust）が使える環境が必要です

[cargo install](https://doc.rust-lang.org/cargo/getting-started/installation.html)

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
> cargo build --release

# ビルドされたコマンドの実行（ヘルプが表示されます）
> dotfiles
```

※ビルドしなくても、`cargo run`で`dotfiles`コマンド同様に使えます。

### コマンドの実行

```zsh
# dotfilesから設定したパスへコピーする場合
> dotfiles copy

# 設定したパスからdotfilesへ`.zshrc`のみコピーする場合
> dotfiles sync zshrc

# copyやsyncを実行するとバックアップファイルが作成されます（`.zshrc.20230505`てきな）
# バックアップファイルを削除する場合は
#   - `dotfiles clean`（設定したパスのバックアップファイル削除）
#   - `dotfiles clean-me`（dotfiles内のバックアップファイル削除）
# を実行してください
```


## 必要なこと
### tmux入れる
```zsh
brew install tmux
```

### neovim入れる
```zsh
brew install neovim
```

### powerlevel10k入れる
```zsh
git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k
```

### 備考
tmuxifierとかも他にも入れないといけないものあるかもなので適宜よろろ

※このREADME自体だいぶ適当なので基本適宜でよろろろ
