# Ruby関連
alias be="bundle exec"

alias goland="/usr/local/bin/goland"
alias rubymine="/usr/local/bin/mine"

# Docker関連
alias dc="docker-compose"
alias da="docker attach"
alias dls="docker container ls"
alias dlog="docker logs -f"

# tmux
alias tmls="tmux ls"
alias tma="tmux attach -t"
alias tms="tmux new -s"
alias tmks="tmux kill-session -t"

alias vi='nvim'

# Rust製コマンドへの置き換え
# ls
alias ls="exa"
alias lsa="exa -a"
alias ll="exa -lh"
alias la="exa -lah"
# grep
alias grep="rg"
# find
alias find="fd"
# cat
alias cat="bat"
# ps
alias ps="procs"
alias pst="procs --sortd CPU | head -n 6"
alias psc="procs --sortd CPU"
