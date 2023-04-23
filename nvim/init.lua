require("options")
require("plugins")
require("keymaps")
require("my-nvim-tree")
require("my-cmp")
require("lsp")

vim.cmd[[colorscheme tokyonight-night]]

-- vim.api.nvim_create_autocmd({'BufEnter','BufAdd','BufNew','BufNewFile','BufWinEnter'}, {
--   group = vim.api.nvim_create_augroup('TS_FOLD_WORKAROUND', {}),
--   callback = function()
--     vim.opt.foldmethod     = 'expr'
--     vim.opt.foldexpr       = 'nvim_treesitter#foldexpr()'
--   end
-- })
 
