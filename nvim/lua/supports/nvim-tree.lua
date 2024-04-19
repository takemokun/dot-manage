vim.opt.termguicolors = true

require('nvim-tree').setup {
  sort_by = "case_sensitive",
  git = {
    enable = true,
  },
  filters = {
    custom = {
      "^.git$",
      ".DS_Store",
    },
  },
  renderer = {
    group_empty = true,
    highlight_git = true,
    indent_markers = {
      enable = true
    },
    icons = {
      show = {
        git = true,
      },
    },
  },
  diagnostics = {
    enable = true,
    show_on_dirs = true,
    show_on_open_dirs = true,
    icons = {
      hint = "",
      info = "",
      warning = "",
      error = "",
    },
  },
}

vim.api.nvim_create_user_command('Ex', function() vim.cmd.NvimTreeToggle() end, {})
