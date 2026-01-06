-- Duck language support for Neovim
local M = {}

M.config = {
  lsp = {
    enabled = true,
    cmd = { "/home/jace/.local/bin/duck-lsp" },
  },
}

function M.setup(opts)
  opts = opts or {}
  M.config = vim.tbl_deep_extend("force", M.config, opts)

  -- Set up filetype detection
  vim.filetype.add({
    extension = {
      duck = "duck",
    },
  })

  -- Set up LSP if enabled
  if M.config.lsp.enabled then
    M.setup_lsp()
  end
end

function M.setup_lsp()
  local lspconfig_ok, lspconfig = pcall(require, "lspconfig")
  local configs_ok, configs = pcall(require, "lspconfig.configs")

  if not lspconfig_ok or not configs_ok then
    -- Fallback: use vim.lsp.start directly
    vim.api.nvim_create_autocmd("FileType", {
      pattern = "duck",
      callback = function()
        vim.lsp.start({
          name = "duck-lsp",
          cmd = M.config.lsp.cmd,
          root_dir = vim.fn.getcwd(),
          settings = {},
        })
      end,
    })
    return
  end

  -- Register duck-lsp with lspconfig if not already registered
  if not configs.duck_lsp then
    configs.duck_lsp = {
      default_config = {
        cmd = M.config.lsp.cmd,
        filetypes = { "duck" },
        root_dir = function(fname)
          return lspconfig.util.find_git_ancestor(fname) or vim.fn.getcwd()
        end,
        settings = {},
      },
    }
  end

  -- Set up the LSP
  lspconfig.duck_lsp.setup({
    cmd = M.config.lsp.cmd,
  })
end

return M
