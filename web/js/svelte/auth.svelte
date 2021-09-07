<script type="ts">
    import { post_request } from './public'
    import type { Sex } from "../../../shared/types"

    let username = 'musmus9';
    let _username;
    let firstname = 'Rasmus';
    let lastname = 'Foldberg';

    let avatar_seed = 'sb213s';
    let sex: Sex = 'Male';

    let password = '123';
    let _password;



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

        let resp = await post_request('/auth/signup', details)

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
        let resp = await post_request('/auth/login', {
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
        <input bind:value={username} type="username" placeholder="Brugernavn" />
        <input bind:value={firstname} type="firstname" placeholder="Fornavn" />
        <input bind:value={lastname} type="lastname" placeholder="Efternavn" />

        <!-- Avatar -->
        <input bind:value={avatar_seed} type="text" placeholder="avater_seed" />
        <input bind:value={sex} type="text" placeholder="Køn (Male, Female, Other)" />

        <!-- Kodeord -->
        <input bind:value={password} type="password" placeholder="Kodeord" />

        <button on:click={on_signup}> Opret </button>
    </div>

    <div class="signup">
        <h1>Log ind</h1>

        <!-- Kodeord -->
        <input bind:value={_username} type="username" placeholder="Brugernavn" />
        <input bind:value={_password} type="password" placeholder="Kodeord" />

        <button on:click={on_login}> Log ind </button>
    </div>
</div>


<style type="scss">
    .auth {
        background-color: rgb(31, 31, 49);
        width: 100vw;
        height: 100vh;

        .signup {
            display: inline-grid;
            background: antiquewhite;

            margin: 1rem;
            border-radius: 5px;

            h1 {
                color: #000;
            }

            * {
                margin: 0.3rem;
                padding: 1rem;
            }
        }
    }
</style>
