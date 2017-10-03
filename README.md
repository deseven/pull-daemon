## pull-daemon
Простенький сервис, слушающий http-запросы на указанном порту и выполняющий указанный скрипт.

### compiling
`rustc pd.rs && strip pd`

### running
`pd $port $command`