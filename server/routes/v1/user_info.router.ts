import express, { NextFunction, Request, Response } from "express"

export const user_info = express.Router()
user_info.use(express.json())


// api/me

user_info.use('/name', (req: Request, res: Response) => {
    if (req?.user) {
        res.send(JSON.stringify(req.user.firstname))
    }
    else {
        res
            .status(404)
            .send('not_logged_in')
    }
})