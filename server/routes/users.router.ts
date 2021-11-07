import express, { NextFunction, Request, Response } from "express"
import { check_against_pattern } from "../../shared/functions/checkers"
import User, { Friendship } from "../models/user.model"
import { collections } from "../setup/database"

export const users_router = express.Router()
users_router.use(express.json())


/*
    * /api/users/search

    Tager i mod to parametre,
    1. query, som fungerer som søgeordet som skal matches i mod brugere.
    2. mode, bruges som et filter. ( all, friends, not_friends, blocked )

*/

type Search_For = 'username' | 'firstname'
type Mode = 'all' | 'friends' | 'not_friends' | 'blocked'

/*
    En API til at søge efter brugere.



*/
users_router.use('/search/:search_for/:mode/:query', async (req: Request, res: Response) => {
    const search_for: string = req?.params?.search_for
    const mode = req?.params?.mode as Mode
    const query: string = req?.params?.query


    if (req?.user) {
  
        // * Tjek at inputs er korrekte * //

        const sf_res = check_against_pattern(search_for, 'search_for');
        const m_res = check_against_pattern(mode, 'mode');
        const q_res = check_against_pattern(query, 'username');

        if (!sf_res || !m_res || !q_res) {
            res
                .status(404)
                .send('bad_parameters')

            return;
        }


        // * Søg i database med inputs * //
  
        let col_query: any = {}
        col_query[search_for] = { $regex: query, $options: 'i' }


        collections.users?.find(col_query)
            .limit(20) // Begrænser søgningen til 20 resultater
            /* @ts-ignore */
            .toArray( (err: Error, users: User[]) => {
                if (err) {
                    res
                        .status(404)
                        .send('something_wrong_happened')
                }

                // Den data som brugeren må se
                const Filter: string[] = ['username', 'firstname', 'lastname', 'rating']
                
                users.forEach( (user: User) => {
                    Object.keys( user ).forEach( key => {
                        if (!Filter.includes(key))
                            /* @ts-ignore */
                            delete user[key];
                    })
                } )

                
                // * Server oplysninger til bruger * //
                
                res.send(users)

            })
    }
    else {
        res
            .status(404)
            .send('not_logged_in')
    }
})


users_router.use('/send_friend_request/:username', async (req: Request, res: Response) => {
    const username: string = req?.params?.username

    if (req?.user) {
  
        // * Tjek at inputs er korrekte * //

        const username_res = check_against_pattern(username, 'username');

        if (!username_res) {
            res
                .status(404)
                .send('bad_parameters')

            return;
        }


        // * Søg i database med inputs * //

        const user = (await collections.users?.findOne({ username })) as User

        if (user && user?.username != req?.user.username) {

            let block_mode: Boolean = false;

            // TODO: Fiks tilføj ven :)
            
            try {
                if (!check_if_it_includes_friend(req.user, user.friend_requests)) throw 'already_sent_request'
                if (!check_if_it_includes_friend(req.user, user.friends)) throw 'already_friends'
                block_mode = check_if_it_includes_friend(req.user, user.blocked)
            }
            catch (err) {
                res.send(err)
            }

            // * Tilføj

            res.send('yay')

        } 
        else {
            res.send('nope')
        }
    }
    else {
        res
            .status(404)
            .send('not_logged_in')
    }
})


function check_if_it_includes_friend(user: User, friends: Friendship[]) {
    friends.forEach( (friend: Friendship) => {
        if (user.username == friend.username)
            return false;
    })

    return true;
}