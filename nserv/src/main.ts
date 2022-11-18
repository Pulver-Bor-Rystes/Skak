import { Database } from './api/database';
import { Users } from './api/users';
import { Server } from './server';

Server.on('ping', (req, data) => {
  console.log('\nreceived topic:', req.topic);
  console.log(data);
  req.reply_sid('pong');
});

Database.init();
Server.init();

console.clear()
console.log("Det kører sgu")

  
Users.login("rasmus", "kodeord")
  .then(val => {
    console.log("yeah:", val)
  })
  .catch(err => {
    console.log("err:", err)
  })
