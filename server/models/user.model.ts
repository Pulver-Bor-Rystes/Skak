import type { ObjectId } from "mongodb";
import type { Sex, Server_Cookie, Rating } from "../../shared/types"


export default class User {
    constructor(
        public username: string,
        public firstname: string,
        public lastname: string,
        public sex: Sex,
        
        public avatar_seed: string,
        public rating: Rating,

        public cookies: Server_Cookie[],
        public password: string,

        public id?: ObjectId,
    ) {}
}