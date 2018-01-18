<?php
require_once "database.php";

use Database\Connection;
use Database\Connection\ConnectingError;
use Database\Connection\CallingError;

try {
    $con = new Connection("mysql:127.0.0.1");
    try {
        $users = $con->GetUsersByAge(19, 25);
        foreach ($users as $user) {
            printf("%s %s\n", $user->info->first_name, $user->info->last_name);
        }
    } catch (CallingError $err) {
        printf("Ошибка выполнения запроса: %s\n", $err);
    } finally {
        $con->close();
    }
} catch (ConnectingError $err) {
    printf("Ошибка подключения к базе данных: %s\n", $err);
}
