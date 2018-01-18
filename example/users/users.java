import database.Connection;
import database.Connection.ConnectingError;
import database.Connection.CallingError;
import database.Users.Entity;

class Main {
    public static void main(String[] args) {
        try {
            Connection con = new Connection("mysql:127.0.0.1");
            try {
                Entity[] users = con.GetUsersByAge(19, 25);
                for (Entity user : users) {
                    System.out.printf("%s %s%n", user.info.first_name, user.info.last_name);
                }
            } catch (CallingError err) {
                System.out.println("Ошибка выполнения запроса");
                err.printStackTrace();
            } finally {
                con.close();
            }
        } catch (ConnectingError err) {
            System.out.println("Ошибка подключения к базе данных");
            err.printStackTrace();
        }
    }
}
