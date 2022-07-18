import type { ObjectId } from "mongodb";
import type { Sex, Server_Cookie, Rating, Settings } from "../../shared/types"



export type Friendship = {
    username: string
    date: number
}

export default class User {
    constructor(
        public username: string,
        public firstname: string,
        public lastname: string,
        public sex: Sex,
        
        public avatar_seed: string,
        public rating: Rating,
        public settings: Settings,

        public friends: Friendship[],
        public friend_requests: Friendship[],
        public blocked: Friendship[],

        public cookies: Server_Cookie[],
        public password: string,

        public id?: ObjectId,
    ) {}
}