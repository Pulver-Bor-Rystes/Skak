<script type="ts">
    import { nget, npost } from '../stores/networking';
    import type { Sex } from "../../../shared/types"

    let username = 'musmus9';
    let _username: any;
    let firstname = 'Rasmus';
    let lastname = 'Foldberg';

    let avatar_seed = 'sb213s';
    let sex: Sex = 'Male';

    let password = '123';
    let _password: any;



    interface Signup_details {
        username: string
        firstname: string
        lastname: string
        
        avatar_seed: string
        sex: Sex
        
        password: string
    }

    const on_signup = async () => {
        const details: Signup_details = {
            username,
            firstname,
            lastname,
            avatar_seed,
            sex,
            password
        }

        let resp = await npost('/api/v1/auth/signup', details)

        console.log(resp)


        if (resp.status) {
            // Yay!
        }
        else {
            // Nay :(
            console.log(resp?.errors)
        }
    };

    const on_login = async () => {
        let resp = await npost('/api/v1/auth/login', {
            'username': _username,
            'password': _password,
        })

        if (resp.status)
            console.log('Logged in!')
        else
            console.log(resp?.errors)
    }

    
</script>



<div class="auth">
    <div class="signup">
        <h1>Opret konto</h1>

        <!-- Standard -->
        <input class="bg-gray-800 m-1 p-1" bind:value={username} type="username" placeholder="Brugernavn" />
        <input class="bg-gray-800 m-1 p-1" bind:value={firstname} type="firstname" placeholder="Fornavn" />
        <input class="bg-gray-800 m-1 p-1" bind:value={lastname} type="lastname" placeholder="Efternavn" />

        <!-- Avatar -->
        <input class="bg-gray-800 m-1 p-1" bind:value={avatar_seed} type="text" placeholder="avater_seed" />
        <input class="bg-gray-800 m-1 p-1" bind:value={sex} type="text" placeholder="Køn (Male, Female, Other)" />

        <!-- Kodeord -->
        <input class="bg-gray-800 m-1 p-1" bind:value={password} type="password" placeholder="Kodeord" />

        <button on:click={on_signup}> Opret </button>
    </div>

    <div class="signup">
        <h1>Log ind</h1>

        <!-- Kodeord -->
        <input class="bg-gray-800 m-1 p-1" bind:value={_username} type="username" placeholder="Brugernavn" />
        <input class="bg-gray-800 m-1 p-1" bind:value={_password} type="password" placeholder="Kodeord" />

        <button on:click={on_login}> Log ind </button>
    </div>
</div>
