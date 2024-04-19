-- Use LspAttach autocommand to only map the following keys
-- after the language server attaches to the current buffer
vim.api.nvim_create_autocmd('LspAttach', {
  group = vim.api.nvim_create_augroup('UserLspConfig', {}),
  callback = function(ev)
    -- Enable completion triggered by <c-x><c-o>
    vim.bo[ev.buf].omnifunc = 'v:lua.vim.lsp.omnifunc'

    -- Buffer local mappings.
    -- See `:help vim.lsp.*` for documentation on any of the below functions
    local opts = { buffer = ev.buf }

    vim.keymap.set('n', 'gD', vim.lsp.buf.declaration, opts)
    vim.keymap.set('n', '<space>wa', vim.lsp.buf.add_workspace_folder, opts)
    vim.keymap.set('n', '<space>wr', vim.lsp.buf.remove_workspace_folder, opts)
    vim.keymap.set('n', '<space>wl', function()
      print(vim.inspect(vim.lsp.buf.list_workspace_folders()))
    end, opts)
    vim.keymap.set('n', '<space>D', vim.lsp.buf.type_definition, opts)
    vim.keymap.set('n', '<space>rn', '<Cmd>Lspsaga rename<CR>', opts)
    vim.keymap.set('n', '<space>ca', vim.lsp.buf.code_action, opts)

    -- FIXME:
    vim.keymap.set('n', '<space>f', function()
      vim.lsp.buf.format { async = true }
    end, opts)
    vim.keymap.set('n', '<A-l>', function()
      vim.lsp.buf.format { async = true }
    end, opts)

    vim.keymap.set('n', '<C-j>', '<Cmd>Lspsaga diagnostic_jump_next<cr>', opts)
    vim.keymap.set('n', 'K', '<Cmd>Lspsaga hover_doc<cr>', opts)
    vim.keymap.set('n', 'gd', '<Cmd>Lspsaga finder def<CR>', opts)
    vim.keymap.set('n', 'gi', '<Cmd>Lspsaga finder imp<CR>', opts)
    vim.keymap.set('n', 'gr', '<Cmd>Lspsaga finder ref<CR>', opts)
    vim.keymap.set('n', 'ga', '<Cmd>Lspsaga finder tyd+ref+imp+def<CR>', opts)
    vim.keymap.set('i', '<C-k>', '<Cmd>Lspsaga signature_help<CR>', opts)
    vim.keymap.set('n', 'gp', '<Cmd>Lspsaga preview_definition<CR>', opts)
    vim.keymap.set('n', '<A-b>', '<Cmd>Lspsaga goto_definition<CR>', opts)
    vim.keymap.set('n', '<A-CR>', '<Cmd>Lspsaga code_action<CR>', opts)
    vim.keymap.set('n', '<space>e', '<Cmd>Lspsaga show_buf_diagnostics<CR>', opts)
    vim.keymap.set('n', '[d', '<Cmd>Lspsaga diagnostic_jump_prev<CR>', opts)
    vim.keymap.set('n', ']d', '<Cmd>Lspsaga diagnostic_jump_next<CR>', opts)
    vim.keymap.set('n', '<space>w', '<Cmd>Lspsaga show_workspace_diagnostics<CR>', opts)
    -- cmd使えるようにしたい
    -- vim.keymap.set('n', '<M-b>', '<Cmd>Lspsaga goto_definition<cr>', opts)
    -- vim.keymap.set('n', '<D-b>', '<Cmd>Lspsaga goto_definition<CR>', opts)
  end,
})
