<?php
    /*
     * Это что-то типа заголовочного файла,
     * чтобы описать примерную структуру сгенерированного файла
     * и чтобы не ломалась подсветка у файла users.php
     */
namespace Database;

class Connection {
    public function __construct(string $connection_string) {}
    public function close() {}
    public function GetUsersByAge($min, $max): array {
        if ($min > $max)
            throw new Connection\CallingError("");
        if ($min == $max)
            throw new Connection\ConnectingError("");
        return [];
    }
}

namespace Database\Connection;

class ConnectingError extends \Exception {}
class CallingError extends \Exception {}
