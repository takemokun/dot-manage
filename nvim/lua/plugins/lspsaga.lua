return {
  'nvimdev/lspsaga.nvim',
  config = function()
    require('lspsaga').setup({
      border_style = "single",
    })
  end,
  dependencies = {
    'nvim-treesitter/nvim-treesitter',
    'nvim-tree/nvim-web-devicons',
  },
}
