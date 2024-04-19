require('lualine').setup({
  lualine_c = {
    {
      'diagnostics',
      sources = { 'nvim_diagnostic', 'nvim_lsp' },
      sections = { 'error', 'warn', 'info', 'hint' },
      symbols = { error = ' ', warn = ' ', info = ' ', hint = '' },
    },
  },
  -- options = {
  --   theme = 'onenord'
  -- },
})

vim.api.nvim_set_option('showmode', false)
