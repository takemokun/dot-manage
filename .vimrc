" Note: Skip initialization for vim-tiny or vim-small.
if 0 | endif

if &compatible
  set nocompatible               " Be iMproved
endif

" Required:
set runtimepath+=~/.vim/bundle/neobundle.vim/


" My Bundles here:
" Refer to |:NeoBundle-examples|.
" Note: You don't set neobundle setting in .gvimrc!

" unite prefix key
nnoremap [unite] <Nop>
nmap <Space>f [unite]
" mappings
nnoremap [unite]b :<C-u>Unite<Space>buffer<CR>
nnoremap [unite]f :<C-u>Unite<Space>file<CR>
nnoremap [unite]m :<C-u>Unite<Space>file_mru<CR>
nnoremap [unite]a :<C-u>Unite buffer file file_mru bookmark<CR>
nnoremap [unite]r :<C-u>Unite<Space>register<CR>
nnoremap [unite]R :<C-u>UniteResume<CR>
let g:unite_enable_start_insert = 1
let g:unite_enable_ignore_case = 1
let g:unite_enable_smart_case = 1

" ESCキーを2回押すと終了する  
 au FileType unite nnoremap <silent> <buffer> <ESC><ESC>:q<CR>
 au FileType unite inoremap <silent> <buffer> <ESC><ESC> <ESC>:q<CR>

" Required:
filetype plugin indent on

" If there are uninstalled bundles found on startup,
" this will conveniently prompt you to install them.

inoremap { {}<LEFT>
inoremap ( ()<LEFT>
inoremap [ []<LEFT>
inoremap ' ''<LEFT>
inoremap " ""<LEFT>
noremap <Space>h  ^
noremap <Space>l  $
noremap <C-g> <esc>
noremap! <C-g> <esc>

set autoindent
set expandtab
set tabstop=2
set shiftwidth=2
set cursorline
set number
set wildmenu
set history=5000
set laststatus=2
set cmdheight=2
set scrolloff=4
set list
set listchars=tab:>.,trail:_,eol:↲,extends:>,precedes:<,nbsp:%
set wildmenu
set wildmode=list:longest,full

set background=dark

" grep検索の実行後にQuickFix Listを表示する
autocmd QuickFixCmdPost *grep* cwindow
let g:user_emmet_leader_key = '<C-E>'

if &compatible
  set nocompatible
endif
" Add the dein installation directory into runtimepath

filetype plugin indent on
syntax enable

autocmd FileType vue syntax sync fromstart
autocmd BufNewFile,BufRead *.{html,htm,vue*} set filetype=html

" indentLine
set list lcs=tab:\|\
let g:indentLine_char = 'c'
let g:indentLine_color_term = 239
