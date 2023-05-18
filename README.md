# dotfiles

## install

```
git clone --depth=1 https://github.com/takemokun/dotfiles.git ~/.config/dotfiles
```

## 使い方

※ cargo（rust）が使える環境が必要です

### .envの設定
```zsh
cp .env.sample .env
vi .env

# HOME_PATHを自分のユーザールートに設定する
HOME_PATH=/Users/username
```

### dotfilesの設定

```zsh
# dotfilesからの操作
cargo run

# commands
copy:       dotfilesからコピー
sync:       dotfilesにコピー
clean:      バックアップファイルの削除 
clean_me: dotfiles内のバックアップファイル削除
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

※このREADME自体だいぶ適当なので基本適宜でよろろ
