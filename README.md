# dotfiles

## install

```
git clone --depth=1 https://github.com/takemokun/dotfiles.git ~/.config/dotfiles
```

## 使い方


```zsh

# enable
sh link.sh
sh linkPowerlevel10k.sh

# disable
sh unlink.sh
sh unlinkPowerlevel10k.sh
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
