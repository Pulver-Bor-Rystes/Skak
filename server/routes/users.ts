import express, { Request, Response } from "express"
import { ObjectId } from "mongodb"
import { collections } from "../setup/database"
import User from "../models/user"


// Global Config
export const users_router = express.Router()
users_router.use(express.json())


users_router.get('/', (req, res) => {
    res.status(200).send('yeah')
})