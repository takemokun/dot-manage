local opts = { noremap = true, silent = true }
local term_opts = { silent = true }

local keymap = vim.api.nvim_set_keymap

--Remap space as leader key
keymap("", "<Space>", "<Nop>", opts)
vim.g.mapleader = " "
vim.g.maplocalleader = " "

-- 行の端に行く
keymap("n", "<Space>h", "^", opts)
keymap("n", "<Space>l", "$", opts)
keymap("x", "<Space>h", "^", opts)
keymap("x", "<Space>l", "$", opts)

-- 行末までのヤンクにする
keymap("n", "Y", "y$", opts)

-- インサートモードとビジュアルモードでのCtrl-g
keymap("i", "<C-g>", "<ESC>", opts)
keymap("v", "<C-g>", "<ESC>", opts)

-- Split window
keymap("n", "ss", ":split<Return><C-w>w", opts)
keymap("n", "sv", ":vsplit<Return><C-w>w", opts)

-- 元のままだとtmuxとかぶる
keymap("n", "<C-h>", "<C-w>h", opts)
keymap("n", "<C-j>", "<C-w>j", opts)
keymap("n", "<C-k>", "<C-w>k", opts)
keymap("n", "<C-l>", "<C-w>l", opts)

-- インサートモードで bash 風キーマップ
-- ンサートモードで bash 風キーマップ
keymap('i', '<C-a>', '<C-o>^', opts)
keymap('i', '<C-e>', '<C-o>$<Right>', opts)
keymap('i', '<C-b>', '<Left>', opts)
keymap('i', '<C-f>', '<Right>', opts)
keymap('i', '<C-n>', '<Down>', opts)
keymap('i', '<C-p>', '<Up>', opts)
keymap('i', '<C-h>', '<BS>', opts)
keymap('i', '<C-d>', '<Del>', opts)
keymap('i', '<C-k>', '<C-o>D<Right>', opts)
keymap('i', '<C-u>', '<C-o>d^', opts)
keymap('i', '<C-w>', '<C-o>db', opts)

local builtin = require('telescope.builtin')
vim.keymap.set('n', '<leader>ff', builtin.find_files, {})
vim.keymap.set('n', '<leader>fg', builtin.live_grep, {})
vim.keymap.set('n', '<leader>fb', builtin.buffers, {})
vim.keymap.set('n', '<leader>fh', builtin.help_tags, {})

vim.keymap.set('n', '<leader>b', vim.cmd.NvimTreeToggle)
