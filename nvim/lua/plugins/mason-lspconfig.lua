return {
  'williamboman/mason-lspconfig.nvim',
  dependencies = {
    'neovim/nvim-lspconfig',
    'hrsh7th/cmp-nvim-lsp',
    "williamboman/mason.nvim",
  },
  config = function()
    require("mason-lspconfig").setup()
    local lspconfig = require('lspconfig')

    require('mason-lspconfig').setup_handlers {
      function(server_name)
        lspconfig[server_name].setup {
          capabilities = require('cmp_nvim_lsp').default_capabilities(),
        }
      end,
    }
  end,
}
