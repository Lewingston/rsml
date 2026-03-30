
function runInConsole(command)

    vim.cmd('wa')
    vim.cmd('split')
    vim.cmd('term ' .. command)
    vim.cmd('setlocal nospell')
    vim.api.nvim_input('G')
end


vim.keymap.set('n', '<C-b>', function()

    runInConsole('cargo build')
end)


vim.keymap.set('n', '<F5>', function()

    runInConsole('cargo run')
end)


vim.keymap.set('n', '<C-c>', function()

    runInConsole('cargo clippy -- ' ..
                 '-Wclippy::all ' ..
                 '-Wclippy::pedantic ' ..
                 '-Wclippy::unwrap_used ' ..
                 '-Aclippy::module_inception')
end)
