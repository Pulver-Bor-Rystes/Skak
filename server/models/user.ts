import { ObjectId } from "mongodb";


interface Rating {
    'blitz': number
    'normal': number
}

interface Cookie {
    'str': string,
    'expiration_date': Date
}

type Sex = 'Male' | 'Female' | 'Other'


export default class Game {
    constructor(
        public username: string,
        public firstname: string,
        public lastname: string,
        public sex: Sex,
        
        public avatar_seed: string,
        public rating: Rating,

        public cookies: Cookie[],
        public password: string,

        public id?: ObjectId,
    ) {}
}