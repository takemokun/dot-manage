# powerlevel10kのテーマがない場合は先にインストール
# git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k

mv ~/.oh-my-zsh/custom/themes/powerlevel10k/internal ~/.oh-my-zsh/custom/themes/powerlevel10k/internal_bk
ln -s ~/.config/dotfiles/oh-my-zsh-themes/powerlevel10k/internal ~/.oh-my-zsh/custom/themes/powerlevel10k/internal
