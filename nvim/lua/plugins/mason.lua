return {
  "williamboman/mason.nvim",
  dependencies = {
    'neovim/nvim-lspconfig',
    'hrsh7th/cmp-nvim-lsp',
  },
  config = function()
    require("mason").setup({})
  end,
}
