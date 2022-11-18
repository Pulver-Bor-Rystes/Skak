import * as mongo_db from 'mongodb';
export const collections: { users?: mongo_db.Collection } = {}


export async function connect_to_db() {
    // For at tilfredsstille ts
    const database_settings = JSON.parse(JSON.stringify(process.env.database))

    const client: mongo_db.MongoClient = new mongo_db.MongoClient(database_settings.uri);
    await client.connect();

    const db: mongo_db.Db = client.db(database_settings.name);

    database_settings.collections.forEach(function (name: string) {
        db.createCollection(name, function (err, res) {
            if (!err)
                console.log(`Collection: '${name}' created`)
        })

        /* @ts-ignore */
        collections[name] = db.collection(name)
    })
}