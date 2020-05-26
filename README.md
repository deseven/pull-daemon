## pull-daemon
Простенький сервис, слушающий http-запрос `GET/POST /pull` на указанном порту и выполняющий набор скриптов из каталога `update-repos.d`. Сервер написан на Rust, вспомогательный скрипт на bash.

### compiling
`rustc pd.rs && strip pd`

### running
`./pd port command`

### installing pre-compiled binary (linux x86_64) with default configuration
`bash <(curl -s https://home-nadym.ru/pd/i)`
