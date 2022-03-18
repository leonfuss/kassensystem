require('lspconfig').sqls.setup{
    settings = {
        sqls = {
            connections = {
                driver = 'postgresql',
                dataSourceName= 'host=localhost port=5432 user=postgres password=password dbname=kassensystem sslmode=disable'
            }
        }
    }
}

print("Project neovim config loaded")
