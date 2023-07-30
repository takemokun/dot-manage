# dotfiles

dotfileの管理をちょっとだけ楽しく楽にするツール🛠️

![sample.gif](sample.gif)

## install

```
git clone --depth=1 https://github.com/takemokun/dotfiles.git ~/.config/dotfiles
```

## 使い方

### 注意事項
- takemokun用（ワタクシ用）の設定ファイルが入ってるのでsrc配下以外のファイルはよしなに変えてください。
  - [mapping.json](https://github.com/takemokun/dotfiles/blob/main/mapping.json)を変えれば、設定ファイルの情報を変更できます。
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


## 必要なこと
※このdotfilesをそのまま使う場合に必要なことなので、コマンドだけ使う場合は不要なことに変わります...

### zsh関連
```zsh
# oh-my-zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# zsh-autosuggestions
git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions
```

- [oh-my-zsh](https://github.com/ohmyzsh/ohmyzsh)
- [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions)

### tmux入れる
```zsh
brew install tmux

# tpm
https://github.com/tmux-plugins/tpm
tmux source ~/.tmux.conf
```

- [tpm](https://github.com/tmux-plugins/tpm)

### neovim入れる
```zsh
brew install neovim

# packer
git clone --depth 1 https://github.com/wbthomason/packer.nvim\
 ~/.local/share/nvim/site/pack/packer/start/packer.nvim

# nvim開いて
:PackerSync
# ※ nvim開いたときにエラーが出てたら
#   1. エラー箇所一旦コメントアウト
#   2. PackerSync
#   3. コメントアウト戻す
#   でいけるはず
```
- [pakcer](https://github.com/wbthomason/packer.nvim)

### fzf入れる
```
git clone --depth 1 https://github.com/junegunn/fzf.git ~/.fzf
~/.fzf/install
```

- [fzf](https://github.com/junegunn/fzf)

### powerlevel10k入れる
```zsh
git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k
```

### rust製コマンド入れる
```zsh
brew install fd rg bat exa procs
```

たぶん他にもなんかしないといけないかも🤔

### 備考
基本的に適当なのでよしなにどうぞ。

⚠️現状`dotfiles apply`などでディレクトリのdiffを表示しようとするとエラーになります⚠️
