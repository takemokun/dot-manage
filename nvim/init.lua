require("core.options")
require("core.keymaps")
require("core.colorscheme")
require("plugins-setup")
require("plugins.lsp.lspconfig")
require("plugins.nvim-tree")
require("plugins.lualine")
require("plugins.cmp")

-- vim.api.nvim_create_autocmd({'BufEnter','BufAdd','BufNew','BufNewFile','BufWinEnter'}, {
--   group = vim.api.nvim_create_augroup('TS_FOLD_WORKAROUND', {}),
--   callback = function()
--     vim.opt.foldmethod     = 'expr'
--     vim.opt.foldexpr       = 'nvim_treesitter#foldexpr()'
--   end
-- })
