<script lang="ts">
    import { socket } from "$lib/ws";
    import { onMount } from "svelte";
    import {
        active_players,
        engines,
        friend_invites,
        friends,
        game_invites,
        rating,
    } from "$lib/state";

    onMount(() => {
        $socket.query("friends", (result, content) => {
            if (!result) return;

            $friends = [];
            content.list.forEach((friend: string) => {
                $friends.push({ name: friend, rating: 1 });
            });

            $friends = $friends;
        });

        $socket.query("my_rating", (result, content) => {
            if (!result) return;
            $rating = content.rating;
        });

        $socket.query("engines", (result, content) => {
            if (!result) return;
            if (content == "IsBeingProcessed") return;

            engines.set(content);
        });
    });

    $socket.on("game:invite_overview", (result, content) => {
        if (!result) return;

        console.log(content.game_invite_overview);
        game_invites.set(content.game_invite_overview);
        $game_invites = $game_invites;
    });

    $socket.on("friend:invite_overview", (result, content) => {
        if (!result) return;

        friend_invites.set(content.friend_invite_overview);
        $friend_invites = $friend_invites;
    });

    $socket.on("invite", (result, content) => {});

    $socket.on("query:active_users", (result, content) => {
        if (!result) return;

        active_players.set(content.List);
    });
</script>
