return {
  "hrsh7th/nvim-cmp",
  dependencies = {
      "hrsh7th/cmp-buffer", "hrsh7th/cmp-nvim-lsp",
      'quangnguyen30192/cmp-nvim-ultisnips', 'hrsh7th/cmp-nvim-lua',
      'octaltree/cmp-look', 'hrsh7th/cmp-path', 'hrsh7th/cmp-calc',
      'f3fora/cmp-spell', 'hrsh7th/cmp-emoji', 'hrsh7th/cmp-cmdline',
      'onsails/lspkind-nvim',
  },
  config = function()
      require("supports.cmp")
  end,
}
