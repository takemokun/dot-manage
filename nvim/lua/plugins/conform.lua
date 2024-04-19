return {
	"stevearc/conform.nvim",
	opts = {},
	enabled = false,
	config = function()
		require("conform").setup({
			format_on_save = {
				-- These options will be passed to conform.format()
				timeout_ms = 500,
				lsp_fallback = true,
			},
			formatters_by_ft = {
				lua = { "stylua" },
				go = { "goimports", "gofmt" },
				-- Use a sub-list to run only the first available formatter
				javascript = { { "biome", "prettierd", "prettier" } },
				typescript = { { "biome", "prettierd", "prettier" } },
				typescriptreact = { { "biome", "prettierd", "prettier" } },
			},
		})

		require("supports.conform")
	end,
	init = function()
		-- If you want the formatexpr, here is the place to set it
		vim.o.formatexpr = "v:lua.require'conform'.formatexpr()"
	end,
}

-- return {
-- 	"stevearc/conform.nvim",
-- 	event = { "BufWritePre" },
-- 	cmd = { "ConformInfo" },
-- 	keys = {
-- 		{
-- 			-- Customize or remove this keymap to your liking
-- 			"<leader>f",
-- 			function()
-- 				require("conform").format({ async = true, lsp_fallback = true })
-- 			end,
-- 			mode = "",
-- 			desc = "Format buffer",
-- 		},
-- 	},
-- 	-- Everything in opts will be passed to setup()
-- 	opts = {
-- 		-- Define your formatters
-- 		formatters_by_ft = {
-- 			lua = { "stylua" },
-- 			go = { "goimports", "gofmt" },
-- 			-- Use a sub-list to run only the first available formatter
-- 			javascript = { { "biome", "prettierd", "prettier" } },
-- 			typescript = { { "biome", "prettierd", "prettier" } },
-- 			typescriptreact = { { "biome", "prettierd", "prettier" } },
-- 		},
-- 		-- Set up format-on-save
-- 		format_on_save = { timeout_ms = 500, lsp_fallback = true },
-- 		-- Customize formatters
-- 		formatters = {
-- 			shfmt = {
-- 				prepend_args = { "-i", "2" },
-- 			},
-- 		},
-- 	},
-- 	init = function()
-- 		-- If you want the formatexpr, here is the place to set it
-- 		vim.o.formatexpr = "v:lua.require'conform'.formatexpr()"
-- 	end,
-- }
