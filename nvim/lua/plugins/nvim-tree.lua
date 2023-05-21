local status, nvim_tree = pcall(require, "nvim-tree")

if not status then
    return
end

vim.g.loaded_netrw = 1
vim.g.loaded_netrwPlugin = 1

-- set termguicolors to enable highlight groups
vim.opt.termguicolors = true

nvim_tree.setup({
    sort_by = "case_sensitive",
    renderer = {
        group_empty = true,
    },
    filters = {
        dotfiles = true,
    },
})

vim.api.nvim_create_user_command('Ex', function() vim.cmd.NvimTreeToggle() end, {})
