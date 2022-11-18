import { Database as DB } from 'sqlite3';
import util from "util"

export class Database {
  static db: DB;
  static each;
  static run;
  static all;


  static init() {
    let db = new DB('mydb.sqlite');
    this.db = db;
    this.each = util.promisify(this.db.each.bind(this.db))
    this.run = util.promisify(this.db.run.bind(this.db))
    this.all = util.promisify(this.db.all.bind(this.db))

    let resp = db.serialize(() => {
      db.run(`
        CREATE TABLE IF NOT EXISTS user (
          user_id INTEGER PRIMARY KEY,
          username TEXT NOT NULL,
          UNIQUE(user_id)
        )
      `)
      
      db.run(`
        CREATE TABLE IF NOT EXISTS cookie (
          cookie_id INTEGER PRIMARY KEY,
          hashed_string TEXT NOT NULL,
          salt TEXT,
          keylen INTEGER,
          iterations INTEGER,
          digest TEXT,
          user_id INTEGER,
          last_used INTEGER,
          FOREIGN KEY (user_id)
            REFERENCES user (user_id)
              ON DELETE CASCADE
        )
      `)
      // last_used er egentlig bare Date.now()
      // hvis last_used = -1, er cookie faktisk et password og må ikke slettes
    })
  }
  
  static async rall(sql_query, params?) {
    let result;
    
    await this.all(sql_query, params)
      .catch(err => { throw err })
      .then(rows => result = rows)
    
    return result
  }

  // static run(sql_query: string, params?: any) {
  //     if (params) {
  //         return this.db.run(sql_query, params)
  //     }
  //     return this.db.run(sql_query)
  // }

  // static query(sql_query: string) {
  //     return this.db.query(sql_query)
  // }
}
