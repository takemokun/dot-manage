local cmp = require 'cmp'
local map = cmp.mapping

cmp.setup {
  mapping = map.preset.insert {
    ['<C-d>'] = map.scroll_docs(-4),
    ['<C-f>'] = map.scroll_docs(4),
    ['<C-Space>'] = map.complete(),
    ['<C-e>'] = map.abort(),
    ['<CR>'] = map.confirm { select = false },
    ["<Tab>"] = vim.schedule_wrap(function(fallback)
      if cmp.visible() then
        cmp.select_next_item({ behavior = cmp.SelectBehavior.Select })
      else
        fallback()
      end
    end),
  },
  sources = cmp.config.sources {
    -- Copilot Source
    { name = "copilot",  group_index = 2 },
    -- Other Sources
    { name = "nvim_lsp", group_index = 2 },
    { name = "path",     group_index = 2 },
    { name = "luasnip",  group_index = 2 },
    { name = 'buffer' },
  },
  snippet = {
    expand = function(args)
      require('luasnip').lsp_expand(args.body) -- For `luasnip` users.
    end,
  },
  -- mapping = {
  --    -- ["<Tab>"] = vim.schedule_wrap(function(fallback)
  --    --     if cmp.visible() then
  --    --         cmp.select_next_item({ behavior = cmp.SelectBehavior.Select })
  --    --     else
  --    --         fallback()
  --    --     end
  --    -- end),
  -- },
  -- sources = {
  --     -- Copilot Source
  --     { name = "copilot", group_index = 2 },
  --     -- Other Sources
  --     { name = "nvim_lsp", group_index = 2 },
  --     { name = "path", group_index = 2 },
  --     { name = "luasnip", group_index = 2 },
  -- },

  -- formatting = {
  --     format = lspkind.cmp_format({
  --         mode = "symbol",
  --         max_width = 50,
  --         symbol_map = { Copilot = "" }
  --     })
  -- },

  formatting = {
    format = require('lspkind').cmp_format {
      preset = 'codicons',
      mode = "symbol",
      max_width = 50,
      symbol_map = { Copilot = "" }
    },
  },
}

-- Set configuration for specific filetype.
-- cmp.setup.filetype('gitcommit', {
--     sources = cmp.config.sources({
--         { name = 'cmp_git' }, -- You can specify the `cmp_git` source if you were installed it.
--     }, {
--         { name = 'buffer' },
--     })
-- })

-- Use buffer source for `/` and `?` (if you enabled `native_menu`, this won't work anymore).
cmp.setup.cmdline({ '/', '?' }, {
  mapping = cmp.mapping.preset.cmdline(),
  sources = {
    { name = 'buffer' },
  }
})

-- Use cmdline & path source for ':' (if you enabled `native_menu`, this won't work anymore).
cmp.setup.cmdline(':', {
  mapping = cmp.mapping.preset.cmdline(),
  sources = cmp.config.sources({
    { name = 'path' }
  }, {
    { name = 'cmdline' }
  })
})

local capabilities = vim.lsp.protocol.make_client_capabilities()
capabilities = require('cmp_nvim_lsp').default_capabilities()

local lspconfig = require('lspconfig')
require('mason-lspconfig').setup_handlers {
  function(server_name)
    lspconfig[server_name].setup {
      capabilities = capabilities,
    }
  end,
}
