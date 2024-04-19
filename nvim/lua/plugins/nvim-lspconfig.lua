return {
  'neovim/nvim-lspconfig',
  config = function() 
      -- require("mason").setup({})
      -- require("mason-lspconfig").setup()
      require("supports.lsp.lspconfig")
  end,
}
