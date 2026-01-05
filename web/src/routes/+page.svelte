<script lang="ts">
    import Board from "$lib/components/board/board.svelte";
    import User from "$lib/components/frontpage/user.svelte";
    import {
        cookie,
        engines,
        friend_invites,
        friends,
        game_invites,
        games,
        logged_in,
        rating,
        username,
    } from "$lib/state";
    import { socket } from "$lib/ws";
    import { onMount } from "svelte";

    let username_input: string | null;
    let password_input: string;

    function submit(e: SubmitEvent) {
        e.preventDefault();

        $socket.send("login:psw", {
            username: username_input,
            password: password_input,
        });
    }

    onMount(() => {
        let name_from_storage = localStorage.getItem("Username");

        if (name_from_storage && username_input == null) {
            username_input = name_from_storage;
        }
    });

    function react_to_invite(type: "game" | "friend", hash: number) {
        let vari = type == "game" ? $game_invites : $friend_invites;

        let invite = vari.find((invite) => {
            return invite.hash == hash;
        });
        if (invite && invite.from != $username) {
            $socket.send("game:accept", {
                hash: invite.hash,
            });
        } else if (invite && invite.from == $username) {
            $socket.send(type + ":cancel_invite", {
                hash: invite.hash,
            });
        }
    }

    $socket.on("game:games", (result, content) => {
        $games = content;
        console.log(content);
    });

    $socket.on("login:psw", (result, content) => {
        if (result) {
            $cookie = content.cookie;
            $logged_in = true;
            $username = username_input || "go ask the developer why he is sooo stupid";
            localStorage.setItem("Username", $username);
        }
    });
</script>

<div class="m-5">
    {#if $logged_in}
        <h2>{$username} ({$rating})</h2>

        <div class="flex">
            <div>
                <h3>Venner:</h3>
                {#each $friends as friend}
                    <User name={friend.name} rating={friend.rating} />
                    <br />
                {/each}

                <br />

                <h3>Tilgængelige skak bots:</h3>
                {#each $engines as engine}
                    <User name={engine.name} rating={engine.rating} />
                    <br />
                {/each}
            </div>

            <br />
            <div>
                <h3>Invitationer:</h3>
                {#each $game_invites as invite}
                    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <p on:click={() => react_to_invite("game", invite.hash)}>
                        🎲 {invite.from} ⇒ {invite.to}
                    </p>
                {/each}

                {#each $friend_invites as invite}
                    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <p on:click={() => react_to_invite("friend", invite.hash)}>
                        👬 {invite.from} ⇒ {invite.to}
                    </p>
                {/each}
            </div>
        </div>

        <!-- <Board \> -->
        {#each $games as game}
            <Board {game} />
        {/each}
    {:else}
        <form on:submit={submit} class="w-[300px] p-5 shadow-md rounded-xl" action="/api/login">
            <div class="grid grid-cols-1 grid-rows-3 gap-2">
                <input
                    bind:value={username_input}
                    id="username"
                    class="border rounded-md p-2 mt-2 w-full"
                    placeholder="brugernavn"
                    type="text"
                />
                <input
                    bind:value={password_input}
                    id="password"
                    class="border rounded-md p-2 mt-2 w-full"
                    placeholder="kodeord"
                    type="password"
                />
                <button class="bg-text-50">Log ind</button>
            </div>
        </form>
    {/if}
</div>
