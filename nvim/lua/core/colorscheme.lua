local status, _ = pcall(vim.cmd, "colorscheme tokyonight-night")
if not status then
    print("Color scheme not found")
    return
end

local highlight_status, _ = pcall(vim.cmd, "highlight StatusLine guibg=#003388")

if not highlight_status then
    print("Cant setting highlight color")
    return
end
