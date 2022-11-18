import { Database as DB } from 'sqlite3';

class Database {
    static db: DB;

    static init() {
        let db = new DB("mydb.sqlite")
        this.db = db;
    }

    static run(sql_query: string, params?: any) {
        if (params) {
            return this.db.exec(sql_query, params)
        }
        return this.db.exec(sql_query)
    }

    static query(sql_query: string) {
        return this.db.query(sql_query)
    }
}